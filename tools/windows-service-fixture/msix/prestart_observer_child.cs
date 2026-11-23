// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Collections.Generic;
using System.ComponentModel;
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
    // The caller owns an exclusive, private staging directory and the capture deadline.
    // Read leases pin the admitted files, and the bookends check path identity. The
    // directory namespace and other loader inputs remain the caller's responsibility.
    // Use this owner on one calling thread.
    public sealed class PrestartObserverChild : IDisposable
    {
        public sealed class ReceiptInputDigests
        {
            public string Before { get; private set; }
            public string After { get; private set; }

            internal ReceiptInputDigests(string before, string after)
            {
                Before = before;
                After = after;
            }
        }

        // The coordinator keeps this original lease through identity collection
        // and retirement. Its facts describe the held bytes, pending independent
        // signature and deployment-window admission.
        public sealed class ArchiveLease : IDisposable
        {
            private readonly PrestartArchiveFacts facts;
            private readonly Stopwatch clock;
            private readonly int timeoutMilliseconds;
            private readonly object gate = new object();
            private HeldFile file;
            private bool retired;
            private bool borrowed;

            private ArchiveLease(HeldFile file, PrestartArchiveFacts facts, Stopwatch clock, int timeoutMilliseconds)
            {
                this.file = file;
                this.facts = facts;
                this.clock = clock;
                this.timeoutMilliseconds = timeoutMilliseconds;
            }

            internal static ArchiveLease Open(string path, long expectedBytes, string expectedSha256,
                Stopwatch clock, int timeoutMilliseconds)
            {
                CheckDeadline(clock, timeoutMilliseconds, "archive");
                CheckFileExpectation(expectedBytes, expectedSha256, PrestartArchiveFacts.MaximumArchiveBytes, "archive");
                RequireWindowsX64();
                CheckLocalPath(path, null);
                if (!path.EndsWith(".msix", StringComparison.OrdinalIgnoreCase))
                    throw new ArgumentException("prestart-observer:archive-path");

                HeldFile file = new HeldFile(path, expectedBytes, expectedSha256);
                try
                {
                    byte[] bytes = file.AdmitArchiveBytes(clock, timeoutMilliseconds);
                    PrestartArchiveFacts facts = PrestartArchiveFacts.Inspect(bytes, expectedSha256);
                    file.ConfirmPath();
                    CheckDeadline(clock, timeoutMilliseconds, "archive");
                    return new ArchiveLease(file, facts, clock, timeoutMilliseconds);
                }
                catch
                {
                    file.Dispose();
                    throw;
                }
            }

            internal PrestartArchiveFacts Confirm(Stopwatch observedClock, int observedTimeoutMilliseconds)
            {
                lock (gate)
                    return ConfirmHeld(observedClock, observedTimeoutMilliseconds);
            }

            private PrestartArchiveFacts ConfirmHeld(Stopwatch observedClock, int observedTimeoutMilliseconds)
            {
                if (retired)
                    throw new InvalidOperationException("prestart-observer:archive-retired");
                if (!ReferenceEquals(clock, observedClock) || timeoutMilliseconds != observedTimeoutMilliseconds)
                    throw new InvalidOperationException("prestart-observer:archive-scope");
                CheckDeadline(clock, timeoutMilliseconds, "archive");
                file.ConfirmPath();
                CheckDeadline(clock, timeoutMilliseconds, "archive");
                return facts;
            }

            internal void BorrowForChild(Stopwatch observedClock, int observedTimeoutMilliseconds)
            {
                lock (gate)
                {
                    ConfirmHeld(observedClock, observedTimeoutMilliseconds);
                    if (borrowed)
                        throw new InvalidOperationException("prestart-observer:archive-busy");
                    borrowed = true;
                }
            }

            internal void ReturnFromChild()
            {
                lock (gate)
                {
                    borrowed = false;
                    if (retired)
                        Close();
                }
            }

            // Withdraw future admission immediately, keeping the native file
            // open for an existing child until its actual retirement.
            public void Dispose()
            {
                lock (gate)
                {
                    retired = true;
                    if (!borrowed)
                        Close();
                }
            }

            private void Close()
            {
                HeldFile owned = file;
                file = null;
                if (owned != null)
                    owned.Dispose();
            }
        }

        private const string ImageName = "nevrotick-prestart-observer.exe";
        private const long MaxImageBytes = 16 * 1024 * 1024;
        private const long MaxIdentityFileBytes = 2 * 1024 * 1024;
        private const long MaxIdentityHelperBytes = 64 * 1024;
        private const string PowerShellName = "powershell.exe";
        private const string IdentityEntryName = "observe_installed_identity.ps1";
        private const string IdentityModuleName = "installed_identity.psm1";
        private const string IdentityHelperName = "prestart_image_path.dll";
        private const int MaxImagePath = 512;
        private const int MaxTimeoutMilliseconds = 120000;
        private const int StartupRetirementMilliseconds = 1000;
        private const string ObserverFlags = " --exact --ignored --nocapture --test-threads=1 --format pretty --color never";

        private const int ErrorInsufficientBuffer = 122;
        private const uint ErrorCancelled = 1223;
        private const ushort ImageFileMachineAmd64 = 0x8664;
        private const uint DriveFixed = 3;
        private const uint GenericRead = 0x80000000;
        private const uint FileShareRead = 1;
        private const uint OpenExisting = 3;
        private const uint HandleFlagInherit = 1;
        private const int FileIdInfo = 18;
        private const int JobObjectBasicAccountingInformation = 1;
        private const int JobObjectExtendedLimitInformation = 9;
        private const uint JobObjectLimitActiveProcess = 0x00000008;
        private const uint JobObjectLimitKillOnJobClose = 0x00002000;
        private const int ProcThreadAttributeHandleList = 0x00020002;
        private const int ProcThreadAttributeJobList = 0x0002000d;
        private const uint StartfUseStdHandles = 0x00000100;
        private const uint CreateSuspended = 0x00000004;
        private const uint DetachedProcess = 0x00000008;
        private const uint CreateUnicodeEnvironment = 0x00000400;
        private const uint ExtendedStartupInfoPresent = 0x00080000;
        private const uint CreateNoWindow = 0x08000000;
        private const uint WaitObject0 = 0;
        private const uint WaitTimeout = 258;
        private const uint WaitFailed = 0xffffffff;

        private readonly HeldFile image;
        private readonly HeldFile[] inputs;
        private readonly HeldFile receiptInput;
        private readonly ArchiveLease archive;
        private readonly PrestartActor actor;
        private readonly Stopwatch clock;
        private readonly int timeoutMilliseconds;
        private readonly KernelHandle process = new KernelHandle();
        private KernelHandle job;
        private PrestartActor.ChildToken childToken;
        private Stream stdout;
        private Stream stderr;
        private int processId;
        private bool processCreated;
        private bool jobMembershipConfirmed;
        private bool archiveBorrowed;
        private bool disposed;

        private PrestartObserverChild(HeldFile image, HeldFile[] inputs, HeldFile receiptInput,
            PrestartActor actor, Stopwatch clock, int timeoutMilliseconds, ArchiveLease archive)
        {
            this.image = image;
            this.inputs = inputs;
            this.receiptInput = receiptInput;
            this.archive = archive;
            this.actor = actor;
            this.clock = clock;
            this.timeoutMilliseconds = timeoutMilliseconds;
        }

        public static PrestartObserverChild Start(
            string imagePath, long expectedBytes, string expectedSha256, string selector,
            PrestartActor actor, Stopwatch clock, int timeoutMilliseconds, string expectedStandardUserSid = null)
        {
            bool requireStandardUser = false;
            switch (selector)
            {
                case "scm_observation_tests::native_installed_roles_prestart_observation":
                case "scm_access_probe::windows::native_installed_roles_prestart_access":
                    break;
                case "scm_access_probe::windows::native_standard_user_prestart_change_config_refused":
                    requireStandardUser = true;
                    break;
                default:
                    throw new ArgumentException("prestart-observer:selector");
            }

            CheckDeadline(clock, timeoutMilliseconds, "launch");
            CheckFileExpectation(expectedBytes, expectedSha256, MaxImageBytes, "image");
            RequireWindowsX64();
            CheckLocalPath(imagePath, ImageName);

            HeldFile image = new HeldFile(imagePath, expectedBytes, expectedSha256);
            return StartFixed(image, new HeldFile[0], image,
                " " + selector + ObserverFlags, "\0", actor, clock, timeoutMilliseconds,
                requireStandardUser: requireStandardUser, expectedStandardUserSid: expectedStandardUserSid);
        }

        public static ArchiveLease OpenArchive(string path, long expectedBytes, string expectedSha256,
            Stopwatch clock, int timeoutMilliseconds)
        {
            return ArchiveLease.Open(path, expectedBytes, expectedSha256, clock, timeoutMilliseconds);
        }

        // The lease stays with the coordinator. The launcher confirms its original
        // file before creating the child and before reporting the completed capture.
        public static PrestartObserverChild StartIdentity(
            string powershellPath, long expectedBytes, string expectedSha256, string inputDirectory,
            long entryBytes, string entrySha256, long moduleBytes, string moduleSha256,
            long helperBytes, string helperSha256, Guid runId, ArchiveLease archive, string subjectSid,
            string expectedArchiveSha256, PrestartActor actor, Stopwatch clock, int timeoutMilliseconds)
        {
            CheckDeadline(clock, timeoutMilliseconds, "launch");
            CheckFileExpectation(expectedBytes, expectedSha256, MaxIdentityFileBytes, "image");
            CheckFileExpectation(entryBytes, entrySha256, MaxIdentityFileBytes, "identity-entry");
            CheckFileExpectation(moduleBytes, moduleSha256, MaxIdentityFileBytes, "identity-module");
            CheckFileExpectation(helperBytes, helperSha256, MaxIdentityHelperBytes, "identity-helper");
            PrestartArchiveFacts archiveFacts;
            try
            {
                if (archive == null || !IsDigest(expectedArchiveSha256))
                    throw new InvalidDataException("prestart-observer:identity-archive-digest");
                archiveFacts = archive.Confirm(clock, timeoutMilliseconds);
                if (!string.Equals(archiveFacts.ArchiveSha256, expectedArchiveSha256, StringComparison.Ordinal))
                    throw new InvalidDataException("prestart-observer:identity-archive-digest");
            }
            catch (Exception failure)
            {
                // No launch input, job or process has been acquired here.
                failure.Data["PrestartObserverJobCreated"] = false;
                failure.Data["PrestartObserverProcessCreated"] = false;
                failure.Data["PrestartObserverRetired"] = true;
                failure.Data["PrestartObserverReleased"] = true;
                throw;
            }
            if (runId == Guid.Empty)
                throw new ArgumentException("prestart-observer:identity-run");
            if (subjectSid == null || subjectSid.Length > 168 ||
                !Regex.IsMatch(subjectSid, @"\AS-1-[0-9]{1,10}(?:-[0-9]{1,10}){1,14}\z"))
                throw new ArgumentException("prestart-observer:identity-subject-sid");

            RequireWindowsX64();
            string systemDirectory = Environment.SystemDirectory;
            string powershellDirectory = systemDirectory + @"\WindowsPowerShell\v1.0";
            string systemPowerShell = powershellDirectory + "\\" + PowerShellName;
            if (!string.Equals(powershellPath, systemPowerShell, StringComparison.Ordinal))
                throw new ArgumentException("prestart-observer:identity-powershell-path");
            CheckLocalPath(powershellPath, PowerShellName);
            CheckLocalPath(inputDirectory, null);

            string entryPath = inputDirectory + "\\" + IdentityEntryName;
            string modulePath = inputDirectory + "\\" + IdentityModuleName;
            string helperPath = inputDirectory + "\\" + IdentityHelperName;
            CheckLocalPath(entryPath, IdentityEntryName);
            CheckLocalPath(modulePath, IdentityModuleName);
            CheckLocalPath(helperPath, IdentityHelperName);

            string arguments = " -NoLogo -NoProfile -NonInteractive -ExecutionPolicy RemoteSigned -File \"" +
                entryPath + "\" -RunId " + runId.ToString("D") + " -PackageFullName " + archiveFacts.PackageFullName +
                " -ExpectedUserSid " + subjectSid + " -SignedPackageSha256 " + archiveFacts.ArchiveSha256 +
                " -EngineBytes " + archiveFacts.EngineBytes.ToString(CultureInfo.InvariantCulture) +
                " -EngineSha256 " + archiveFacts.EngineSha256 +
                " -BrokerBytes " + archiveFacts.BrokerBytes.ToString(CultureInfo.InvariantCulture) +
                " -BrokerSha256 " + archiveFacts.BrokerSha256;
            string windowsDirectory = Path.GetDirectoryName(systemDirectory);
            string environmentBlock = "PSModuleAnalysisCachePath=NUL\0PSModulePath=" +
                powershellDirectory + "\\Modules\0" +
                "SystemRoot=" + windowsDirectory + "\0windir=" + windowsDirectory + "\0";
            HeldFile module = new HeldFile(modulePath, moduleBytes, moduleSha256);
            HeldFile[] inputs =
            {
                new HeldFile(entryPath, entryBytes, entrySha256),
                module,
                new HeldFile(helperPath, helperBytes, helperSha256)
            };
            return StartFixed(new HeldFile(powershellPath, expectedBytes, expectedSha256, resolveSystemSpelling: true), inputs,
                module, arguments, environmentBlock, actor, clock, timeoutMilliseconds, subjectSid, archive: archive);
        }

        private static PrestartObserverChild StartFixed(HeldFile image, HeldFile[] inputs, HeldFile receiptInput,
            string arguments, string environmentBlock, PrestartActor actor, Stopwatch clock, int timeoutMilliseconds,
            string identitySubjectSid = null, bool requireStandardUser = false, string expectedStandardUserSid = null,
            ArchiveLease archive = null)
        {
            if (actor == null)
                throw new ArgumentNullException("actor");
            actor.VerifyCurrent(clock, timeoutMilliseconds);

            var child = new PrestartObserverChild(image, inputs, receiptInput, actor, clock, timeoutMilliseconds, archive);
            try
            {
                image.Admit(clock, timeoutMilliseconds);
                foreach (HeldFile input in inputs)
                    input.Admit(clock, timeoutMilliseconds);
                // Preserve input admission failures before checking the deployment subject.
                // A mismatched actor must never reach native job or process creation.
                if (identitySubjectSid != null)
                    actor.VerifyCurrent(clock, timeoutMilliseconds, identitySubjectSid);
                if (requireStandardUser)
                    actor.VerifyStandardUser(clock, timeoutMilliseconds, expectedStandardUserSid);
                if (archive != null)
                {
                    archive.BorrowForChild(clock, timeoutMilliseconds);
                    child.archiveBorrowed = true;
                }
                child.CreateChild(arguments, environmentBlock);
                return child;
            }
            catch (Exception primary)
            {
                child.RetireFailedLaunch(primary);
                throw;
            }
        }

        public int ProcessId
        {
            get { RequireOpen(); return processId; }
        }

        public Stream Stdout
        {
            get { RequireOpen(); return stdout; }
        }

        public Stream Stderr
        {
            get { RequireOpen(); return stderr; }
        }

        public bool HasExited
        {
            get
            {
                RequireOpen();
                uint result = WaitForSingleObject(process, 0);
                switch (result)
                {
                    case WaitObject0: return true;
                    case WaitTimeout: return false;
                    case WaitFailed: throw NativeError("wait-process");
                    default: throw new InvalidOperationException("prestart-observer:wait-result");
                }
            }
        }

        public int ExitCode
        {
            get
            {
                if (!HasExited)
                    throw new InvalidOperationException("prestart-observer:process-running");
                uint code;
                if (!GetExitCodeProcess(process, out code))
                    throw NativeError("read-exit-code");
                return unchecked((int)code);
            }
        }

        public bool IsJobEmpty
        {
            get
            {
                RequireOpen();
                JobAccounting accounting;
                if (!QueryInformationJobObject(job, JobObjectBasicAccountingInformation, out accounting,
                    (uint)Marshal.SizeOf(typeof(JobAccounting)), IntPtr.Zero))
                    throw NativeError("query-job");
                return accounting.ActiveProcesses == 0;
            }
        }

        // The caller waits for actual process/reader completion after requesting termination.
        public void Terminate()
        {
            RequireOpen();
            Exception processFailure = null;
            try
            {
                // An original whose membership was never confirmed could not
                // have been resumed. Keep recovery available on its held handle.
                if (processCreated && !jobMembershipConfirmed && !process.IsInvalid && !HasExited &&
                    !TerminateProcess(process, ErrorCancelled))
                    throw NativeError("terminate-startup-process");
            }
            catch (Exception failure) { processFailure = failure; }
            if (!TerminateJobObject(job, ErrorCancelled))
            {
                Win32Exception jobFailure = NativeError("terminate-job");
                if (processFailure != null)
                    throw new AggregateException("prestart-observer:terminate", processFailure, jobFailure);
                throw jobFailure;
            }
            if (processFailure != null)
                throw processFailure;
        }

        public ReceiptInputDigests ConfirmImage()
        {
            CheckDeadline(clock, timeoutMilliseconds, "collection");
            if (!HasExited || !IsJobEmpty)
                throw new InvalidOperationException("prestart-observer:job-running");

            try
            {
                childToken.VerifyUnchanged(clock, timeoutMilliseconds);
                actor.VerifyCurrent(clock, timeoutMilliseconds);
            }
            catch (TimeoutException failure)
            {
                throw new TimeoutException("prestart-child:collection-deadline", failure);
            }
            image.Confirm(clock, timeoutMilliseconds);
            foreach (HeldFile input in inputs)
                input.Confirm(clock, timeoutMilliseconds);
            if (archive != null)
                archive.Confirm(clock, timeoutMilliseconds);
            CheckDeadline(clock, timeoutMilliseconds, "collection");
            return new ReceiptInputDigests(receiptInput.AdmittedSha256, receiptInput.ConfirmedSha256);
        }

        private void RetireFailedLaunch(Exception primary)
        {
            primary.Data["PrestartObserverJobCreated"] = job != null && !job.IsInvalid;
            primary.Data["PrestartObserverProcessCreated"] = processCreated;
            if (processCreated)
            {
                if (processId > 0)
                    primary.Data["PrestartObserverProcessId"] = processId;

                try { Terminate(); }
                catch (Exception failure) { primary.Data["PrestartObserverTerminationFailure"] = failure; }
                try
                {
                    // Retiring an owned child remains necessary after the capture expires.
                    Stopwatch retirement = Stopwatch.StartNew();
                    while (!HasExited || !IsJobEmpty)
                    {
                        if (retirement.ElapsedMilliseconds >= StartupRetirementMilliseconds)
                            throw new TimeoutException("prestart-observer:startup-retirement");
                        Thread.Sleep(5);
                    }
                }
                catch (Exception failure)
                {
                    primary.Data["PrestartObserverRetirementFailure"] = failure;
                    primary.Data["PrestartObserverRetired"] = false;
                    primary.Data["PrestartObserverReleased"] = false;
                    primary.Data["PrestartObserverChild"] = this;
                    return;
                }
            }
            primary.Data["PrestartObserverRetired"] = true;
            try
            {
                Dispose();
                primary.Data["PrestartObserverReleased"] = true;
            }
            catch (Exception failure)
            {
                primary.Data["PrestartObserverReleased"] = false;
                primary.Data["PrestartObserverDisposalFailure"] = failure;
            }
        }

        public void Dispose()
        {
            if (disposed)
                return;
            if (archiveBorrowed && processCreated && (!HasExited || !IsJobEmpty))
                throw new InvalidOperationException("prestart-observer:archive-child-running");
            disposed = true;

            // The job is the fail-safe if the caller arrives here without normal retirement.
            // Still release every owned resource if a managed disposal throws.
            List<Exception> failures = null;
            var owners = new List<IDisposable> { job, stdout, stderr, childToken, process, image };
            owners.AddRange(inputs);
            foreach (IDisposable owner in owners)
            {
                if (owner == null)
                    continue;
                try { owner.Dispose(); }
                catch (Exception failure)
                {
                    if (failures == null)
                        failures = new List<Exception>();
                    failures.Add(failure);
                }
            }
            if (archiveBorrowed)
            {
                archiveBorrowed = false;
                try { archive.ReturnFromChild(); }
                catch (Exception failure)
                {
                    if (failures == null)
                        failures = new List<Exception>();
                    failures.Add(failure);
                }
            }
            if (failures != null)
                throw new AggregateException("prestart-observer:dispose", failures);
        }

        private void RequireOpen()
        {
            if (disposed)
                throw new ObjectDisposedException("PrestartObserverChild");
        }

        private static void CheckDeadline(Stopwatch clock, int timeoutMilliseconds, string phase)
        {
            if (clock == null || !clock.IsRunning || timeoutMilliseconds <= 0 ||
                timeoutMilliseconds > MaxTimeoutMilliseconds)
                throw new ArgumentException("prestart-observer:" + phase + "-clock");
            if (clock.ElapsedMilliseconds >= timeoutMilliseconds)
            {
                if (phase == "archive")
                    throw new TimeoutException("prestart-observer:archive-deadline");
                throw new TimeoutException(phase == "collection" ?
                    "prestart-child:collection-deadline" : "prestart-observer:launch-deadline");
            }
        }

        private static void RequireWindowsX64()
        {
            if (Environment.OSVersion.Platform != PlatformID.Win32NT || !Environment.Is64BitProcess)
                throw new PlatformNotSupportedException("prestart-observer:windows-x64");
            ushort processMachine;
            ushort nativeMachine;
            if (!IsWow64Process2(GetCurrentProcess(), out processMachine, out nativeMachine))
                throw NativeError("query-machine");
            if (processMachine != 0 || nativeMachine != ImageFileMachineAmd64)
                throw new PlatformNotSupportedException("prestart-observer:windows-x64");
        }

        private static bool IsDigest(string digest)
        {
            return digest != null && digest.Length == 64 && Regex.IsMatch(digest, @"\A[0-9a-f]{64}\z");
        }

        private static void CheckFileExpectation(long bytes, string digest, long maximum, string input)
        {
            if (bytes <= 0 || bytes > maximum)
                throw new ArgumentException("prestart-observer:" + input + "-length");
            if (!IsDigest(digest))
                throw new ArgumentException("prestart-observer:" + input + "-sha256");
        }

        private static void CheckLocalPath(string path, string fileName)
        {
            if (path == null || path.Length > MaxImagePath ||
                !Regex.IsMatch(path, @"\A[A-Za-z]:\\[^\x00-\x1f""*/?<>|:]+\z"))
                throw new ArgumentException("prestart-observer:image-path");

            string[] components = path.Substring(3).Split('\\');
            if (fileName != null && (components.Length < 2 || components[components.Length - 1] != fileName))
                throw new ArgumentException("prestart-observer:image-path");
            foreach (string component in components)
            {
                if (component.Length == 0 || component.Length > 255 ||
                    component.EndsWith(".", StringComparison.Ordinal) ||
                    component.EndsWith(" ", StringComparison.Ordinal))
                    throw new ArgumentException("prestart-observer:image-path");
            }
            if (GetDriveTypeW(path.Substring(0, 3)) != DriveFixed)
                throw new ArgumentException("prestart-observer:image-drive");
        }

        private static FileStream OpenImage(string path)
        {
            SafeFileHandle handle = CreateFileW(path, GenericRead, FileShareRead, IntPtr.Zero,
                OpenExisting, 0, IntPtr.Zero);
            if (handle.IsInvalid)
            {
                Win32Exception failure = NativeError("open-image");
                handle.Dispose();
                throw failure;
            }
            try { return new FileStream(handle, FileAccess.Read, 1, false); }
            catch { handle.Dispose(); throw; }
        }

        private static FileIdentity ReadImageIdentity(SafeFileHandle handle, string path)
        {
            string normalizedPath;
            return ReadImageIdentity(handle, path, StringComparison.Ordinal, out normalizedPath);
        }

        private static FileIdentity ReadImageIdentity(SafeFileHandle handle, string path,
            StringComparison comparison, out string normalizedPath)
        {
            var observed = new StringBuilder(MaxImagePath + 5);
            uint count = GetFinalPathNameByHandleW(handle, observed, (uint)observed.Capacity, 0);
            if (count == 0)
                throw NativeError("resolve-image-path");
            string finalPath = observed.ToString();
            if (count >= observed.Capacity || count != observed.Length ||
                !string.Equals(finalPath, @"\\?\" + path, comparison))
                throw new InvalidDataException("prestart-observer:image-final-path");

            FileIdentity identity;
            if (!GetFileInformationByHandleEx(handle, FileIdInfo, out identity,
                (uint)Marshal.SizeOf(typeof(FileIdentity))))
                throw NativeError("query-image-identity");
            normalizedPath = finalPath.Substring(4);
            return identity;
        }

        private sealed class HeldFile : IDisposable
        {
            internal string Path { get; private set; }
            internal string AdmittedSha256 { get; private set; }
            internal string ConfirmedSha256 { get; private set; }
            private readonly long expectedBytes;
            private readonly string expectedSha256;
            private readonly bool resolveSystemSpelling;
            private FileStream stream;
            private FileIdentity admittedIdentity;

            internal HeldFile(string path, long bytes, string digest, bool resolveSystemSpelling = false)
            {
                Path = path;
                expectedBytes = bytes;
                expectedSha256 = digest;
                this.resolveSystemSpelling = resolveSystemSpelling;
            }

            internal void Admit(Stopwatch clock, int timeoutMilliseconds)
            {
                Open(clock, timeoutMilliseconds, "launch");
                AdmittedSha256 = CheckBytes(clock, timeoutMilliseconds, "launch");
            }

            private void Open(Stopwatch clock, int timeoutMilliseconds, string phase)
            {
                CheckDeadline(clock, timeoutMilliseconds, phase);
                stream = OpenImage(Path);
                // SystemDirectory can spell "system32" differently from the
                // held OS file. Only the fixed interpreter gets this conversion.
                string normalizedPath;
                StringComparison comparison = resolveSystemSpelling ?
                    StringComparison.OrdinalIgnoreCase : StringComparison.Ordinal;
                admittedIdentity = ReadImageIdentity(stream.SafeFileHandle, Path, comparison, out normalizedPath);
                Path = normalizedPath;
            }

            internal byte[] AdmitArchiveBytes(Stopwatch clock, int timeoutMilliseconds)
            {
                Open(clock, timeoutMilliseconds, "archive");
                if (stream.Length != expectedBytes)
                    throw new InvalidDataException("prestart-archive:archive-size");
                byte[] bytes = new byte[checked((int)expectedBytes)];
                int offset = 0;
                while (offset < bytes.Length)
                {
                    CheckDeadline(clock, timeoutMilliseconds, "archive");
                    int count = stream.Read(bytes, offset, Math.Min(bytes.Length - offset, 64 * 1024));
                    if (count == 0)
                        throw new InvalidDataException("prestart-archive:archive-size");
                    offset += count;
                }
                if (stream.ReadByte() != -1 || stream.Length != expectedBytes)
                    throw new InvalidDataException("prestart-archive:archive-size");
                CheckDeadline(clock, timeoutMilliseconds, "archive");
                return bytes;
            }

            internal void ConfirmPath()
            {
                using (FileStream current = OpenImage(Path))
                {
                    FileIdentity currentIdentity = ReadImageIdentity(current.SafeFileHandle, Path);
                    FileIdentity heldIdentity = ReadImageIdentity(stream.SafeFileHandle, Path);
                    if (!admittedIdentity.Matches(currentIdentity) || !admittedIdentity.Matches(heldIdentity))
                        throw new InvalidDataException("prestart-observer:image-identity");
                }
            }

            internal void Confirm(Stopwatch clock, int timeoutMilliseconds)
            {
                string digest = CheckBytes(clock, timeoutMilliseconds, "collection");
                ConfirmPath();
                ConfirmedSha256 = digest;
            }

            public void Dispose()
            {
                if (stream != null)
                    stream.Dispose();
            }

            private string CheckBytes(Stopwatch clock, int timeoutMilliseconds, string phase)
            {
                CheckDeadline(clock, timeoutMilliseconds, phase);
                if (stream.Length != expectedBytes)
                    throw new InvalidDataException("prestart-observer:image-length");

                // Read at most the admitted extent and one EOF probe even if the file changes.
                stream.Position = 0;
                byte[] buffer = new byte[64 * 1024];
                using (SHA256 hash = new SHA256CryptoServiceProvider())
                {
                    long remaining = expectedBytes;
                    while (remaining > 0)
                    {
                        CheckDeadline(clock, timeoutMilliseconds, phase);
                        int count = stream.Read(buffer, 0, (int)Math.Min(remaining, buffer.Length));
                        if (count == 0)
                            throw new InvalidDataException("prestart-observer:image-length");
                        hash.TransformBlock(buffer, 0, count, buffer, 0);
                        remaining -= count;
                    }
                    CheckDeadline(clock, timeoutMilliseconds, phase);
                    if (stream.ReadByte() != -1 || stream.Length != expectedBytes)
                        throw new InvalidDataException("prestart-observer:image-length");
                    hash.TransformFinalBlock(buffer, 0, 0);
                    CheckDeadline(clock, timeoutMilliseconds, phase);
                    string digest = BitConverter.ToString(hash.Hash).Replace("-", "").ToLowerInvariant();
                    if (!string.Equals(digest, expectedSha256, StringComparison.Ordinal))
                        throw new InvalidDataException("prestart-observer:image-digest");
                    return digest;
                }
            }
        }

        private void CreateChild(string arguments, string environmentBlock)
        {
            job = CreateJobObjectW(IntPtr.Zero, null);
            if (job.IsInvalid)
                throw NativeError("create-job");
            var limits = new JobExtendedLimits();
            limits.Basic.LimitFlags = JobObjectLimitActiveProcess | JobObjectLimitKillOnJobClose;
            limits.Basic.ActiveProcessLimit = 1;
            if (!SetInformationJobObject(job, JobObjectExtendedLimitInformation, ref limits,
                (uint)Marshal.SizeOf(typeof(JobExtendedLimits))))
                throw NativeError("configure-job");

            using (var input = new PipeEnds())
            using (var output = new PipeEnds())
            using (var error = new PipeEnds())
            using (var thread = new KernelHandle())
            {
                input.Write.Dispose();
                MakeInheritable(input.Read);
                MakeInheritable(output.Write);
                MakeInheritable(error.Write);
                stdout = output.TakeReader();
                stderr = error.TakeReader();

                Launch(input.Read, output.Write, error.Write, thread, arguments, environmentBlock);
                input.Read.Dispose();
                output.Write.Dispose();
                error.Write.Dispose();

                ConfirmSuspendedProcess();
                // Keep the PowerShell image and its three named inputs at their admitted paths
                // when the suspended child becomes runnable.
                if (inputs.Length != 0)
                {
                    CheckDeadline(clock, timeoutMilliseconds, "launch");
                    ushort processMachine;
                    ushort nativeMachine;
                    if (!IsWow64Process2(process, out processMachine, out nativeMachine))
                        throw NativeError("query-child-machine");
                    if (processMachine != 0 || nativeMachine != ImageFileMachineAmd64)
                        throw new InvalidDataException("prestart-observer:child-machine");
                    CheckDeadline(clock, timeoutMilliseconds, "launch");
                    image.ConfirmPath();
                    foreach (HeldFile admittedInput in inputs)
                    {
                        CheckDeadline(clock, timeoutMilliseconds, "launch");
                        admittedInput.ConfirmPath();
                    }
                }
                childToken = actor.VerifyChild(process, clock, timeoutMilliseconds);
                actor.VerifyCurrent(clock, timeoutMilliseconds);
                CheckDeadline(clock, timeoutMilliseconds, "launch");
                uint suspendCount = ResumeThread(thread);
                if (suspendCount == uint.MaxValue)
                    throw NativeError("resume-thread");
                if (suspendCount != 1)
                    throw new InvalidOperationException("prestart-observer:thread-suspend-count");
            }
        }

        private void ConfirmSuspendedProcess()
        {
            bool contained;
            if (!IsProcessInJob(process, job, out contained))
                throw NativeError("query-process-job");
            if (!contained)
                throw new InvalidOperationException("prestart-observer:process-job");
            jobMembershipConfirmed = true;

            var observed = new StringBuilder(MaxImagePath + 1);
            uint count = (uint)observed.Capacity;
            if (!QueryFullProcessImageNameW(process, 0, observed, ref count))
                throw NativeError("query-process-image");
            if (count != observed.Length ||
                !string.Equals(observed.ToString(), image.Path, StringComparison.Ordinal))
                throw new InvalidDataException("prestart-observer:process-image");
        }

        private static void MakeInheritable(SafeFileHandle handle)
        {
            if (!SetHandleInformation(handle, HandleFlagInherit, HandleFlagInherit))
                throw NativeError("set-pipe-inheritance");
        }

        private void Launch(SafeFileHandle input, SafeFileHandle output, SafeFileHandle error,
            KernelHandle thread, string arguments, string environmentBlock)
        {
            UIntPtr listBytes = UIntPtr.Zero;
            bool sized = InitializeProcThreadAttributeList(IntPtr.Zero, 2, 0, ref listBytes);
            int sizeError = Marshal.GetLastWin32Error();
            if (sized || sizeError != ErrorInsufficientBuffer)
                throw new Win32Exception(sizeError, "prestart-observer:size-attributes");
            if (listBytes.ToUInt64() == 0 || listBytes.ToUInt64() > 65536)
                throw new InvalidOperationException("prestart-observer:attribute-size");

            IntPtr attributeList = IntPtr.Zero;
            IntPtr handleValues = IntPtr.Zero;
            IntPtr environment = IntPtr.Zero;
            bool initialized = false;
            try
            {
                attributeList = Marshal.AllocHGlobal((int)listBytes.ToUInt64());
                handleValues = Marshal.AllocHGlobal(IntPtr.Size * 4);
                // Each fixed block ends in NUL. The marshaler adds the final UTF-16 NUL.
                environment = Marshal.StringToHGlobalUni(environmentBlock);
                if (!InitializeProcThreadAttributeList(attributeList, 2, 0, ref listBytes))
                    throw NativeError("initialize-attributes");
                initialized = true;

                var startup = new StartupInfoEx();
                startup.Info.Size = (uint)Marshal.SizeOf(typeof(StartupInfoEx));
                startup.Info.Flags = StartfUseStdHandles;
                startup.Info.StdIn = input.DangerousGetHandle();
                startup.Info.StdOut = output.DangerousGetHandle();
                startup.Info.StdErr = error.DangerousGetHandle();
                startup.AttributeList = attributeList;

                Marshal.WriteIntPtr(handleValues, 0, startup.Info.StdIn);
                Marshal.WriteIntPtr(handleValues, IntPtr.Size, startup.Info.StdOut);
                Marshal.WriteIntPtr(handleValues, IntPtr.Size * 2, startup.Info.StdErr);
                IntPtr jobValue = IntPtr.Add(handleValues, IntPtr.Size * 3);
                Marshal.WriteIntPtr(jobValue, job.DangerousGetHandle());
                SetAttribute(attributeList, ProcThreadAttributeHandleList, handleValues, IntPtr.Size * 3);
                SetAttribute(attributeList, ProcThreadAttributeJobList, jobValue, IntPtr.Size);

                var commandLine = new StringBuilder("\"" + image.Path + "\"" + arguments);
                string directory = image.Path.Substring(0, image.Path.LastIndexOf('\\'));
                uint consoleMode = inputs.Length == 0 ? DetachedProcess : CreateNoWindow;
                uint flags = ExtendedStartupInfoPresent | CreateUnicodeEnvironment | consoleMode | CreateSuspended;
                ProcessInformation created;
                actor.VerifyCurrent(clock, timeoutMilliseconds);
                CheckDeadline(clock, timeoutMilliseconds, "launch");
                bool launched = CreateProcessW(image.Path, commandLine, IntPtr.Zero, IntPtr.Zero, true,
                    flags, environment, directory, ref startup, out created);
                if (!launched)
                    throw NativeError("create-process");
                processCreated = true;
                process.Take(created.Process);
                thread.Take(created.Thread);
                if (process.IsInvalid || thread.IsInvalid || created.ProcessId == 0 ||
                    created.ProcessId > int.MaxValue)
                    throw new InvalidOperationException("prestart-observer:process-identity");
                processId = (int)created.ProcessId;
            }
            finally
            {
                if (initialized)
                    DeleteProcThreadAttributeList(attributeList);
                Marshal.FreeHGlobal(environment);
                Marshal.FreeHGlobal(handleValues);
                Marshal.FreeHGlobal(attributeList);

                // These handles must outlive the raw values stored in the attribute list.
                GC.KeepAlive(input);
                GC.KeepAlive(output);
                GC.KeepAlive(error);
                GC.KeepAlive(job);
            }
        }

        private static void SetAttribute(IntPtr list, int attribute, IntPtr value, int bytes)
        {
            if (!UpdateProcThreadAttribute(list, 0, new UIntPtr((uint)attribute), value,
                new UIntPtr((uint)bytes), IntPtr.Zero, IntPtr.Zero))
                throw NativeError("set-attributes");
        }

        private static Win32Exception NativeError(string stage)
        {
            int error = Marshal.GetLastWin32Error();
            return new Win32Exception(error, "prestart-observer:" + stage);
        }

        private sealed class PipeEnds : IDisposable
        {
            internal SafeFileHandle Read;
            internal readonly SafeFileHandle Write;

            internal PipeEnds()
            {
                IntPtr read;
                IntPtr write;
                // The native outputs have defined ownership only after success.
                if (!CreatePipe(out read, out write, IntPtr.Zero, 0))
                    throw NativeError("create-pipe");
                try
                {
                    Read = new SafeFileHandle(read, true);
                    Write = new SafeFileHandle(write, true);
                }
                catch
                {
                    if (Read != null)
                        Read.Dispose();
                    else
                        CloseHandle(read);
                    CloseHandle(write);
                    throw;
                }
            }

            internal Stream TakeReader()
            {
                var stream = new FileStream(Read, FileAccess.Read, 1, false);
                Read = null;
                return stream;
            }

            public void Dispose()
            {
                Write.Dispose();
                if (Read != null)
                    Read.Dispose();
            }
        }

        private sealed class KernelHandle : SafeHandleZeroOrMinusOneIsInvalid
        {
            public KernelHandle() : base(true) { }
            internal void Take(IntPtr value) { SetHandle(value); }
            protected override bool ReleaseHandle() { return CloseHandle(handle); }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct StartupInfo
        {
            internal uint Size;
            internal IntPtr Reserved;
            internal IntPtr Desktop;
            internal IntPtr Title;
            internal uint X;
            internal uint Y;
            internal uint XSize;
            internal uint YSize;
            internal uint XCountChars;
            internal uint YCountChars;
            internal uint FillAttribute;
            internal uint Flags;
            internal ushort ShowWindow;
            internal ushort ReservedSize;
            internal IntPtr ReservedBytes;
            internal IntPtr StdIn;
            internal IntPtr StdOut;
            internal IntPtr StdErr;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct StartupInfoEx
        {
            internal StartupInfo Info;
            internal IntPtr AttributeList;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct ProcessInformation
        {
            internal IntPtr Process;
            internal IntPtr Thread;
            internal uint ProcessId;
            internal uint ThreadId;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct JobBasicLimits
        {
            internal long PerProcessUserTimeLimit;
            internal long PerJobUserTimeLimit;
            internal uint LimitFlags;
            internal UIntPtr MinimumWorkingSetSize;
            internal UIntPtr MaximumWorkingSetSize;
            internal uint ActiveProcessLimit;
            internal UIntPtr Affinity;
            internal uint PriorityClass;
            internal uint SchedulingClass;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct IoCounters
        {
            internal ulong ReadOperationCount;
            internal ulong WriteOperationCount;
            internal ulong OtherOperationCount;
            internal ulong ReadTransferCount;
            internal ulong WriteTransferCount;
            internal ulong OtherTransferCount;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct JobExtendedLimits
        {
            internal JobBasicLimits Basic;
            internal IoCounters Io;
            internal UIntPtr ProcessMemoryLimit;
            internal UIntPtr JobMemoryLimit;
            internal UIntPtr PeakProcessMemoryUsed;
            internal UIntPtr PeakJobMemoryUsed;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct JobAccounting
        {
            internal long TotalUserTime;
            internal long TotalKernelTime;
            internal long ThisPeriodTotalUserTime;
            internal long ThisPeriodTotalKernelTime;
            internal uint TotalPageFaultCount;
            internal uint TotalProcesses;
            internal uint ActiveProcesses;
            internal uint TotalTerminatedProcesses;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct FileIdentity
        {
            internal ulong VolumeSerialNumber;
            internal ulong FileIdLow;
            internal ulong FileIdHigh;

            internal bool Matches(FileIdentity other)
            {
                return VolumeSerialNumber == other.VolumeSerialNumber &&
                    FileIdLow == other.FileIdLow && FileIdHigh == other.FileIdHigh;
            }
        }

        [DllImport("kernel32.dll", ExactSpelling = true)]
        private static extern IntPtr GetCurrentProcess();

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool IsWow64Process2(IntPtr process, out ushort processMachine, out ushort nativeMachine);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool IsWow64Process2(KernelHandle process, out ushort processMachine, out ushort nativeMachine);

        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
        private static extern uint GetDriveTypeW(string rootPath);

        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern SafeFileHandle CreateFileW(string fileName, uint desiredAccess, uint shareMode,
            IntPtr security, uint creationDisposition, uint flagsAndAttributes, IntPtr templateFile);

        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern uint GetFinalPathNameByHandleW(SafeFileHandle file, StringBuilder path,
            uint pathCharacters, uint flags);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool GetFileInformationByHandleEx(SafeFileHandle file, int informationClass,
            out FileIdentity information, uint informationBytes);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool CreatePipe(out IntPtr read, out IntPtr write,
            IntPtr security, uint bufferBytes);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool SetHandleInformation(SafeFileHandle handle, uint mask, uint flags);

        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern KernelHandle CreateJobObjectW(IntPtr security, string name);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool SetInformationJobObject(KernelHandle job, int informationClass,
            ref JobExtendedLimits information, uint informationBytes);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool QueryInformationJobObject(KernelHandle job, int informationClass,
            out JobAccounting information, uint informationBytes, IntPtr returnBytes);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool IsProcessInJob(KernelHandle process, KernelHandle job, out bool contained);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool TerminateProcess(KernelHandle process, uint exitCode);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool TerminateJobObject(KernelHandle job, uint exitCode);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool InitializeProcThreadAttributeList(IntPtr attributes, int count,
            uint flags, ref UIntPtr bytes);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool UpdateProcThreadAttribute(IntPtr attributes, uint flags,
            UIntPtr attribute, IntPtr value, UIntPtr valueBytes, IntPtr previousValue, IntPtr returnBytes);

        [DllImport("kernel32.dll", ExactSpelling = true)]
        private static extern void DeleteProcThreadAttributeList(IntPtr attributes);

        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern bool CreateProcessW(string application, StringBuilder commandLine,
            IntPtr processSecurity, IntPtr threadSecurity, bool inheritHandles, uint flags,
            IntPtr environment, string currentDirectory, ref StartupInfoEx startup, out ProcessInformation process);

        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern bool QueryFullProcessImageNameW(KernelHandle process, uint flags,
            StringBuilder path, ref uint pathCharacters);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern uint ResumeThread(KernelHandle thread);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern uint WaitForSingleObject(KernelHandle handle, uint milliseconds);

        [DllImport("kernel32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool GetExitCodeProcess(KernelHandle process, out uint exitCode);

        [DllImport("kernel32.dll", ExactSpelling = true)]
        private static extern bool CloseHandle(IntPtr handle);
    }
}
