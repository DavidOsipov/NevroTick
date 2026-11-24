// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using Microsoft.Win32.SafeHandles;

namespace NevroTick
{
    // One native owner for the original watchdog and its inherited writers.
    // The access probe still owns all protocol bytes on stdin and stdout.
    internal static class PrestartAccessParent
    {
        private static int Main(string[] args)
        {
            Stopwatch clock = Stopwatch.StartNew();
            try
            {
                PrestartObserverChild.ObserveWatchdogAccessSettlement(args, clock);
                return 0;
            }
            catch (Exception)
            {
                // An absent receipt and this independent nonzero process exit
                // also cover an unusable or partially written status pipe.
                return 1;
            }
        }
    }

    public sealed partial class PrestartObserverChild
    {
        private const string AccessWatchdogName = "prestart_deployment_watchdog.exe";
        private const string AccessParentName = "prestart_access_parent.exe";
        private const string AccessParentSupervisionMode = "--supervise-access-parent";
        private const string AccessDiagnosticName = "prestart-access-diagnostics.log";
        private const string AccessSettlementMode = "--observe-access-settlement";
        private const string AccessExitVersion = "nevrotick-prestart-access-exit-v1";
        private const int AccessDiagnosticLimit = 32 * 1024;
        // Keep the forwarded record within the host reader's single-line bound.
        private const int AccessStatusRecordLimit = 271;
        private const int AccessStatusPipeClosed = 109;
        private const uint AccessWatchdogProcessLimit = 1 + DeploymentGuardianActiveProcessLimit;
        private const uint AccessParentProcessLimit = 1 + AccessWatchdogProcessLimit;
        private const uint AccessGenericWrite = 0x40000000;
        private const uint AccessFileWriteData = 0x0002;
        private const uint AccessSynchronize = 0x00100000;
        private const int AccessFileStandardInfo = 1;
        private const int AccessFileAttributeTagInfo = 9;

        internal static void ObserveWatchdogAccessSettlement(string[] args, Stopwatch clock)
        {
            RequireWindowsX64();
            AccessWatchdog operation = new AccessWatchdog(args, clock);
            operation.Run();
        }

        private sealed class AccessWatchdog
        {
            private const int Forwarded = 6;
            private readonly string[] args;
            private readonly Stopwatch clock;
            private readonly Guid run;
            private readonly int work;
            private readonly int cooperativeEnd;
            private readonly int retirement;
            private readonly bool supervisesParent;
            private readonly string diagnosticPath;
            private readonly HeldFile image;
            private readonly KernelHandle process = new KernelHandle();
            private readonly KernelHandle thread = new KernelHandle();
            private readonly KernelHandle input = new KernelHandle();
            private readonly KernelHandle output = new KernelHandle();
            private readonly KernelHandle error = new KernelHandle();
            private KernelHandle job;
            private SafeFileHandle diagnostic;
            private FileIdentity diagnosticIdentity;
            private PrestartActor actor;
            private PrestartActor.ChildToken childToken;
            private uint pid;
            private bool processCreated;
            private bool contained;
            private bool jobEmpty;

            internal AccessWatchdog(string[] arguments, Stopwatch clock)
            {
                const int innerLength = Forwarded + 13;
                const int supervisionFields = 4;
                supervisesParent = arguments.Length == innerLength + supervisionFields &&
                    arguments[0] == AccessParentSupervisionMode;
                if (supervisesParent)
                {
                    args = new string[innerLength];
                    Array.Copy(arguments, supervisionFields, args, 0, args.Length);
                }
                else
                {
                    args = arguments;
                }
                this.clock = clock;
                int guardianWork;
                int guardianRetirement;
                if (args.Length != innerLength ||
                    (args[3] != "access" && args[3] != "standard") ||
                    args[4].Length != 32 || !Regex.IsMatch(args[4], @"\A[0-9a-f]{32}\z") ||
                    !Guid.TryParseExact(args[Forwarded + 8], "D", out run) || run == Guid.Empty ||
                    !String.Equals(run.ToString("D"), args[Forwarded + 8], StringComparison.Ordinal) ||
                    !AccessBudget(args[Forwarded + 9], args[Forwarded + 10], out guardianWork, out guardianRetirement) ||
                    !AccessBudget(args[Forwarded + 11], args[Forwarded + 12], out work, out retirement) ||
                    retirement - work < (supervisesParent ? 4 : 2))
                    throw new ArgumentException("prestart-access-parent:input");

                // Leave part of the existing native allowance for the parent to
                // terminate and observe the job if the watchdog stops responding.
                cooperativeEnd = work + (retirement - work) / 2;
                CheckLocalPath(args[0], AccessWatchdogName);
                diagnosticPath = args[5];
                CheckLocalPath(diagnosticPath, AccessDiagnosticName);
                CheckLocalPath(args[Forwarded], DeploymentGuardianName);
                CheckLocalPath(args[Forwarded + 5], DeploymentEntryName);
                CheckFileExpectation(AccessLength(args[Forwarded + 1]), args[Forwarded + 2], MaxIdentityFileBytes,
                    "deployment-guardian");
                CheckFileExpectation(AccessLength(args[Forwarded + 3]), args[Forwarded + 4], MaxIdentityFileBytes,
                    "deployment-interpreter");
                CheckFileExpectation(AccessLength(args[Forwarded + 6]), args[Forwarded + 7], MaxIdentityFileBytes,
                    "deployment-owner");
                long imageBytes = AccessLength(args[1]);
                CheckFileExpectation(imageBytes, args[2], MaxImageBytes, "access-watchdog");
                if (supervisesParent)
                {
                    string parentPath = arguments[1];
                    CheckLocalPath(parentPath, AccessParentName);
                    long parentBytes = AccessLength(arguments[2]);
                    CheckFileExpectation(parentBytes, arguments[3], MaxImageBytes, "access-parent");
                    image = new HeldFile(parentPath, parentBytes, arguments[3]);
                }
                else
                {
                    image = new HeldFile(args[0], imageBytes, args[2]);
                }
            }

            internal void Run()
            {
                // Only this process owns the outward status writer. The outer
                // owner receives the inner parent's record through a private pipe.
                IntPtr status = GetStdHandle(StdErrorHandle);
                if (GetFileType(status) != FileTypePipe || status == GetStdHandle(StdInputHandle) ||
                    status == GetStdHandle(StdOutputHandle))
                    throw new InvalidDataException("prestart-access-parent:status-pipe");

                byte[] bytes = null;
                using (image)
                using (actor = PrestartActor.CaptureCurrent(clock, work))
                using (process)
                using (thread)
                using (input)
                using (output)
                using (error)
                using (PipeEnds privateStatus = supervisesParent ? new PipeEnds() : null)
                {
                    try
                    {
                        image.Admit(clock, work);
                        if (supervisesParent)
                        {
                            ConfirmSupervisorImage();
                            PreparePrivateStatusWriter(privateStatus);
                        }
                        else
                        {
                            OpenDiagnostic();
                        }
                        DuplicateStandardHandle(StdInputHandle, input, false, "stdin");
                        DuplicateStandardHandle(StdOutputHandle, output, false, "stdout");
                        PrepareJob();
                        Launch();
                        CloseOwnedHandle(input, "close-access-stdin");
                        CloseOwnedHandle(output, "close-access-stdout");
                        CloseOwnedHandle(error, "close-access-stderr");
                        using (childToken = ConfirmSuspended())
                        {
                            CheckDeadline(clock, work, "access-resume");
                            uint suspendCount = ResumeThread(thread);
                            if (suspendCount == uint.MaxValue)
                                throw DeploymentNativeError("resume-access-watchdog");
                            if (suspendCount != 1)
                                throw new InvalidDataException("prestart-access-parent:suspend-count");
                            CloseOwnedHandle(thread, "close-access-thread");

                            // The raw status is meaningful only after a signalled wait
                            // on the original process. Even an exit of 259 is then a DWORD.
                            WaitForWatchdog(cooperativeEnd);
                            uint exit = ReadWatchdogExit();
                            ObserveEmptyJob(cooperativeEnd);
                            if (supervisesParent && exit != 0)
                                throw new InvalidDataException("prestart-access-parent:inner-exit");
                            if (!supervisesParent && exit == 0)
                                CheckDeadline(clock, work, "access-work");
                            childToken.VerifyUnchanged(clock, retirement);
                            actor.VerifyCurrent(clock, retirement);
                            image.Confirm(clock, retirement);

                            if (!supervisesParent)
                            {
                                int size;
                                string digest = ReadDiagnostic(out size);
                                string line = String.Format(CultureInfo.InvariantCulture,
                                    "{0} {1:D} {2} {3} {4} {5} {6} {7} {8}\n", AccessExitVersion, run, args[3],
                                    args[4], image.ConfirmedSha256, pid, exit, size, digest);
                                bytes = Encoding.ASCII.GetBytes(line);
                            }
                        }
                        CloseRetiredHandles();
                        if (supervisesParent)
                            bytes = ReadRetiredParentStatus(privateStatus);
                        CheckDeadline(clock, retirement, "access-release");
                    }
                    catch (Exception primary)
                    {
                        try { Recover(); }
                        catch (Exception recovery)
                        {
                            throw new AggregateException("prestart-access-parent:recovery", primary, recovery);
                        }
                        throw;
                    }
                    finally
                    {
                        // If checked retirement failed, kill-on-close remains the
                        // fallback. The parent withholds the receipt in either case.
                        if (job != null)
                            job.Dispose();
                        if (diagnostic != null)
                            diagnostic.Dispose();
                    }
                }

                CheckDeadline(clock, retirement, "access-publication");
                uint written;
                if (!WriteAccessStatus(status, bytes, (uint)bytes.Length, out written, IntPtr.Zero) ||
                    written != bytes.Length)
                    throw new IOException("prestart-access-parent:status-write");
                CheckDeadline(clock, retirement, "access-publication");
            }

            private void PreparePrivateStatusWriter(PipeEnds status)
            {
                IntPtr current = GetCurrentProcess();
                IntPtr duplicate;
                if (!DuplicateHandle(current, status.Write.DangerousGetHandle(), current, out duplicate,
                    0, true, DuplicateSameAccess))
                    throw DeploymentNativeError("duplicate-access-status");
                error.Take(duplicate);
                CloseOwnedHandle(status.Write, "close-access-private-status-writer");
            }

            private byte[] ReadRetiredParentStatus(PipeEnds status)
            {
                // Only the inner parent inherited this writer. We read it
                // after the original process and every job member have exited.
                using (MemoryStream bytes = new MemoryStream(AccessStatusRecordLimit))
                {
                    byte[] chunk = new byte[AccessStatusRecordLimit + 1];
                    while (true)
                    {
                        CheckDeadline(clock, retirement, "access-parent-status");
                        uint limit = checked((uint)(chunk.Length - bytes.Length));
                        uint read;
                        if (!ReadAccessBytes(status.Read, chunk, limit, out read, IntPtr.Zero))
                        {
                            int errorCode = Marshal.GetLastWin32Error();
                            if (errorCode != AccessStatusPipeClosed)
                                throw new System.ComponentModel.Win32Exception(errorCode, "prestart-access-parent:read-status");
                            break;
                        }
                        if (read == 0 || read > limit || bytes.Length + read > AccessStatusRecordLimit)
                            throw new InvalidDataException("prestart-access-parent:status-size");
                        bytes.Write(chunk, 0, checked((int)read));
                    }
                    if (bytes.Length == 0)
                        throw new InvalidDataException("prestart-access-parent:status-missing");
                    CloseOwnedHandle(status.Read, "close-access-private-status-reader");
                    return bytes.ToArray();
                }
            }

            private void ConfirmSupervisorImage()
            {
                StringBuilder observed = new StringBuilder(MaxImagePath + 1);
                uint count = (uint)observed.Capacity;
                if (!QueryAccessSupervisorImage(GetCurrentProcess(), 0, observed, ref count))
                    throw DeploymentNativeError("access-supervisor-image");
                if (count != observed.Length || !String.Equals(observed.ToString(), image.Path, StringComparison.Ordinal))
                    throw new InvalidDataException("prestart-access-parent:supervisor-image");
                image.ConfirmPath();
            }

            private void OpenDiagnostic()
            {
                CheckDeadline(clock, work, "access-diagnostic");
                // Keep the original read/write lock through readback. It excludes
                // foreign writers and rename/delete while allowing private readers.
                diagnostic = CreateFileW(diagnosticPath, GenericRead | AccessGenericWrite, FileShareRead, IntPtr.Zero,
                    OpenExisting, OpenReparsePoint, IntPtr.Zero);
                if (diagnostic.IsInvalid)
                    throw DeploymentNativeError("open-access-diagnostic");
                IntPtr source = diagnostic.DangerousGetHandle();
                if (GetFileType(source) != FileTypeDisk)
                    throw new InvalidDataException("prestart-access-parent:diagnostic-file");
                diagnosticIdentity = ReadImageIdentity(diagnostic, diagnosticPath);
                if (CheckedDiagnosticSize() != 0)
                    throw new InvalidDataException("prestart-access-parent:diagnostic-empty");

                IntPtr duplicate;
                IntPtr current = GetCurrentProcess();
                const uint writer = AccessFileWriteData | FileReadAttributes | AccessSynchronize;
                if (!DuplicateHandle(current, source, current, out duplicate, writer, true, 0))
                    throw DeploymentNativeError("duplicate-access-diagnostic");
                error.Take(duplicate);
                if (error.IsInvalid || GetFileType(error.DangerousGetHandle()) != FileTypeDisk)
                    throw new InvalidDataException("prestart-access-parent:diagnostic-writer");
            }

            private void PrepareJob()
            {
                CheckDeadline(clock, work, "access-job");
                job = CreateJobObjectW(IntPtr.Zero, null);
                if (job.IsInvalid)
                    throw DeploymentNativeError("create-access-job");
                const uint flags = JobObjectLimitActiveProcess | JobObjectLimitKillOnJobClose;
                uint processLimit = supervisesParent ? AccessParentProcessLimit : AccessWatchdogProcessLimit;
                JobExtendedLimits limits = new JobExtendedLimits();
                limits.Basic.LimitFlags = flags;
                limits.Basic.ActiveProcessLimit = processLimit;
                uint size = (uint)Marshal.SizeOf(typeof(JobExtendedLimits));
                if (!SetInformationJobObject(job, JobObjectExtendedLimitInformation, ref limits, size))
                    throw DeploymentNativeError("configure-access-job");
                if (!QueryJobLimits(job, JobObjectExtendedLimitInformation, out limits, size, IntPtr.Zero))
                    throw DeploymentNativeError("query-access-job-limits");
                if (limits.Basic.LimitFlags != flags || limits.Basic.ActiveProcessLimit != processLimit)
                    throw new InvalidDataException("prestart-access-parent:job-limits");
            }

            private void Launch()
            {
                UIntPtr listBytes = UIntPtr.Zero;
                bool sized = InitializeProcThreadAttributeList(IntPtr.Zero, 2, 0, ref listBytes);
                int sizeError = Marshal.GetLastWin32Error();
                if (sized || sizeError != ErrorInsufficientBuffer)
                    throw new System.ComponentModel.Win32Exception(sizeError, "prestart-access-parent:size-attributes");
                if (listBytes.ToUInt64() == 0 || listBytes.ToUInt64() > 65536)
                    throw new InvalidDataException("prestart-access-parent:attribute-size");

                IntPtr attributes = IntPtr.Zero;
                IntPtr handles = IntPtr.Zero;
                IntPtr environment = IntPtr.Zero;
                bool initialized = false;
                try
                {
                    attributes = Marshal.AllocHGlobal((int)listBytes.ToUInt64());
                    handles = Marshal.AllocHGlobal(IntPtr.Size * 4);
                    string windows = Path.GetDirectoryName(Environment.SystemDirectory);
                    environment = Marshal.StringToHGlobalUni("SystemRoot=" + windows + "\0windir=" + windows + "\0");
                    if (!InitializeProcThreadAttributeList(attributes, 2, 0, ref listBytes))
                        throw DeploymentNativeError("initialize-access-attributes");
                    initialized = true;

                    StartupInfoEx startup = new StartupInfoEx();
                    startup.Info.Size = (uint)Marshal.SizeOf(typeof(StartupInfoEx));
                    startup.Info.Flags = StartfUseStdHandles;
                    startup.Info.StdIn = input.DangerousGetHandle();
                    startup.Info.StdOut = output.DangerousGetHandle();
                    startup.Info.StdErr = error.DangerousGetHandle();
                    startup.AttributeList = attributes;
                    Marshal.WriteIntPtr(handles, 0, startup.Info.StdIn);
                    Marshal.WriteIntPtr(handles, IntPtr.Size, startup.Info.StdOut);
                    Marshal.WriteIntPtr(handles, IntPtr.Size * 2, startup.Info.StdErr);
                    IntPtr jobValue = IntPtr.Add(handles, IntPtr.Size * 3);
                    Marshal.WriteIntPtr(jobValue, job.DangerousGetHandle());
                    SetAttribute(attributes, ProcThreadAttributeHandleList, handles, IntPtr.Size * 3);
                    SetAttribute(attributes, ProcThreadAttributeJobList, jobValue, IntPtr.Size);

                    // All forwarded paths excluded quotes and trailing separators
                    // at admission. All other fields are canonical identifiers,
                    // bounded digit strings or lowercase hex.
                    StringBuilder command = new StringBuilder();
                    command.Append('"').Append(image.Path).Append('"');
                    int firstArgument = supervisesParent ? 0 : Forwarded;
                    for (int index = firstArgument; index < Forwarded + 11; index++)
                        command.Append(" \"").Append(args[index]).Append('"');
                    actor.VerifyCurrent(clock, work);
                    int elapsed = checked((int)Math.Ceiling(clock.Elapsed.TotalMilliseconds));
                    if (elapsed >= work)
                        throw new TimeoutException("prestart-access-parent:handoff-deadline");
                    command.Append(' ').Append((work - elapsed).ToString(CultureInfo.InvariantCulture))
                        .Append(' ').Append((cooperativeEnd - elapsed).ToString(CultureInfo.InvariantCulture));
                    if (!supervisesParent)
                        command.Append(' ').Append(AccessSettlementMode);

                    uint flags = ExtendedStartupInfoPresent | CreateUnicodeEnvironment | CreateNoWindow | CreateSuspended;
                    ProcessInformation created;
                    if (!CreateProcessW(image.Path, command, IntPtr.Zero, IntPtr.Zero, true, flags, environment,
                        Path.GetDirectoryName(image.Path), ref startup, out created))
                        throw DeploymentNativeError("create-access-watchdog");
                    processCreated = true;
                    process.Take(created.Process);
                    thread.Take(created.Thread);
                    pid = created.ProcessId;
                    if (process.IsInvalid || thread.IsInvalid || pid == 0)
                        throw new InvalidDataException("prestart-access-parent:process-identity");
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

            private PrestartActor.ChildToken ConfirmSuspended()
            {
                CheckDeadline(clock, work, "access-watchdog");
                bool member;
                if (!IsProcessInJob(process, job, out member))
                    throw DeploymentNativeError("access-watchdog-membership");
                if (!member)
                    throw new InvalidDataException("prestart-access-parent:job-membership");
                contained = true;
                StringBuilder observed = new StringBuilder(MaxImagePath + 1);
                uint count = (uint)observed.Capacity;
                if (!QueryFullProcessImageNameW(process, 0, observed, ref count))
                    throw DeploymentNativeError("access-watchdog-image");
                if (count != observed.Length || !String.Equals(observed.ToString(), image.Path, StringComparison.Ordinal))
                    throw new InvalidDataException("prestart-access-parent:process-image");
                ushort processMachine;
                ushort nativeMachine;
                if (!IsWow64Process2(process, out processMachine, out nativeMachine))
                    throw DeploymentNativeError("access-watchdog-machine");
                if (processMachine != 0 || nativeMachine != ImageFileMachineAmd64)
                    throw new InvalidDataException("prestart-access-parent:process-machine");
                image.ConfirmPath();
                return actor.VerifyChild(process, clock, work);
            }

            private void WaitForWatchdog(int cutoff)
            {
                while (true)
                {
                    int remaining = checked((int)Math.Ceiling(CheckDeadline(clock, cutoff, "access-watchdog-exit").TotalMilliseconds));
                    uint result = WaitForSingleObject(process, (uint)Math.Min(remaining, 50));
                    if (result == WaitObject0)
                    {
                        CheckDeadline(clock, cutoff, "access-watchdog-exit");
                        return;
                    }
                    if (result == WaitFailed)
                        throw DeploymentNativeError("wait-access-watchdog");
                    if (result != WaitTimeout)
                        throw new InvalidDataException("prestart-access-parent:wait-result");
                }
            }

            private uint ReadWatchdogExit()
            {
                uint exit;
                if (!GetExitCodeProcess(process, out exit))
                    throw DeploymentNativeError("access-watchdog-exit");
                return exit;
            }

            private void ObserveEmptyJob(int cutoff)
            {
                while (true)
                {
                    CheckDeadline(clock, cutoff, "access-job-empty");
                    JobAccounting accounting;
                    if (!QueryInformationJobObject(job, JobObjectBasicAccountingInformation, out accounting,
                        (uint)Marshal.SizeOf(typeof(JobAccounting)), IntPtr.Zero))
                        throw DeploymentNativeError("account-access-job");
                    CheckDeadline(clock, cutoff, "access-job-empty");
                    if (accounting.ActiveProcesses == 0 && accounting.TotalProcesses != 0)
                    {
                        jobEmpty = true;
                        return;
                    }
                    Thread.Sleep(1);
                }
            }

            private void Recover()
            {
                if (!processCreated || jobEmpty)
                    return;
                List<Exception> failures = new List<Exception>();
                // An unconfirmed original was never resumed. Target that handle
                // separately as well as requesting termination of the original job.
                if (!contained && !process.IsInvalid && !TerminateProcess(process, ErrorCancelled))
                    failures.Add(DeploymentNativeError("terminate-unconfirmed-access-watchdog"));
                if (!TerminateJobObject(job, ErrorCancelled))
                    failures.Add(DeploymentNativeError("terminate-access-job"));
                // A refused request is distinct from the observed outcome. The
                // original process may have exited while the request was made.
                try
                {
                    WaitForWatchdog(retirement);
                    ReadWatchdogExit();
                    ObserveEmptyJob(retirement);
                    CloseRetiredHandles();
                    CheckDeadline(clock, retirement, "access-recovery-release");
                }
                catch (Exception observation)
                {
                    failures.Add(observation);
                }
                if (failures.Count != 0)
                    throw new AggregateException("prestart-access-parent:retirement", failures);
            }

            private void CloseRetiredHandles()
            {
                CloseOwnedHandle(process, "close-access-process");
                CloseOwnedHandle(diagnostic, "close-access-diagnostic");
                CloseOwnedHandle(job, "close-access-job");
            }

            private int CheckedDiagnosticSize()
            {
                DiagnosticAttributeTag attributes;
                if (!GetAccessDiagnosticAttributes(diagnostic, AccessFileAttributeTagInfo, out attributes,
                    (uint)Marshal.SizeOf(typeof(DiagnosticAttributeTag))))
                    throw DeploymentNativeError("attributes-access-diagnostic");
                DiagnosticStandardInfo standard;
                if (!GetAccessDiagnosticStandardInfo(diagnostic, AccessFileStandardInfo, out standard,
                    (uint)Marshal.SizeOf(typeof(DiagnosticStandardInfo))))
                    throw DeploymentNativeError("info-access-diagnostic");
                if ((attributes.Attributes & (FileAttributes.Directory | FileAttributes.ReparsePoint |
                    FileAttributes.Device)) != 0 || standard.Directory != 0 || standard.DeletePending != 0 ||
                    standard.NumberOfLinks != 1)
                    throw new InvalidDataException("prestart-access-parent:diagnostic-file");
                if (standard.EndOfFile < 0 || standard.EndOfFile > AccessDiagnosticLimit)
                    throw new InvalidDataException("prestart-access-parent:diagnostic-size");
                return checked((int)standard.EndOfFile);
            }

            private string ReadDiagnostic(out int size)
            {
                CheckDeadline(clock, retirement, "access-diagnostic");
                size = CheckedDiagnosticSize();
                long offset;
                if (!SeekAccessDiagnostic(diagnostic, 0, out offset, 0) || offset != 0)
                    throw DeploymentNativeError("seek-access-diagnostic");
                byte[] bytes = new byte[size];
                if (size != 0)
                {
                    uint read;
                    if (!ReadAccessBytes(diagnostic, bytes, (uint)size, out read, IntPtr.Zero))
                        throw DeploymentNativeError("read-access-diagnostic");
                    if (read != size)
                        throw new InvalidDataException("prestart-access-parent:diagnostic-read");
                }
                if (CheckedDiagnosticSize() != size ||
                    !diagnosticIdentity.Matches(ReadImageIdentity(diagnostic, diagnosticPath)))
                    throw new InvalidDataException("prestart-access-parent:diagnostic-identity");
                CheckDeadline(clock, retirement, "access-diagnostic");
                using (SHA256 hash = new SHA256CryptoServiceProvider())
                    return BitConverter.ToString(hash.ComputeHash(bytes)).Replace("-", String.Empty).ToLowerInvariant();
            }

            private static bool AccessBudget(string work, string retirement, out int workMs, out int retirementMs)
            {
                workMs = 0;
                retirementMs = 0;
                return work.Length <= 6 && retirement.Length <= 6 &&
                    Int32.TryParse(work, NumberStyles.None, CultureInfo.InvariantCulture, out workMs) &&
                    Int32.TryParse(retirement, NumberStyles.None, CultureInfo.InvariantCulture, out retirementMs) &&
                    DeploymentDeadlinesValid(workMs, retirementMs);
            }

            private static long AccessLength(string value)
            {
                long length;
                if (value.Length > 8 || !Int64.TryParse(value, NumberStyles.None, CultureInfo.InvariantCulture, out length))
                    throw new ArgumentException("prestart-access-parent:length");
                return length;
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct DiagnosticAttributeTag
        {
            internal FileAttributes Attributes;
            internal uint ReparseTag;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct DiagnosticStandardInfo
        {
            internal long AllocationSize;
            internal long EndOfFile;
            internal uint NumberOfLinks;
            internal byte DeletePending;
            internal byte Directory;
        }

        [DllImport("kernel32.dll", EntryPoint = "GetFileInformationByHandleEx", ExactSpelling = true, SetLastError = true)]
        private static extern bool GetAccessDiagnosticAttributes(SafeFileHandle file, int informationClass,
            out DiagnosticAttributeTag information, uint informationBytes);

        [DllImport("kernel32.dll", EntryPoint = "GetFileInformationByHandleEx", ExactSpelling = true, SetLastError = true)]
        private static extern bool GetAccessDiagnosticStandardInfo(SafeFileHandle file, int informationClass,
            out DiagnosticStandardInfo information, uint informationBytes);

        [DllImport("kernel32.dll", EntryPoint = "SetFilePointerEx", ExactSpelling = true, SetLastError = true)]
        private static extern bool SeekAccessDiagnostic(SafeFileHandle file, long distance, out long position, uint origin);

        [DllImport("kernel32.dll", EntryPoint = "ReadFile", ExactSpelling = true, SetLastError = true)]
        private static extern bool ReadAccessBytes(SafeFileHandle file, [Out] byte[] buffer, uint count,
            out uint read, IntPtr overlapped);

        [DllImport("kernel32.dll", EntryPoint = "WriteFile", ExactSpelling = true, SetLastError = true)]
        private static extern bool WriteAccessStatus(IntPtr status, byte[] buffer, uint count,
            out uint written, IntPtr overlapped);

        [DllImport("kernel32.dll", EntryPoint = "QueryFullProcessImageNameW", ExactSpelling = true, SetLastError = true,
            CharSet = CharSet.Unicode)]
        private static extern bool QueryAccessSupervisorImage(IntPtr process, uint flags, StringBuilder path,
            ref uint size);
    }
}
