// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.IO;

namespace NevroTick
{
    public static class PrestartScmWindowControls
    {
        private const int TimeoutMilliseconds = 100;
        private const string Acquisition = "connect,open-engine,open-broker";
        private const string Retirement = "close-broker,close-engine,close-manager";

        private sealed class ScriptedApi : PrestartScmWindow.IApi
        {
            private static readonly IntPtr Manager = new IntPtr(25);
            private static readonly IntPtr Engine = new IntPtr(271);
            private static readonly IntPtr Broker = new IntPtr(613);
            private readonly HashSet<IntPtr> live = new HashSet<IntPtr>();
            public readonly List<string> Calls = new List<string>();
            public readonly Dictionary<string, int> Failures = new Dictionary<string, int>();
            public uint BrokerConfigType = 0x210;
            public uint BrokerStartType = 4;
            public uint BrokerStatusType = 0x10;
            public uint BrokerState = 1;
            public long Elapsed;
            public bool ExpireAfterBrokerStatus;
            public bool ActorCurrent = true;
            public string WithdrawAfterStatus;
            public readonly Exception ActorRefusal = new InvalidOperationException("scripted-actor-withdrawn");

            public IntPtr Connect()
            {
                Record("connect");
                Assert(live.Add(Manager), "manager acquired twice");
                return Manager;
            }

            public IntPtr Open(IntPtr manager, bool broker)
            {
                Assert(manager == Manager && live.Contains(manager), "service opened without original manager");
                Record(broker ? "open-broker" : "open-engine");
                IntPtr service = broker ? Broker : Engine;
                Assert(live.Add(service), "service acquired twice");
                return service;
            }

            public PrestartScmWindow.Configuration QueryConfiguration(IntPtr service)
            {
                bool broker = CheckService(service);
                Record(broker ? "config-broker" : "config-engine");
                return new PrestartScmWindow.Configuration(broker ? BrokerConfigType : 0x80000210U,
                    broker ? BrokerStartType : 4U);
            }

            public PrestartScmWindow.Status QueryStatus(IntPtr service)
            {
                bool broker = CheckService(service);
                string operation = broker ? "status-broker" : "status-engine";
                Record(operation);
                if (broker && ExpireAfterBrokerStatus) Elapsed = TimeoutMilliseconds;
                if (operation == WithdrawAfterStatus) ActorCurrent = false;
                return new PrestartScmWindow.Status(broker ? BrokerStatusType : 0x80000010U,
                    broker ? BrokerState : 1U);
            }

            public void Close(IntPtr handle)
            {
                Assert(live.Remove(handle), "closed invalid, foreign or previously retired handle");
                Record(handle == Broker ? "close-broker" : handle == Engine ? "close-engine" : "close-manager");
            }

            public PrestartScmWindow OpenWindow()
            {
                return PrestartScmWindow.Open(this, () => Elapsed, delegate
                {
                    if (!ActorCurrent) throw ActorRefusal;
                }, TimeoutMilliseconds);
            }

            private bool CheckService(IntPtr service)
            {
                Assert(live.Contains(Engine) && live.Contains(Broker), "sample taken outside the held pair");
                Assert(service == Engine || service == Broker, "sample used a foreign handle");
                return service == Broker;
            }

            private void Record(string operation)
            {
                Calls.Add(operation);
                int code;
                if (Failures.TryGetValue(operation, out code)) throw new Win32Exception(code);
            }
        }

        public static void Run()
        {
            HealthyPair();
            RefuseObservationFromScriptedWindow();
            ObservationPublicationAndCollectionCutoff();
            PublicationWithdrawsAfterSampleOnArchiveFailure();
            CollectionClockRefusalIsPermanent();
            PartialOpen();
            InitialRefusal(a => a.BrokerStartType = 3, "query-configuration-broker", "not-disabled",
                ",config-engine,status-engine,config-broker");
            InitialRefusal(a => a.BrokerState = 4, "query-status-broker", "not-stopped",
                ",config-engine,status-engine,config-broker,status-broker");
            LaterRefusal(a => a.BrokerConfigType ^= 0x200, "query-configuration-broker", "type-changed");
            LaterRefusal(a => a.BrokerStatusType ^= 0x200, "query-status-broker", "type-changed");
            LaterRefusal(a => a.BrokerStartType = 3, "query-configuration-broker", "not-disabled");
            LaterRefusal(a => a.BrokerState = 4, "query-status-broker", "not-stopped");
            LaterRefusal(a => a.Failures.Add("config-broker", 5), "query-configuration-broker", "native-failure", 5,
                failClose: true);
            LaterRefusal(a => a.Failures.Add("status-broker", 6), "query-status-broker", "native-failure", 6);
            LaterRefusal(a => a.Elapsed = TimeoutMilliseconds, "query-configuration-engine", "deadline",
                rejectBeforeQueries: true);
            LaterRefusal(a => a.Elapsed = 0, "query-configuration-engine", "invalid-clock", rejectBeforeQueries: true);
            LaterRefusal(a => a.ExpireAfterBrokerStatus = true, "sample-complete", "deadline");
            ActorRefusedBeforeConnect();
            ActorWithdrawn("status-engine", "query-configuration-broker", "config-engine,status-engine");
            ActorWithdrawn("status-broker", "sample-complete", "config-engine,status-engine,config-broker,status-broker");
            RetirementFailure();
            RetirementDeadline();
        }

        private static void RefuseObservationFromScriptedWindow()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                api.Calls.Clear();
                Exception identityRefusal = Refuse(() => window.StartVerifiedIdentity(null, 0, null, null, 0,
                    null, 0, null, 0, null, Guid.Empty, null, null, null), "verified identity from a scripted window");
                Assert(identityRefusal is InvalidOperationException &&
                    identityRefusal.Message == "prestart-scm-window:deployment-scope",
                    "scripted window did not refuse verified identity admission: " + identityRefusal);
                Exception refusal = Refuse(() => window.StartVerifiedObservation(null, 0, null),
                    "verified observation from a scripted window");
                Assert(refusal is InvalidOperationException && refusal.Message == "prestart-scm-window:deployment-scope",
                    "scripted window did not refuse before verified observation admission: " + refusal);
                Exception confirmation = Refuse(() => window.ConfirmDeployment(),
                    "deployment confirmation from a scripted window");
                Assert(confirmation is InvalidOperationException &&
                    confirmation.Message == "prestart-scm-window:deployment-scope",
                    "scripted window attempted deployment confirmation: " + confirmation);
                AssertTrace(api, String.Empty);
                AssertSample(window.Check());
                AssertSamples(api.Calls);
                api.Calls.Clear();
                Assert(window.Retire() == null, "scripted window lost retirement after refusing observation");
                AssertTrace(api, Retirement);
            }
        }

        private static void ObservationPublicationAndCollectionCutoff()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                api.Calls.Clear();
                int checks = 0;
                api.Elapsed = TimeoutMilliseconds - 2;
                window.WithOriginalSample(delegate
                {
                    AssertSamples(api.Calls);
                    api.Elapsed = TimeoutMilliseconds - 1;
                    checks++;
                });
                Assert(checks == 1, "healthy rooted publication skipped its callback");

                api.Calls.Clear();
                window.RequireCollectionTime();
                AssertTrace(api, String.Empty);
                Exception late = Refuse(() => window.WithOriginalSample(delegate
                {
                    AssertSamples(api.Calls);
                    api.Elapsed = TimeoutMilliseconds;
                    checks++;
                }), "original SCM deadline elapsed during archive confirmation");
                AssertFailure(late, "sample-complete", "deadline");
                Assert(checks == 2, "late publication did not reach its archive callback");
                AssertSamples(api.Calls);

                api.Calls.Clear();
                Exception later = Refuse(() => window.RequireCollectionTime(), "collection after original cutoff");
                Assert(Object.ReferenceEquals(later, late), "late publication refusal was not retained");
                AssertTrace(api, String.Empty);
                Assert(Object.ReferenceEquals(window.Retire(), late), "retirement lost late publication refusal");
                AssertTrace(api, Retirement);
            }
        }

        private static void PublicationWithdrawsAfterSampleOnArchiveFailure()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                api.Calls.Clear();
                InvalidOperationException archiveFailure = new InvalidOperationException("scripted-archive-withdrawn");
                Exception failure = Refuse(() => window.WithOriginalSample(delegate
                {
                    AssertSamples(api.Calls);
                    throw archiveFailure;
                }), "archive withdrawn after the held SCM sample");
                Assert(Object.ReferenceEquals(failure, archiveFailure), "post-sample check lost the archive failure");

                api.Calls.Clear();
                Exception later = Refuse(() => window.Check(), "sample after publication withdrawal");
                Assert(Object.ReferenceEquals(later, failure), "publication withdrawal was not permanent");
                AssertTrace(api, String.Empty);
                Assert(Object.ReferenceEquals(window.Retire(), failure), "retirement lost publication withdrawal");
                AssertTrace(api, Retirement);
            }
        }

        private static void CollectionClockRefusalIsPermanent()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                api.Calls.Clear();
                api.Elapsed = -1;
                Exception failure = Refuse(() => window.RequireCollectionTime(), "stopped collection clock");
                AssertFailure(failure, "collection", "invalid-clock");
                api.Elapsed = TimeoutMilliseconds - 1;
                Exception afterRestart = Refuse(() => window.Check(), "sampling after collection clock restart");
                Assert(Object.ReferenceEquals(afterRestart, failure), "clock restart revived the original SCM window");
                AssertTrace(api, String.Empty);
                Assert(Object.ReferenceEquals(window.Retire(), failure), "retirement lost collection clock refusal");
                AssertTrace(api, Retirement);
            }
        }

        public static void ExpectUnsignedObservationRefused(string archivePath, long expectedArchiveBytes,
            string expectedArchiveSha256, string expectedDeploymentSid)
        {
            const int timeoutMilliseconds = 15000;
            Stopwatch originalClock = Stopwatch.StartNew();
            string signerPin = new string('0', 64);
            using (PrestartActor actor = PrestartActor.CaptureCurrent(originalClock, timeoutMilliseconds))
            using (PrestartObserverChild.ArchiveLease archive = PrestartObserverChild.OpenArchive(
                archivePath, expectedArchiveBytes, expectedArchiveSha256, originalClock, timeoutMilliseconds))
            {
                int windowChecks = 0;
                Action<Action> checkObservationWindow = _ => windowChecks++;
                AssertObservationRefusal(archive, expectedArchiveSha256, null, expectedDeploymentSid, actor,
                    originalClock, timeoutMilliseconds, checkObservationWindow, typeof(ArgumentException),
                    "prestart-observer:archive-signer-sha256");
                AssertObservationRefusal(archive, null, signerPin, expectedDeploymentSid, actor,
                    originalClock, timeoutMilliseconds, checkObservationWindow, typeof(InvalidDataException),
                    "prestart-observer:observation-archive-digest");
                AssertObservationRefusal(archive, expectedArchiveSha256, signerPin, expectedDeploymentSid, actor,
                    Stopwatch.StartNew(), timeoutMilliseconds, checkObservationWindow, typeof(InvalidOperationException),
                    "prestart-observer:archive-scope");
                AssertObservationRefusal(archive, expectedArchiveSha256, signerPin, expectedDeploymentSid, actor,
                    originalClock, timeoutMilliseconds, checkObservationWindow, typeof(InvalidDataException),
                    "prestart-observer:archive-signature-required");
                string inputPin = new string('1', 64);
                Exception identityRefusal = Refuse(() => PrestartObserverChild.StartVerifiedWindowIdentity(
                    null, 1, inputPin, null, 1, inputPin, 1, inputPin, 1, inputPin, Guid.Empty,
                    archive, expectedDeploymentSid, expectedArchiveSha256, signerPin, actor, originalClock,
                    timeoutMilliseconds, checkObservationWindow, delegate { windowChecks++; }),
                    "unverified archive for the same-window identity");
                Assert(identityRefusal is InvalidDataException &&
                    identityRefusal.Message == "prestart-observer:archive-signature-required",
                    "same-window identity did not check its held signer before launch: " + identityRefusal);
                AssertNoChild(identityRefusal);
                Assert(windowChecks == 0, "a refused archive reached the observation window");

                // The unsigned fixture exercises the shared archive/actor publication check.
                archive.ConfirmPublication(originalClock, timeoutMilliseconds, null, actor, "control");
                actor.Dispose();
                Exception actorRefusal = Refuse(() => archive.ConfirmPublication(
                    originalClock, timeoutMilliseconds, null, actor, "control"),
                    "archive publication with the original actor disposed");
                ObjectDisposedException disposedActor = actorRefusal as ObjectDisposedException;
                Assert(disposedActor != null && disposedActor.ObjectName == "PrestartActor",
                    "the intact archive masked the disposed actor: " + actorRefusal);
                archive.Dispose();
                Exception archiveRefusal = Refuse(() => archive.ConfirmPublication(
                    originalClock, timeoutMilliseconds, null, actor, "control"), "retired archive publication");
                Assert(archiveRefusal is InvalidOperationException &&
                    archiveRefusal.Message == "prestart-observer:archive-retired",
                    "the disposed actor masked the first archive refusal: " + archiveRefusal);
            }
            using (FileStream reopened = File.Open(archivePath, FileMode.Open, FileAccess.Read, FileShare.None))
                Assert(reopened.Length == expectedArchiveBytes, "the original archive did not reopen exclusively");
        }

        private static void AssertObservationRefusal(PrestartObserverChild.ArchiveLease archive, string archivePin,
            string signerPin, string expectedDeploymentSid, PrestartActor actor, Stopwatch clock,
            int timeoutMilliseconds, Action<Action> checkWindow, Type expectedType, string expectedReason)
        {
            Exception refusal = Refuse(delegate
            {
                // The refusal must reach archive admission before the invalid image descriptor.
                using (PrestartObserverChild child = PrestartObserverChild.StartVerifiedDeploymentObservation(
                    null, 0, null, archive, archivePin, signerPin, expectedDeploymentSid, actor, clock,
                    timeoutMilliseconds, checkWindow, delegate { }))
                {
                }
            }, expectedReason);
            Assert(refusal.GetType() == expectedType && refusal.Message == expectedReason,
                "expected " + expectedReason + ", received " + refusal);
            AssertNoChild(refusal);
        }

        private static void AssertNoChild(Exception refusal)
        {
            Assert(Object.Equals(refusal.Data["PrestartObserverJobCreated"], false) &&
                Object.Equals(refusal.Data["PrestartObserverProcessCreated"], false) &&
                Object.Equals(refusal.Data["PrestartObserverRetired"], true) &&
                Object.Equals(refusal.Data["PrestartObserverReleased"], true) &&
                !refusal.Data.Contains("PrestartObserverProcessId"), "archive refusal reported a native launch effect");
        }

        public static void ExpectAbsent(string expectedDeploymentSid)
        {
            Stopwatch clock = Stopwatch.StartNew();
            using (PrestartActor actor = PrestartActor.CaptureCurrent(clock, 5000))
            {
                foreach (string wrongSid in new string[] { "S-1-0-0", expectedDeploymentSid.ToLowerInvariant() })
                {
                    Assert(!String.Equals(wrongSid, expectedDeploymentSid, StringComparison.Ordinal),
                        "negative subject is the current canonical SID");
                    Exception wrong = Refuse(() => OpenAndRetire(actor, wrongSid, clock), "wrong deployment SID");
                    AssertFailure(wrong, "open-manager", "deployment-actor");
                    Assert(wrong.InnerException != null &&
                        wrong.InnerException.Message == "prestart-observer:identity-subject-actor",
                        "wrong deployment SID did not reach the live actor comparison");
                }
                Exception refusal = Refuse(() => OpenAndRetire(actor, expectedDeploymentSid, clock),
                    "absent fixed laboratory key");
                AssertFailure(refusal, "open-engine", "service-missing", 1060);
                Assert(refusal.Data["RetirementFailures"] == null, "native absence reported an SCM close failure");
            }
        }

        private static void OpenAndRetire(PrestartActor actor, string subject, Stopwatch clock)
        {
            using (PrestartScmWindow window = PrestartScmWindow.Open(actor, subject, clock, 5000))
            {
                Assert(window.Retire() == null, "unexpected installed pair could not be retired");
            }
        }

        private static void HealthyPair()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                AssertSample(window.Initial);
                AssertAcquisitionAndSamples(api.Calls);
                api.Calls.Clear();
                AssertSample(window.Check());
                AssertSamples(api.Calls);
                api.Calls.Clear();
                Assert(window.Retire() == null, "healthy pair retirement failed");
                AssertTrace(api, Retirement);
                int calls = api.Calls.Count;
                AssertFailure(Refuse(() => window.Check(), "sampling after retirement"), "check", "retired");
                Assert(window.Retire() == null, "clean retirement result was discarded");
                Assert(api.Calls.Count == calls, "operation after retirement reached the SCM");
            }
            AssertTrace(api, Retirement);
        }

        private static void PartialOpen()
        {
            ScriptedApi api = new ScriptedApi();
            api.Failures.Add("open-broker", 1060);
            api.Failures.Add("close-engine", 6);
            api.Failures.Add("close-manager", 5);
            Exception refusal = Refuse(() => api.OpenWindow(), "Broker open after Engine acquisition");
            AssertFailure(refusal, "open-broker", "service-missing", 1060);
            AssertRetirementFailures(refusal, "close-engine", 6, "close-manager", 5);
            AssertTrace(api, Acquisition + ",close-engine,close-manager");
        }

        private static void InitialRefusal(Action<ScriptedApi> inject, string operation, string kind, string samples)
        {
            ScriptedApi api = new ScriptedApi();
            inject(api);
            Exception refusal = Refuse(() => api.OpenWindow(), "invalid initial sample");
            AssertFailure(refusal, operation, kind);
            AssertTrace(api, Acquisition + samples + "," + Retirement);
        }

        private static void LaterRefusal(Action<ScriptedApi> inject, string operation, string kind, int nativeCode = 0,
            bool rejectBeforeQueries = false, bool failClose = false)
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                api.Elapsed = 1;
                AssertSample(window.Check());
                api.Calls.Clear();
                inject(api);
                string label = operation + ":" + kind;
                Exception refusal = Refuse(() => window.Check(), label);
                AssertFailure(refusal, operation, kind, nativeCode);
                Assert((api.Calls.Count == 0) == rejectBeforeQueries,
                    label + " reached the wrong observation phase");

                api.BrokerConfigType = 0x210;
                api.BrokerStartType = 4;
                api.BrokerStatusType = 0x10;
                api.BrokerState = 1;
                api.Failures.Clear();
                api.ExpireAfterBrokerStatus = false;
                api.Elapsed = 2;
                int callsAtRefusal = api.Calls.Count;
                Exception later = Refuse(() => window.Check(), label + " after restored inputs");
                Assert(Object.ReferenceEquals(refusal, later), label + " did not preserve the first refusal");
                Assert(api.Calls.Count == callsAtRefusal, label + " made new native calls after refusal");
                api.Calls.Clear();
                if (failClose)
                {
                    api.Failures.Add("close-broker", 1722);
                    api.Failures.Add("close-manager", 6);
                }
                Assert(Object.ReferenceEquals(refusal, window.Retire()), label + " retirement replaced the refusal");
                if (failClose) AssertRetirementFailures(refusal, "close-broker", 1722, "close-manager", 6);
                else Assert(refusal.Data["RetirementFailures"] == null, label + " reported an extra cleanup failure");
                AssertTrace(api, Retirement);
            }
            AssertTrace(api, Retirement);
        }

        private static void ActorRefusedBeforeConnect()
        {
            ScriptedApi api = new ScriptedApi();
            api.ActorCurrent = false;
            Exception refusal = Refuse(() => api.OpenWindow(), "actor withdrawn at admission");
            AssertFailure(refusal, "open-manager", "deployment-actor");
            Assert(Object.ReferenceEquals(api.ActorRefusal, refusal.InnerException), "original actor failure was lost");
            AssertTrace(api, String.Empty);
        }

        private static void ActorWithdrawn(string statusOperation, string refusalOperation, string expectedCalls)
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                PrestartScmWindow.SamplePair prior = window.Check();
                AssertSample(prior);
                api.Calls.Clear();
                api.WithdrawAfterStatus = statusOperation;
                Exception refusal = Refuse(() => window.Check(), "actor withdrawn during sampling");
                AssertFailure(refusal, refusalOperation, "deployment-actor");
                Assert(Object.ReferenceEquals(api.ActorRefusal, refusal.InnerException), "original actor failure was lost");
                AssertTrace(api, expectedCalls);
                AssertSample(prior);
                AssertSample(window.Initial);

                api.ActorCurrent = true;
                api.WithdrawAfterStatus = null;
                api.Calls.Clear();
                Exception restored = Refuse(() => window.Check(), "restoration cannot reopen a withdrawn window");
                Assert(Object.ReferenceEquals(refusal, restored), "authority refusal was not latched");
                AssertTrace(api, String.Empty);
                Assert(Object.ReferenceEquals(refusal, window.Retire()), "retirement replaced the actor refusal");
                Assert(refusal.Data["RetirementFailures"] == null, "retirement reported an unexpected close failure");
                AssertTrace(api, Retirement);
            }
            AssertTrace(api, Retirement);
        }

        private static void RetirementFailure()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                AssertSample(window.Check());
                api.Calls.Clear();
                api.Failures.Add("close-broker", 1722);
                api.Failures.Add("close-engine", 6);
                api.Failures.Add("close-manager", 5);
                Exception refusal = window.Retire();
                AssertFailure(refusal, "close-broker", "native-failure", 1722);
                AssertRetirementFailures(refusal, "close-engine", 6, "close-manager", 5);
                Refuse(() => window.Check(), "sampling after failed retirement");
                Assert(Object.ReferenceEquals(refusal, window.Retire()), "retirement failure was lost after release");
                AssertTrace(api, Retirement);
            }
            AssertTrace(api, Retirement);
        }

        private static void RetirementDeadline()
        {
            ScriptedApi api = new ScriptedApi();
            using (PrestartScmWindow window = api.OpenWindow())
            {
                api.Calls.Clear();
                api.Elapsed = TimeoutMilliseconds;
                Exception refusal = window.Retire();
                AssertFailure(refusal, "retire", "deadline");
                AssertTrace(api, Retirement);
            }
            AssertTrace(api, Retirement);
        }


        private static void AssertSample(PrestartScmWindow.SamplePair sample)
        {
            Assert(sample.Engine.ConfigurationType == 0x80000210U && sample.Engine.StartType == 4 &&
                sample.Engine.StatusType == 0x80000010U && sample.Engine.CurrentState == 1,
                "Engine raw fields or disabled/stopped state changed");
            Assert(sample.Broker.ConfigurationType == 0x210 && sample.Broker.StartType == 4 &&
                sample.Broker.StatusType == 0x10 && sample.Broker.CurrentState == 1,
                "Broker raw fields or disabled/stopped state changed");
        }

        private static void AssertAcquisitionAndSamples(List<string> calls)
        {
            Assert(calls.Count == 7 && String.Join(",", calls.GetRange(0, 3)) == Acquisition,
                "unexpected attempt order: " + String.Join(",", calls));
            AssertSamples(calls.GetRange(3, 4));
        }

        private static void AssertSamples(List<string> calls)
        {
            List<string> sorted = new List<string>(calls);
            sorted.Sort(StringComparer.Ordinal);
            Assert(String.Join(",", sorted) == "config-broker,config-engine,status-broker,status-engine",
                "expected one raw config and status read per original handle: " + String.Join(",", calls));
        }

        private static void AssertTrace(ScriptedApi api, string expected)
        {
            Assert(String.Join(",", api.Calls) == expected,
                "expected " + expected + ", observed " + String.Join(",", api.Calls));
        }

        private static void AssertFailure(Exception refusal, string operation, string kind, int code = 0)
        {
            object expectedCode = code == 0 ? null : (object)code;
            Assert(refusal != null && Object.Equals(refusal.Data["Operation"], operation) &&
                Object.Equals(refusal.Data["Kind"], kind) && Object.Equals(refusal.Data["NativeErrorCode"], expectedCode),
                "expected " + operation + ":" + kind + " (Win32 " + code + "), received " + refusal);
        }

        private static void AssertRetirementFailures(Exception refusal, string firstOperation, int firstCode,
            string secondOperation, int secondCode)
        {
            Exception[] later = refusal.Data["RetirementFailures"] as Exception[];
            Assert(later != null && later.Length == 2, "wrong number of later SCM close failures");
            AssertFailure(later[0], firstOperation, "native-failure", firstCode);
            AssertFailure(later[1], secondOperation, "native-failure", secondCode);
        }

        private static Exception Refuse(Action act, string label)
        {
            try { act(); }
            catch (Exception refusal) { return refusal; }
            throw new InvalidOperationException(label + " was accepted");
        }

        private static void Assert(bool expected, string message)
        {
            if (!expected) throw new InvalidOperationException("scm-window-control: " + message);
        }
    }
}
