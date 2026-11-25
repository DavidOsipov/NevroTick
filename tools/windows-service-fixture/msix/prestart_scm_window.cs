// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace NevroTick
{
    // Point samples from the original two SCM objects under the retained actor.
    // The deployment owner establishes role eligibility and continuous quiescence.
    public sealed class PrestartScmWindow : IDisposable
    {
        private const int MaxTimeoutMilliseconds = 120000;
        private const int ErrorServiceDoesNotExist = 1060;
        private const uint ServiceDisabled = 4;
        private const uint ServiceStopped = 1;

        private readonly object gate = new object();
        private readonly IApi api;
        private readonly Func<long> elapsedMilliseconds;
        private readonly Action verifyActor;
        private readonly NativeScope scope;
        private readonly int timeoutMilliseconds;
        private IntPtr manager;
        private IntPtr engine;
        private IntPtr broker;
        private long lastElapsed;
        private bool retired;
        private Exception failure;
        private SamplePair initial;
        private bool identityAttempted;
        private bool observationAttempted;
        private DeploymentIdentity identity;
        private PrestartObserverChild observation;

        private sealed class DeploymentIdentity
        {
            internal PrestartObserverChild Child;
            internal readonly PrestartObserverChild.ArchiveLease Archive;
            internal readonly string ArchiveSha256;
            internal readonly string SignerSha256;

            internal DeploymentIdentity(PrestartObserverChild.ArchiveLease archive, string archiveSha256,
                string signerSha256)
            {
                Archive = archive;
                ArchiveSha256 = archiveSha256;
                SignerSha256 = signerSha256;
            }
        }

        private sealed class NativeScope
        {
            internal readonly PrestartActor Actor;
            internal readonly string DeploymentSid;
            internal readonly Stopwatch Clock;

            internal NativeScope(PrestartActor actor, string deploymentSid, Stopwatch clock)
            {
                Actor = actor;
                DeploymentSid = deploymentSid;
                Clock = clock;
            }
        }

        private PrestartScmWindow(IApi api, Func<long> elapsedMilliseconds, Action verifyActor,
            int timeoutMilliseconds, NativeScope scope)
        {
            this.api = api;
            this.elapsedMilliseconds = elapsedMilliseconds;
            this.verifyActor = verifyActor;
            this.scope = scope;
            this.timeoutMilliseconds = timeoutMilliseconds;
        }

        public sealed class Sample
        {
            public uint ConfigurationType { get; private set; }
            public uint StartType { get; private set; }
            public uint StatusType { get; private set; }
            public uint CurrentState { get; private set; }

            internal Sample(Configuration configuration, Status status)
            {
                ConfigurationType = configuration.ServiceType;
                StartType = configuration.StartType;
                StatusType = status.ServiceType;
                CurrentState = status.CurrentState;
            }
        }

        public sealed class SamplePair
        {
            public Sample Engine { get; private set; }
            public Sample Broker { get; private set; }

            internal SamplePair(Sample engine, Sample broker)
            {
                Engine = engine;
                Broker = broker;
            }
        }

        // The initial pair is historical. Only Check obtains a new point sample.
        public SamplePair Initial { get { return initial; } }

        public static PrestartScmWindow Open(PrestartActor actor, string expectedDeploymentSid,
            Stopwatch clock, int timeoutMilliseconds)
        {
            if (actor == null)
                throw new ArgumentNullException("actor");
            if (String.IsNullOrEmpty(expectedDeploymentSid))
                throw new ArgumentException("prestart-scm-window:deployment-subject", "expectedDeploymentSid");
            if (clock == null)
                throw new ArgumentNullException("clock");
            if (Environment.OSVersion.Platform != PlatformID.Win32NT || !Environment.Is64BitProcess)
                throw new PlatformNotSupportedException("prestart-scm-window:windows-x64");

            long start = Stopwatch.GetTimestamp();
            double initialElapsed = clock.Elapsed.TotalMilliseconds;
            return Open(NativeApi.Instance, delegate
            {
                if (!clock.IsRunning)
                    return -1;
                double elapsedFromOpen = (Stopwatch.GetTimestamp() - start)
                    * (1000.0 / Stopwatch.Frequency);
                return (long)Math.Max(clock.Elapsed.TotalMilliseconds, initialElapsed + elapsedFromOpen);
            }, delegate { actor.VerifyCurrent(clock, timeoutMilliseconds, expectedDeploymentSid); },
                timeoutMilliseconds, new NativeScope(actor, expectedDeploymentSid, clock));
        }

        internal static PrestartScmWindow Open(IApi api, Func<long> elapsedMilliseconds, Action verifyActor,
            int timeoutMilliseconds)
        {
            return Open(api, elapsedMilliseconds, verifyActor, timeoutMilliseconds, null);
        }

        private static PrestartScmWindow Open(IApi api, Func<long> elapsedMilliseconds, Action verifyActor,
            int timeoutMilliseconds, NativeScope scope)
        {
            if (api == null)
                throw new ArgumentNullException("api");
            if (elapsedMilliseconds == null)
                throw new ArgumentNullException("elapsedMilliseconds");
            if (verifyActor == null)
                throw new ArgumentNullException("verifyActor");
            if (timeoutMilliseconds < 1 || timeoutMilliseconds > MaxTimeoutMilliseconds)
                throw new ArgumentOutOfRangeException("timeoutMilliseconds");

            PrestartScmWindow owner = new PrestartScmWindow(api, elapsedMilliseconds, verifyActor, timeoutMilliseconds, scope);
            try
            {
                owner.manager = owner.Acquire("open-manager", api.Connect);
                owner.engine = owner.Acquire("open-engine", delegate { return api.Open(owner.manager, false); });
                owner.broker = owner.Acquire("open-broker", delegate { return api.Open(owner.manager, true); });
                owner.initial = owner.ReadPair();
                return owner;
            }
            catch (Exception error)
            {
                owner.failure = error;
                owner.Retire();
                throw;
            }
        }

        // The one deployment attempt retains its original verified archive through both children.
        public PrestartObserverChild StartVerifiedIdentity(string powershellPath, long expectedBytes,
            string expectedSha256, string inputDirectory, long entryBytes, string entrySha256,
            long moduleBytes, string moduleSha256, long helperBytes, string helperSha256, Guid runId,
            PrestartObserverChild.ArchiveLease archive, string expectedArchiveSha256, string expectedSignerSha256)
        {
            lock (gate)
            {
                RequireDeploymentScope("start-identity");
                if (identityAttempted)
                    throw Refusal("start-identity", "attempt-spent");
                identityAttempted = true;

                if (archive == null)
                    throw new ArgumentNullException("archive");
                scope.Actor.VerifyCurrent(scope.Clock, timeoutMilliseconds, scope.DeploymentSid);
                PrestartArchiveFacts facts = archive.Confirm(scope.Clock, timeoutMilliseconds, null, null);
                if (facts.PayloadSha256 == null)
                    throw Refusal("start-identity", "payload-binding-required");
                archive.VerifySignature(expectedArchiveSha256, expectedSignerSha256, scope.Actor,
                    scope.Clock, timeoutMilliseconds);
                RequireCurrentTime("start-identity");

                DeploymentIdentity attempt = new DeploymentIdentity(archive, expectedArchiveSha256,
                    expectedSignerSha256);
                PrestartObserverChild child = PrestartObserverChild.StartVerifiedWindowIdentity(powershellPath,
                    expectedBytes, expectedSha256, inputDirectory, entryBytes, entrySha256, moduleBytes,
                    moduleSha256, helperBytes, helperSha256, runId, archive, scope.DeploymentSid,
                    expectedArchiveSha256, expectedSignerSha256, scope.Actor, scope.Clock, timeoutMilliseconds,
                    WithOriginalSample, RequireCollectionTime);
                attempt.Child = child;
                identity = attempt;
                return child;
            }
        }

        // The coordinator validates the completed identity receipt before dispatching this selector.
        public PrestartObserverChild StartVerifiedObservation(string imagePath, long expectedBytes, string expectedSha256)
        {
            lock (gate)
            {
                RequireDeploymentScope("start-observation");
                if (identity == null || !identity.Child.SuccessfulWindowCapturePublished)
                    throw Refusal("start-observation", "identity-incomplete");
                if (observationAttempted)
                    throw Refusal("start-observation", "attempt-spent");
                observationAttempted = true;

                observation = PrestartObserverChild.StartVerifiedDeploymentObservation(imagePath, expectedBytes,
                    expectedSha256, identity.Archive, identity.ArchiveSha256, identity.SignerSha256, scope.DeploymentSid,
                    scope.Actor, scope.Clock, timeoutMilliseconds, WithOriginalSample, RequireCollectionTime);
                return observation;
            }
        }

        // Call after reading both receipts and before retiring this exact window.
        // The held handles supply point observations. The outer owner ensures quiescence.
        public void ConfirmDeployment()
        {
            const string operation = "confirm-deployment";
            lock (gate)
            {
                RequireDeploymentScope(operation);
                if (identity == null || !identity.Child.SuccessfulWindowCapturePublished ||
                    observation == null || !observation.SuccessfulWindowCapturePublished)
                    throw Refusal(operation, "captures-incomplete");

                try
                {
                    observation.ConfirmPublication();
                }
                catch (Exception error)
                {
                    if (failure == null)
                        failure = error;
                    throw;
                }
            }
        }

        private void RequireDeploymentScope(string operation)
        {
            if (scope == null)
                throw new InvalidOperationException("prestart-scm-window:deployment-scope");
            RequireCurrentTime(operation);
        }

        // Preserve SCM then archive lock order at the final publication check.
        internal void WithOriginalSample(Action afterSample)
        {
            lock (gate)
            {
                Check();
                if (afterSample != null)
                {
                    try
                    {
                        afterSample();
                        RequireCurrentTime("sample-complete");
                    }
                    catch (Exception error)
                    {
                        failure = error;
                        throw;
                    }
                }
            }
        }

        // The collector polls the retained budget and state without querying SCM.
        internal void RequireCollectionTime()
        {
            lock (gate)
                RequireCurrentTime("collection");
        }

        private void RequireCurrentTime(string operation)
        {
            if (failure != null)
                throw failure;
            if (retired)
                throw Refusal(operation, "retired");
            try
            {
                RequireTime(operation);
            }
            catch (Exception error)
            {
                failure = error;
                throw;
            }
        }

        // Opens no new handles. A failed observation permanently refuses this lease.
        public SamplePair Check()
        {
            lock (gate)
            {
                if (failure != null)
                    throw failure;
                if (retired)
                    throw Refusal("check", "retired");
                try
                {
                    return ReadPair();
                }
                catch (Exception error)
                {
                    failure = error;
                    throw;
                }
            }
        }

        // Call explicitly and inspect the return value before accepting the surrounding
        // operation. Each acquired handle has one close attempt. The returned first
        // failure carries subsequent close errors in Data["RetirementFailures"].
        // An unconfirmed native close requires retirement of the enclosing process.
        public Exception Retire()
        {
            lock (gate)
            {
                if (retired)
                    return failure;
                retired = true;
                List<Exception> laterFailures = new List<Exception>(3);
                CaptureRetirementDeadline(laterFailures);
                Close(ref broker, "close-broker", laterFailures);
                Close(ref engine, "close-engine", laterFailures);
                Close(ref manager, "close-manager", laterFailures);
                CaptureRetirementDeadline(laterFailures);
                if (laterFailures.Count != 0)
                    failure.Data["RetirementFailures"] = laterFailures.ToArray();
                GC.SuppressFinalize(this);
                return failure;
            }
        }

        // Dispose attempts retirement without masking an exception already in flight.
        // Only Retire exposes the result required for an explicit success decision.
        public void Dispose()
        {
            Retire();
        }

        // Abandonment can release native resources, with no successful retirement result.
        ~PrestartScmWindow()
        {
            if (Object.ReferenceEquals(api, NativeApi.Instance))
            {
                NativeApi.CloseAbandoned(broker);
                NativeApi.CloseAbandoned(engine);
                NativeApi.CloseAbandoned(manager);
            }
        }

        private void RequireTime(string operation)
        {
            long elapsed = elapsedMilliseconds();
            if (elapsed < 0 || elapsed < lastElapsed)
                throw Refusal(operation, "invalid-clock");
            lastElapsed = elapsed;
            if (elapsed >= timeoutMilliseconds)
                throw Refusal(operation, "deadline");
        }

        private IntPtr Acquire(string operation, Func<IntPtr> open)
        {
            IntPtr handle = Observe(operation, open);
            if (handle == IntPtr.Zero)
                throw Refusal(operation, "null-handle");
            return handle;
        }

        private T Observe<T>(string operation, Func<T> query)
        {
            RequireObservation(operation);
            try
            {
                return query();
            }
            catch (Win32Exception error)
            {
                throw NativeRefusal(operation, error);
            }
        }

        private void RequireObservation(string operation)
        {
            RequireTime(operation);
            try
            {
                verifyActor();
            }
            catch (TimeoutException error)
            {
                throw Refusal(operation, "deadline", error);
            }
            catch (Exception error)
            {
                throw Refusal(operation, "deployment-actor", error);
            }
            RequireTime(operation);
        }

        private SamplePair ReadPair()
        {
            Sample engineSample = ReadRole(engine, "engine", initial == null ? null : initial.Engine);
            Sample brokerSample = ReadRole(broker, "broker", initial == null ? null : initial.Broker);
            RequireObservation("sample-complete");
            return new SamplePair(engineSample, brokerSample);
        }

        private Sample ReadRole(IntPtr service, string role, Sample original)
        {
            string configurationOperation = "query-configuration-" + role;
            Configuration configuration = Observe(configurationOperation,
                delegate { return api.QueryConfiguration(service); });
            if (configuration.StartType != ServiceDisabled)
                throw Refusal(configurationOperation, "not-disabled");
            if (original != null && configuration.ServiceType != original.ConfigurationType)
                throw Refusal(configurationOperation, "type-changed");

            string statusOperation = "query-status-" + role;
            Status status = Observe(statusOperation, delegate { return api.QueryStatus(service); });
            if (status.CurrentState != ServiceStopped)
                throw Refusal(statusOperation, "not-stopped");
            if (original != null && status.ServiceType != original.StatusType)
                throw Refusal(statusOperation, "type-changed");
            return new Sample(configuration, status);
        }

        private void CaptureRetirementDeadline(List<Exception> laterFailures)
        {
            if (failure != null)
                return;
            try
            {
                RequireTime("retire");
            }
            catch (Exception error)
            {
                RetainFailure(error, laterFailures);
            }
        }

        private void Close(ref IntPtr slot, string operation, List<Exception> laterFailures)
        {
            IntPtr handle = slot;
            slot = IntPtr.Zero;
            if (handle == IntPtr.Zero)
                return;
            CaptureRetirementDeadline(laterFailures);
            try
            {
                api.Close(handle);
            }
            catch (Win32Exception error)
            {
                RetainFailure(NativeRefusal(operation, error), laterFailures);
            }
            catch (Exception error)
            {
                RetainFailure(error, laterFailures);
            }
        }

        private void RetainFailure(Exception error, List<Exception> laterFailures)
        {
            if (failure == null)
                failure = error;
            else
                laterFailures.Add(error);
        }

        private static Exception Refusal(string operation, string kind, Exception cause = null)
        {
            InvalidOperationException error = new InvalidOperationException(
                "prestart-scm-window:" + operation + ":" + kind, cause);
            error.Data["Operation"] = operation;
            error.Data["Kind"] = kind;
            return error;
        }

        private static Exception NativeRefusal(string operation, Win32Exception cause)
        {
            bool missing = cause.NativeErrorCode == ErrorServiceDoesNotExist
                && (operation == "open-engine" || operation == "open-broker");
            Exception error = Refusal(operation, missing ? "service-missing" : "native-failure");
            error.Data["NativeErrorCode"] = cause.NativeErrorCode;
            return error;
        }

        internal struct Configuration
        {
            internal readonly uint ServiceType;
            internal readonly uint StartType;

            internal Configuration(uint serviceType, uint startType)
            {
                ServiceType = serviceType;
                StartType = startType;
            }
        }

        internal struct Status
        {
            internal readonly uint ServiceType;
            internal readonly uint CurrentState;

            internal Status(uint serviceType, uint currentState)
            {
                ServiceType = serviceType;
                CurrentState = currentState;
            }
        }

        internal interface IApi
        {
            IntPtr Connect();
            IntPtr Open(IntPtr manager, bool broker);
            Configuration QueryConfiguration(IntPtr service);
            Status QueryStatus(IntPtr service);
            void Close(IntPtr handle);
        }

        private sealed class NativeApi : IApi
        {
            internal static readonly NativeApi Instance = new NativeApi();
            private const uint ManagerConnect = 0x0001;
            private const uint ServiceQueryConfig = 0x0001;
            private const uint ServiceQueryStatus = 0x0004;
            private const int ConfigurationBytes = 8192;
            private const int ProcessStatusBytes = 9 * sizeof(uint);
            private const int ProcessStatusInformation = 0;
            private const string EngineName = "NevroTick-Lab-S02-Engine";
            private const string BrokerName = "NevroTick-Lab-S02-Broker";

            public IntPtr Connect()
            {
                IntPtr handle = OpenSCManagerW(IntPtr.Zero, IntPtr.Zero, ManagerConnect);
                if (handle == IntPtr.Zero)
                    throw NativeError();
                return handle;
            }

            public IntPtr Open(IntPtr manager, bool broker)
            {
                string name = broker ? BrokerName : EngineName;
                IntPtr handle = OpenServiceW(manager, name, ServiceQueryConfig | ServiceQueryStatus);
                if (handle == IntPtr.Zero)
                    throw NativeError();
                return handle;
            }

            public Configuration QueryConfiguration(IntPtr service)
            {
                IntPtr buffer = Marshal.AllocHGlobal(ConfigurationBytes);
                try
                {
                    uint required;
                    if (!QueryServiceConfigW(service, buffer, ConfigurationBytes, out required))
                        throw NativeError();
                    // QUERY_SERVICE_CONFIGW starts with two DWORDs. No pointer is followed.
                    return new Configuration(unchecked((uint)Marshal.ReadInt32(buffer, 0)),
                        unchecked((uint)Marshal.ReadInt32(buffer, sizeof(uint))));
                }
                finally
                {
                    Marshal.FreeHGlobal(buffer);
                }
            }

            public Status QueryStatus(IntPtr service)
            {
                IntPtr buffer = Marshal.AllocHGlobal(ProcessStatusBytes);
                try
                {
                    uint required;
                    if (!QueryServiceStatusEx(service, ProcessStatusInformation, buffer, ProcessStatusBytes, out required))
                        throw NativeError();
                    // The first two DWORDs are type and state. A Stopped PID is invalid.
                    return new Status(unchecked((uint)Marshal.ReadInt32(buffer, 0)),
                        unchecked((uint)Marshal.ReadInt32(buffer, sizeof(uint))));
                }
                finally
                {
                    Marshal.FreeHGlobal(buffer);
                }
            }

            public void Close(IntPtr handle)
            {
                if (!CloseServiceHandle(handle))
                    throw NativeError();
            }

            internal static void CloseAbandoned(IntPtr handle)
            {
                if (handle != IntPtr.Zero)
                    CloseServiceHandle(handle);
            }

            private static Win32Exception NativeError()
            {
                return new Win32Exception(Marshal.GetLastWin32Error(), "SCM native operation failed");
            }

            [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
            private static extern IntPtr OpenSCManagerW(IntPtr machine, IntPtr database, uint access);

            [DllImport("advapi32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
            private static extern IntPtr OpenServiceW(IntPtr manager, string service, uint access);

            [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
            [return: MarshalAs(UnmanagedType.Bool)]
            private static extern bool QueryServiceConfigW(IntPtr service, IntPtr configuration,
                uint bytes, out uint required);

            [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
            [return: MarshalAs(UnmanagedType.Bool)]
            private static extern bool QueryServiceStatusEx(IntPtr service, int level,
                IntPtr status, uint bytes, out uint required);

            [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
            [return: MarshalAs(UnmanagedType.Bool)]
            private static extern bool CloseServiceHandle(IntPtr service);
        }
    }
}
