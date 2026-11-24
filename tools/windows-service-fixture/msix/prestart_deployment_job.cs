// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using Microsoft.Win32.SafeHandles;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;

namespace NevroTick
{
    public sealed partial class PrestartObserverChild
    {
        private const string DeploymentEntryName = "prestart_deployment_owner.ps1";
        private const string DeploymentGuardianName = "prestart_deployment_guardian.ps1";
        private const int MaximumDeploymentRetirementAllowance = 30000;
        private const uint DeploymentActiveProcessLimit = 2;
        private const uint DeploymentGuardianActiveProcessLimit = 3;
        private const int MaximumRecoveryMembers = 8;
        private const int BasicProcessIdList = 3;
        private const uint DuplicateSameAccess = 2;
        private const uint FileTypeDisk = 1;
        private const uint FileReadAttributes = 0x80;
        private const uint FileShareWrite = 2;
        private const uint OpenDirectory = 0x02000000;
        private const uint OpenReparsePoint = 0x00200000;
        private const uint FileTypePipe = 3;
        private const uint StdInputHandle = unchecked((uint)-10);
        private const uint StdOutputHandle = unchecked((uint)-11);
        private const uint StdErrorHandle = unchecked((uint)-12);
        private static int deploymentJobTaken;

        public sealed class DeploymentJobReport
        {
            public int? Pid { get; private set; }
            public int? ExitCode { get; private set; }
            public bool JobEmpty { get; private set; }
            public bool Recovered { get; private set; }
            public uint? ActiveAtRecovery { get; private set; }
            public uint[] MemberPidsAtRecovery { get; private set; }
            public uint? TotalProcesses { get; private set; }
            public uint? VerifiedProcessLimit { get; internal set; }
            public uint? TotalAtRecovery { get; internal set; }
            public uint? LimitTerminationsAtRecovery { get; internal set; }
            public uint? LimitTerminationsAtEmpty { get; internal set; }
            public long? RecoveryStartTick { get; internal set; }
            public long? TerminationAcceptedTick { get; internal set; }
            public long? JobEmptyTick { get; internal set; }
            public long TickFrequency { get { return Stopwatch.Frequency; } }
            public Exception Failure { get; private set; }
            public Exception RecoveryFailure { get; private set; }

            internal DeploymentJobReport(int? pid, int? exitCode, uint? totalProcesses, bool recovered,
                uint? activeAtRecovery, uint[] memberPidsAtRecovery, Exception failure, Exception recoveryFailure)
            {
                Pid = pid;
                ExitCode = exitCode;
                JobEmpty = totalProcesses.HasValue;
                Recovered = recovered;
                ActiveAtRecovery = activeAtRecovery;
                MemberPidsAtRecovery = memberPidsAtRecovery;
                TotalProcesses = totalProcesses;
                Failure = failure;
                RecoveryFailure = recoveryFailure;
            }
        }

        // The dedicated guardian has trusted, private redirected standard handles and
        // exclusive use of its process. Its parent pre-verifies companion inputs and
        // owns the guardian deadline, including synchronous native-call interruption.
        // This method retains the fixed script and interpreter; it emits no wire bytes.
        public static DeploymentJobReport RunDeploymentOwnerJob(string entryPath, long entryBytes,
            string entrySha256, long powerShellBytes, string powerShellSha256, Guid runId,
            int workMilliseconds, int retirementMilliseconds)
        {
            Stopwatch clock = Stopwatch.StartNew();
            if (Interlocked.Exchange(ref deploymentJobTaken, 1) != 0)
                return new DeploymentJobReport(null, null, null, false, null, null,
                    new InvalidOperationException("prestart-deployment-job:already-used"), null);
            if (!DeploymentDeadlinesValid(workMilliseconds, retirementMilliseconds))
                return new DeploymentJobReport(null, null, null, false, null, null,
                    new ArgumentException("prestart-deployment-job:deadlines"), null);

            DeploymentJob job = new DeploymentJob(clock, workMilliseconds, retirementMilliseconds, null);
            return job.Run(entryPath, entryBytes, entrySha256, powerShellBytes, powerShellSha256, runId);
        }

        // The dedicated watchdog owns the original guardian and its descendants
        // until the outer job retires. The guardian's independent deadlines govern
        // the inner owner and may be later than the watchdog's work cutoff.
        public static DeploymentJobReport RunDeploymentGuardianJob(string guardianPath, long guardianBytes,
            string guardianSha256, long powerShellBytes, string powerShellSha256, string ownerPath,
            long ownerBytes, string ownerSha256, Guid runId, int guardianWorkMilliseconds,
            int guardianRetirementMilliseconds, int watchdogWorkMilliseconds, int watchdogRetirementMilliseconds)
        {
            Stopwatch clock = Stopwatch.StartNew();
            if (Interlocked.Exchange(ref deploymentJobTaken, 1) != 0)
                return new DeploymentJobReport(null, null, null, false, null, null,
                    new InvalidOperationException("prestart-deployment-job:already-used"), null);
            if (!DeploymentDeadlinesValid(guardianWorkMilliseconds, guardianRetirementMilliseconds) ||
                !DeploymentDeadlinesValid(watchdogWorkMilliseconds, watchdogRetirementMilliseconds))
                return new DeploymentJobReport(null, null, null, false, null, null,
                    new ArgumentException("prestart-deployment-job:deadlines"), null);

            DeploymentGuardianArguments guardian = new DeploymentGuardianArguments(ownerPath, ownerBytes,
                ownerSha256, guardianWorkMilliseconds, guardianRetirementMilliseconds);
            DeploymentJob job = new DeploymentJob(clock, watchdogWorkMilliseconds,
                watchdogRetirementMilliseconds, guardian);
            return job.Run(guardianPath, guardianBytes, guardianSha256, powerShellBytes, powerShellSha256, runId);
        }

        private static bool DeploymentDeadlinesValid(int workMilliseconds, int retirementMilliseconds)
        {
            return workMilliseconds > 0 && retirementMilliseconds <= MaxTimeoutMilliseconds &&
                retirementMilliseconds > workMilliseconds &&
                retirementMilliseconds - workMilliseconds <= MaximumDeploymentRetirementAllowance;
        }

        private sealed class DeploymentGuardianArguments
        {
            private readonly string ownerPath;
            private readonly long ownerBytes;
            private readonly string ownerSha256;
            private readonly int workMilliseconds;
            private readonly int retirementMilliseconds;

            internal DeploymentGuardianArguments(string ownerPath, long ownerBytes, string ownerSha256,
                int workMilliseconds, int retirementMilliseconds)
            {
                this.ownerPath = ownerPath;
                this.ownerBytes = ownerBytes;
                this.ownerSha256 = ownerSha256;
                this.workMilliseconds = workMilliseconds;
                this.retirementMilliseconds = retirementMilliseconds;
            }

            internal void CheckOwner()
            {
                CheckLocalPath(ownerPath, DeploymentEntryName);
                CheckFileExpectation(ownerBytes, ownerSha256, MaxIdentityFileBytes, "deployment-owner");
            }

            internal void AppendArguments(StringBuilder command, long powerShellBytes, string powerShellSha256,
                Guid runId)
            {
                command.Append(" -EntryPath \"").Append(ownerPath).Append("\" -EntryBytes ")
                    .Append(ownerBytes.ToString(CultureInfo.InvariantCulture))
                    .Append(" -EntrySha256 ").Append(ownerSha256)
                    .Append(" -PowerShellBytes ").Append(powerShellBytes.ToString(CultureInfo.InvariantCulture))
                    .Append(" -PowerShellSha256 ").Append(powerShellSha256)
                    .Append(" -RunId ").Append(runId.ToString("D"))
                    .Append(" -WorkMilliseconds ").Append(workMilliseconds.ToString(CultureInfo.InvariantCulture))
                    .Append(" -RetirementMilliseconds ").Append(retirementMilliseconds.ToString(CultureInfo.InvariantCulture));
            }
        }

        // Owns all native handles on the calling thread. Neither original process
        // custody nor the unnamed job handle is transferred to the client or child.
        private sealed class DeploymentJob
        {
            private readonly Stopwatch clock;
            private readonly int workMilliseconds;
            private readonly int retirementMilliseconds;
            private readonly DeploymentGuardianArguments guardian;
            private readonly KernelHandle process = new KernelHandle();
            private readonly KernelHandle thread = new KernelHandle();
            private readonly KernelHandle input = new KernelHandle();
            private readonly KernelHandle output = new KernelHandle();
            private readonly KernelHandle error = new KernelHandle();
            private readonly List<Exception> recoveryFailures = new List<Exception>();
            private KernelHandle job;
            private SafeFileHandle scratch;
            private HeldFile image;
            private HeldFile entry;
            private PrestartActor actor;
            private PrestartActor.ChildToken childToken;
            private bool launchAttempted;
            private bool processCreated;
            private bool contained;
            private bool terminationRequested;
            private int? pid;
            private int? exitCode;
            private uint? totalProcesses;
            private uint? verifiedProcessLimit;
            private JobAccounting? recoveryAccounting;
            private uint? limitTerminationsAtEmpty;
            private uint[] memberPidsAtRecovery;
            private long? recoveryStartTick;
            private long? terminationAcceptedTick;
            private long? jobEmptyTick;

            internal DeploymentJob(Stopwatch clock, int workMilliseconds, int retirementMilliseconds,
                DeploymentGuardianArguments guardian)
            {
                this.clock = clock;
                this.workMilliseconds = workMilliseconds;
                this.retirementMilliseconds = retirementMilliseconds;
                this.guardian = guardian;
            }

            internal DeploymentJobReport Run(string entryPath, long entryBytes, string entrySha256,
                long powerShellBytes, string powerShellSha256, Guid runId)
            {
                Exception failure = null;
                try
                {
                    Start(entryPath, entryBytes, entrySha256, powerShellBytes, powerShellSha256, runId);
                    ObserveExit(workMilliseconds);
                    if (exitCode.Value != 0)
                        failure = new InvalidDataException(guardian == null ?
                            "prestart-deployment-job:owner-exit" : "prestart-deployment-job:guardian-exit");
                    CloseOwnedHandle(process, "close-process");
                    ObserveEmptyJob(workMilliseconds);
                    if (failure == null)
                    {
                        childToken.VerifyUnchanged(clock, workMilliseconds);
                        actor.VerifyCurrent(clock, workMilliseconds);
                        image.Confirm(clock, workMilliseconds);
                        entry.Confirm(clock, workMilliseconds);
                        RequireTime(workMilliseconds, "work-deadline");
                    }
                }
                catch (Exception problem)
                {
                    if (failure == null)
                        failure = problem;
                    else
                        recoveryFailures.Add(problem);
                    if (launchAttempted && !totalProcesses.HasValue)
                        Recover();
                }
                finally
                {
                    // The job's checked close also requests kill-on-close if
                    // explicit recovery could not confirm retirement.
                    ReleaseHandle(job, "close-job");
                    ReleaseHandle(thread, "close-thread");
                    ReleaseHandle(process, "close-process");
                    ReleaseHandle(input, "close-stdin");
                    ReleaseHandle(output, "close-stdout");
                    ReleaseHandle(error, "close-stderr");
                    ReleaseHandle(scratch, "close-scratch");
                    Release(childToken);
                    Release(actor);
                    Release(entry);
                    Release(image);
                }

                if (launchAttempted)
                    TryRecover(() => RequireTime(retirementMilliseconds, "release-deadline"));

                Exception recoveryFailure = null;
                if (recoveryFailures.Count == 1)
                    recoveryFailure = recoveryFailures[0];
                else if (recoveryFailures.Count > 1)
                    recoveryFailure = new AggregateException("prestart-deployment-job:retirement", recoveryFailures);
                bool recovered = terminationRequested && exitCode.HasValue && totalProcesses.HasValue &&
                    recoveryFailure == null;
                return new DeploymentJobReport(pid, exitCode, totalProcesses, recovered,
                    recoveryAccounting.HasValue ? (uint?)recoveryAccounting.Value.ActiveProcesses : null,
                    memberPidsAtRecovery, failure, recoveryFailure)
                {
                    VerifiedProcessLimit = verifiedProcessLimit,
                    TotalAtRecovery = recoveryAccounting.HasValue ? (uint?)recoveryAccounting.Value.TotalProcesses : null,
                    LimitTerminationsAtRecovery = recoveryAccounting.HasValue ?
                        (uint?)recoveryAccounting.Value.TotalTerminatedProcesses : null,
                    LimitTerminationsAtEmpty = limitTerminationsAtEmpty,
                    RecoveryStartTick = recoveryStartTick,
                    TerminationAcceptedTick = terminationAcceptedTick,
                    JobEmptyTick = jobEmptyTick
                };
            }

            private void Start(string entryPath, long entryBytes, string entrySha256,
                long powerShellBytes, string powerShellSha256, Guid runId)
            {
                if (runId == Guid.Empty)
                    throw new ArgumentException("prestart-deployment-job:run");
                RequireWindowsX64();
                string systemDirectory = Environment.SystemDirectory;
                string powerShellDirectory = systemDirectory + @"\WindowsPowerShell\v1.0";
                string powerShellPath = powerShellDirectory + "\\" + PowerShellName;
                CheckLocalPath(powerShellPath, PowerShellName);
                CheckLocalPath(entryPath, guardian == null ? DeploymentEntryName : DeploymentGuardianName);
                CheckFileExpectation(powerShellBytes, powerShellSha256, MaxIdentityFileBytes, "deployment-interpreter");
                CheckFileExpectation(entryBytes, entrySha256, MaxIdentityFileBytes, "deployment-entry");
                if (guardian != null)
                    guardian.CheckOwner();

                actor = PrestartActor.CaptureCurrent(clock, workMilliseconds);
                image = new HeldFile(powerShellPath, powerShellBytes, powerShellSha256, resolveSystemSpelling: true);
                entry = new HeldFile(entryPath, entryBytes, entrySha256);
                image.Admit(clock, workMilliseconds);
                entry.Admit(clock, workMilliseconds);
                string temporaryDirectory = OpenScratchDirectory();

                // GetStdHandle values are borrowed from the private launcher.
                // Only the three fresh duplicates go into HANDLE_LIST.
                DuplicateStandardHandle(StdInputHandle, input, false, "stdin");
                DuplicateStandardHandle(StdOutputHandle, output, false, "stdout");
                DuplicateStandardHandle(StdErrorHandle, error, true, "stderr");
                RequireTime(workMilliseconds, "work-deadline");

                // Default security attributes make this unnamed handle noninheritable.
                // The outer job also contains the guardian throughout inner job
                // creation, execution and retirement.
                job = CreateJobObjectW(IntPtr.Zero, null);
                if (job.IsInvalid)
                    throw DeploymentNativeError("create-job");
                const uint requiredFlags = JobObjectLimitActiveProcess | JobObjectLimitKillOnJobClose;
                uint requiredProcessLimit = guardian == null ?
                    DeploymentActiveProcessLimit : DeploymentGuardianActiveProcessLimit;
                JobExtendedLimits limits = new JobExtendedLimits();
                limits.Basic.LimitFlags = requiredFlags;
                limits.Basic.ActiveProcessLimit = requiredProcessLimit;
                if (!SetInformationJobObject(job, JobObjectExtendedLimitInformation, ref limits,
                    (uint)Marshal.SizeOf(typeof(JobExtendedLimits))))
                    throw DeploymentNativeError("configure-job");
                JobExtendedLimits observedLimits;
                if (!QueryJobLimits(job, JobObjectExtendedLimitInformation, out observedLimits,
                    (uint)Marshal.SizeOf(typeof(JobExtendedLimits)), IntPtr.Zero))
                    throw DeploymentNativeError("query-job-limits");
                if (observedLimits.Basic.LimitFlags != requiredFlags ||
                    observedLimits.Basic.ActiveProcessLimit != requiredProcessLimit)
                    throw new InvalidDataException("prestart-deployment-job:job-limits");
                verifiedProcessLimit = observedLimits.Basic.ActiveProcessLimit;

                string windowsDirectory = Path.GetDirectoryName(systemDirectory);
                string environment = "PSModuleAnalysisCachePath=NUL\0PSModulePath=" + powerShellDirectory +
                    "\\Modules\0SystemRoot=" + windowsDirectory + "\0TEMP=" + temporaryDirectory +
                    "\0TMP=" + temporaryDirectory + "\0windir=" + windowsDirectory + "\0";
                Launch(runId, powerShellBytes, powerShellSha256, environment);
                CloseOwnedHandle(input, "close-stdin");
                CloseOwnedHandle(output, "close-stdout");
                CloseOwnedHandle(error, "close-stderr");

                ConfirmSuspendedChild();
                uint suspendCount = ResumeThread(thread);
                if (suspendCount == uint.MaxValue)
                    throw DeploymentNativeError("resume-thread");
                if (suspendCount != 1)
                    throw new InvalidOperationException("prestart-deployment-job:suspend-count");
                CloseOwnedHandle(thread, "close-thread");
            }

            private string OpenScratchDirectory()
            {
                const string name = "prestart-temp";
                string path = Path.Combine(Path.GetDirectoryName(entry.Path), name);
                CheckLocalPath(path, name);
                // The caller provides an owned directory. Keep its original
                // handle without delete sharing through actual job retirement.
                scratch = CreateFileW(path, FileReadAttributes, FileShareRead | FileShareWrite, IntPtr.Zero,
                    OpenExisting, OpenDirectory | OpenReparsePoint, IntPtr.Zero);
                if (scratch.IsInvalid)
                    throw DeploymentNativeError("open-scratch");
                FileAttributes attributes = File.GetAttributes(path);
                if ((attributes & FileAttributes.Directory) == 0 || (attributes & FileAttributes.ReparsePoint) != 0)
                    throw new InvalidDataException("prestart-deployment-job:scratch-directory");
                ReadImageIdentity(scratch, path);
                return path;
            }

            private void ConfirmSuspendedChild()
            {
                RequireTime(workMilliseconds, "work-deadline");
                bool member;
                if (!IsProcessInJob(process, job, out member))
                    throw DeploymentNativeError("query-membership");
                if (!member)
                    throw new InvalidDataException("prestart-deployment-job:membership");
                contained = true;

                StringBuilder observed = new StringBuilder(MaxImagePath + 1);
                uint count = (uint)observed.Capacity;
                if (!QueryFullProcessImageNameW(process, 0, observed, ref count))
                    throw DeploymentNativeError("query-image");
                if (count != observed.Length ||
                    !string.Equals(observed.ToString(), image.Path, StringComparison.Ordinal))
                    throw new InvalidDataException("prestart-deployment-job:process-image");
                ushort processMachine;
                ushort nativeMachine;
                if (!IsWow64Process2(process, out processMachine, out nativeMachine))
                    throw DeploymentNativeError("query-machine");
                if (processMachine != 0 || nativeMachine != ImageFileMachineAmd64)
                    throw new InvalidDataException("prestart-deployment-job:process-machine");

                image.ConfirmPath();
                entry.ConfirmPath();
                childToken = actor.VerifyChild(process, clock, workMilliseconds);
                actor.VerifyCurrent(clock, workMilliseconds);
                RequireTime(workMilliseconds, "work-deadline");
            }

            private void Launch(Guid runId, long powerShellBytes, string powerShellSha256, string environmentBlock)
            {
                UIntPtr listBytes = UIntPtr.Zero;
                bool sized = InitializeProcThreadAttributeList(IntPtr.Zero, 2, 0, ref listBytes);
                int sizeError = Marshal.GetLastWin32Error();
                if (sized || sizeError != ErrorInsufficientBuffer)
                    throw new Win32Exception(sizeError, "prestart-deployment-job:size-attributes");
                if (listBytes.ToUInt64() == 0 || listBytes.ToUInt64() > 65536)
                    throw new InvalidOperationException("prestart-deployment-job:attribute-size");

                IntPtr attributes = IntPtr.Zero;
                IntPtr handles = IntPtr.Zero;
                IntPtr environment = IntPtr.Zero;
                bool initialized = false;
                try
                {
                    attributes = Marshal.AllocHGlobal((int)listBytes.ToUInt64());
                    handles = Marshal.AllocHGlobal(IntPtr.Size * 4);
                    environment = Marshal.StringToHGlobalUni(environmentBlock);
                    if (!InitializeProcThreadAttributeList(attributes, 2, 0, ref listBytes))
                        throw DeploymentNativeError("initialize-attributes");
                    initialized = true;

                    StartupInfoEx startup = new StartupInfoEx();
                    startup.Info.Size = (uint)Marshal.SizeOf(typeof(StartupInfoEx));
                    startup.Info.Flags = StartfUseStdHandles;
                    startup.AttributeList = attributes;
                    startup.Info.StdIn = input.DangerousGetHandle();
                    startup.Info.StdOut = output.DangerousGetHandle();
                    startup.Info.StdErr = error.DangerousGetHandle();
                    Marshal.WriteIntPtr(handles, 0, startup.Info.StdIn);
                    Marshal.WriteIntPtr(handles, IntPtr.Size, startup.Info.StdOut);
                    Marshal.WriteIntPtr(handles, IntPtr.Size * 2, startup.Info.StdErr);
                    IntPtr jobValue = IntPtr.Add(handles, IntPtr.Size * 3);
                    Marshal.WriteIntPtr(jobValue, job.DangerousGetHandle());
                    SetAttribute(attributes, ProcThreadAttributeHandleList, handles, IntPtr.Size * 3);
                    SetAttribute(attributes, ProcThreadAttributeJobList, jobValue, IntPtr.Size);

                    StringBuilder command = new StringBuilder("\"" + image.Path +
                        "\" -NoLogo -NoProfile -NonInteractive -ExecutionPolicy RemoteSigned -File \"" +
                        entry.Path + "\"");
                    if (guardian == null)
                        command.Append(" -RunId ").Append(runId.ToString("D"));
                    else
                        guardian.AppendArguments(command, powerShellBytes, powerShellSha256, runId);
                    string directory = Path.GetDirectoryName(image.Path);
                    uint flags = ExtendedStartupInfoPresent | CreateUnicodeEnvironment | CreateNoWindow | CreateSuspended;
                    ProcessInformation created;
                    actor.VerifyCurrent(clock, workMilliseconds);
                    RequireTime(workMilliseconds, "work-deadline");
                    launchAttempted = true;
                    if (!CreateProcessW(image.Path, command, IntPtr.Zero, IntPtr.Zero, true,
                        flags, environment, directory, ref startup, out created))
                        throw DeploymentNativeError("create-process");
                    processCreated = true;
                    process.Take(created.Process);
                    thread.Take(created.Thread);
                    if (created.ProcessId != 0 && created.ProcessId <= int.MaxValue)
                        pid = (int)created.ProcessId;
                    if (!pid.HasValue || process.IsInvalid || thread.IsInvalid)
                        throw new InvalidOperationException("prestart-deployment-job:process-identity");
                }
                finally
                {
                    if (initialized)
                        DeleteProcThreadAttributeList(attributes);
                    Marshal.FreeHGlobal(environment);
                    Marshal.FreeHGlobal(handles);
                    Marshal.FreeHGlobal(attributes);
                    GC.KeepAlive(input);
                    GC.KeepAlive(output);
                    GC.KeepAlive(error);
                    GC.KeepAlive(job);
                }
            }

            private void Recover()
            {
                recoveryStartTick = clock.ElapsedTicks;
                TryRecover(() => recoveryAccounting = ReadAccounting());
                TryRecover(() => memberPidsAtRecovery = ReadJobMemberPids());

                // An original whose membership was not confirmed could not
                // have been resumed. Keep recovery on the original handle.
                if (processCreated && !contained && !process.IsInvalid && !process.IsClosed)
                    TryRecover(() =>
                    {
                        if (TerminateProcess(process, ErrorCancelled))
                        {
                            terminationRequested = true;
                            return;
                        }
                        Win32Exception terminateFailure = DeploymentNativeError("terminate-unconfirmed-process");
                        // TerminateProcess also refuses an already-exited process.
                        bool hasExited;
                        try { hasExited = WaitForProcess(0); }
                        catch (Exception observationFailure)
                        {
                            throw new AggregateException("prestart-deployment-job:terminate-original",
                                terminateFailure, observationFailure);
                        }
                        if (!hasExited)
                            throw terminateFailure;
                    });

                TryRecover(() =>
                {
                    if (!TerminateJobObject(job, ErrorCancelled))
                        throw DeploymentNativeError("terminate-job");
                    terminationAcceptedTick = clock.ElapsedTicks;
                    terminationRequested = true;
                });
                ReleaseHandle(thread, "close-thread");
                if (processCreated && !exitCode.HasValue)
                    TryRecover(() => ObserveExit(retirementMilliseconds));
                if (exitCode.HasValue)
                    ReleaseHandle(process, "close-process");
                if (!processCreated || process.IsClosed)
                    TryRecover(() => ObserveEmptyJob(retirementMilliseconds));
            }

            private void ObserveExit(int cutoffMilliseconds)
            {
                if (process.IsInvalid || process.IsClosed)
                    throw new InvalidOperationException("prestart-deployment-job:original-process-unavailable");
                while (!WaitForProcess(Math.Min(50U, RemainingWait(cutoffMilliseconds, "exit-deadline"))))
                {
                }
                uint code;
                if (!GetExitCodeProcess(process, out code))
                    throw DeploymentNativeError("read-exit-code");
                exitCode = unchecked((int)code);
            }

            private bool WaitForProcess(uint milliseconds)
            {
                uint result = WaitForSingleObject(process, milliseconds);
                switch (result)
                {
                    case WaitObject0: return true;
                    case WaitTimeout: return false;
                    case WaitFailed: throw DeploymentNativeError("wait-process");
                    default: throw new InvalidOperationException("prestart-deployment-job:wait-result");
                }
            }

            private JobAccounting ReadAccounting()
            {
                JobAccounting accounting;
                if (!QueryInformationJobObject(job, JobObjectBasicAccountingInformation, out accounting,
                    (uint)Marshal.SizeOf(typeof(JobAccounting)), IntPtr.Zero))
                    throw DeploymentNativeError("query-job");
                return accounting;
            }

            private uint[] ReadJobMemberPids()
            {
                // The Live PID list and accounting are separate native observations.
                // This Windows x64 record has two DWORDs before the ULONG_PTR array.
                const int headerBytes = 8;
                const int identifierBytes = 8;
                const int bytes = headerBytes + MaximumRecoveryMembers * identifierBytes;
                IntPtr buffer = Marshal.AllocHGlobal(bytes);
                try
                {
                    if (!QueryJobMembers(job, BasicProcessIdList, buffer, bytes, IntPtr.Zero))
                        throw DeploymentNativeError("query-job-members");
                    int assigned = Marshal.ReadInt32(buffer);
                    int count = Marshal.ReadInt32(buffer, sizeof(uint));
                    if (count < 0 || count != assigned || count > MaximumRecoveryMembers)
                        throw new InvalidDataException("prestart-deployment-job:member-count");
                    uint[] members = new uint[count];
                    for (int index = 0; index < count; index++)
                    {
                        long value = Marshal.ReadInt64(buffer, headerBytes + index * identifierBytes);
                        if (value <= 0 || value > uint.MaxValue)
                            throw new InvalidDataException("prestart-deployment-job:member-pid");
                        members[index] = (uint)value;
                    }
                    return members;
                }
                finally
                {
                    Marshal.FreeHGlobal(buffer);
                }
            }

            private void ObserveEmptyJob(int cutoffMilliseconds)
            {
                while (true)
                {
                    RequireTime(cutoffMilliseconds, "job-deadline");
                    JobAccounting accounting = ReadAccounting();
                    RequireTime(cutoffMilliseconds, "job-deadline");
                    if (accounting.ActiveProcesses == 0)
                    {
                        totalProcesses = accounting.TotalProcesses;
                        limitTerminationsAtEmpty = accounting.TotalTerminatedProcesses;
                        jobEmptyTick = clock.ElapsedTicks;
                        return;
                    }
                    Thread.Sleep((int)Math.Min(5U, RemainingWait(cutoffMilliseconds, "job-deadline")));
                }
            }

            private void RequireTime(int cutoffMilliseconds, string operation)
            {
                RemainingWait(cutoffMilliseconds, operation);
            }

            private uint RemainingWait(int cutoffMilliseconds, string operation)
            {
                double remaining = cutoffMilliseconds - clock.Elapsed.TotalMilliseconds;
                if (remaining <= 0)
                    throw new TimeoutException("prestart-deployment-job:" + operation);
                return (uint)Math.Ceiling(remaining);
            }

            private void ReleaseHandle(SafeHandle handle, string operation)
            {
                TryRecover(() => CloseOwnedHandle(handle, operation));
            }

            private void Release(IDisposable owner)
            {
                if (owner != null)
                    TryRecover(owner.Dispose);
            }

            private void TryRecover(Action operation)
            {
                try { operation(); }
                catch (Exception problem) { recoveryFailures.Add(problem); }
            }
        }

        private static void DuplicateStandardHandle(uint descriptor, KernelHandle target, bool allowDisk,
            string role)
        {
            IntPtr source = GetStdHandle(descriptor);
            if (source == new IntPtr(-1))
                throw DeploymentNativeError("get-" + role);
            if (source == IntPtr.Zero)
                throw new InvalidDataException("prestart-deployment-job:missing-" + role);
            // The dedicated parent supplies pipe ends, never sockets. FILE_TYPE_PIPE
            // is the documented file-type classification for these Windows handles.
            uint type = GetFileType(source);
            if (type != FileTypePipe && !(allowDisk && type == FileTypeDisk))
                throw new InvalidDataException("prestart-deployment-job:invalid-" + role);

            IntPtr current = GetCurrentProcess();
            IntPtr duplicate;
            if (!DuplicateHandle(current, source, current, out duplicate, 0, true, DuplicateSameAccess))
                throw DeploymentNativeError("duplicate-" + role);
            target.Take(duplicate);
            if (target.IsInvalid)
                throw new InvalidOperationException("prestart-deployment-job:duplicate-" + role);
        }

        // SafeHandle.Dispose does not report the BOOL returned by its release
        // callback. The dedicated owner explicitly checks its native closes.
        private static void CloseOwnedHandle(SafeHandle handle, string operation)
        {
            if (handle == null || handle.IsClosed || handle.IsInvalid)
                return;
            if (!CloseDeploymentHandle(handle.DangerousGetHandle()))
                throw DeploymentNativeError(operation);
            handle.SetHandleAsInvalid();
        }

        private static Win32Exception DeploymentNativeError(string operation)
        {
            int code = Marshal.GetLastWin32Error();
            return new Win32Exception(code, "prestart-deployment-job:" + operation);
        }

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern IntPtr GetStdHandle(uint descriptor);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern uint GetFileType(IntPtr handle);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool DuplicateHandle(IntPtr sourceProcess, IntPtr source, IntPtr targetProcess,
            out IntPtr duplicate, uint desiredAccess, bool inheritable, uint options);

        [DllImport("kernel32.dll", EntryPoint = "QueryInformationJobObject", ExactSpelling = true, SetLastError = true)]
        private static extern bool QueryJobMembers(KernelHandle job, int informationClass, IntPtr buffer,
            int bytes, IntPtr returned);

        [DllImport("kernel32.dll", EntryPoint = "QueryInformationJobObject", ExactSpelling = true, SetLastError = true)]
        private static extern bool QueryJobLimits(KernelHandle job, int informationClass,
            out JobExtendedLimits limits, uint bytes, IntPtr returned);

        [DllImport("kernel32.dll", EntryPoint = "CloseHandle", ExactSpelling = true, SetLastError = true)]
        private static extern bool CloseDeploymentHandle(IntPtr handle);
    }
}
