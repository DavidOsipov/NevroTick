// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.ComponentModel;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Security.Principal;
using Microsoft.Win32.SafeHandles;

namespace NevroTick
{
    // A query-only observation of this launcher's primary token on this boot.
    // The window owner determines which observed actor may serve each role.
    public sealed class PrestartActor : IDisposable
    {
        private const uint TokenQuery = 0x0008;
        private const int ErrorInsufficientBuffer = 122;
        private const int ErrorNoToken = 1008;
        private const int MaxTimeoutMilliseconds = 120000;
        private const int MaxInformationBytes = 256 * 1024;
        private const int PrimaryToken = 1;
        private const uint DefaultElevationType = 1;
        private const int SidHeaderBytes = 8;
        private const int MaxSidSubAuthorities = 15;
        private const string AdministratorsSid = "S-1-5-32-544";
        private const string CallerActor = "prestart-observer:caller-actor";
        private const string ChildActor = "prestart-observer:child-actor";
        private static readonly int StatisticsBytes = Marshal.SizeOf(typeof(TokenStatistics));
        private static readonly int SidEntryBytes = Marshal.SizeOf(typeof(NativeSidEntry));
        private static readonly int GroupEntryOffset = Marshal.OffsetOf(typeof(NativeGroups), "First").ToInt32();
        private static readonly int PrivilegeEntryBytes = Marshal.SizeOf(typeof(PrivilegeEntry));

        private readonly SafeFileHandle token;
        private readonly Snapshot snapshot;

        private PrestartActor(SafeFileHandle token, Snapshot snapshot)
        {
            this.token = token;
            this.snapshot = snapshot;
        }

        public static PrestartActor CaptureCurrent(Stopwatch clock, int timeoutMilliseconds)
        {
            CheckDeadline(clock, timeoutMilliseconds);
            if (Environment.OSVersion.Platform != PlatformID.Win32NT || !Environment.Is64BitProcess)
                throw new PlatformNotSupportedException("prestart-observer:windows-x64");
            CheckDeadline(clock, timeoutMilliseconds);
            RequireProcessActor();

            SafeFileHandle current = OpenCurrent(clock, timeoutMilliseconds);
            try
            {
                Snapshot observed = ReadSnapshot(current, CallerActor, clock, timeoutMilliseconds);
                var actor = new PrestartActor(current, observed);
                actor.VerifyCurrent(clock, timeoutMilliseconds);
                return actor;
            }
            catch
            {
                current.Dispose();
                throw;
            }
        }

        internal static void RequireProcessActor()
        {
            // CreateProcessW selects the process primary token. Refuse a thread token.
            SafeFileHandle threadToken;
            if (OpenThreadToken(GetCurrentThread(), 0, true, out threadToken))
            {
                using (threadToken)
                    throw new InvalidOperationException("prestart-observer:thread-token");
            }
            int error = Marshal.GetLastWin32Error();
            if (threadToken != null)
                threadToken.Dispose();
            if (error != ErrorNoToken)
                throw new Win32Exception(error, "prestart-observer:query-thread-token");
        }

        internal void VerifyCurrent(Stopwatch clock, int timeoutMilliseconds, string requiredSubjectSid = null)
        {
            if (token.IsClosed)
                throw new ObjectDisposedException("PrestartActor");
            CheckDeadline(clock, timeoutMilliseconds);
            RequireProcessActor();
            using (SafeFileHandle current = OpenCurrent(clock, timeoutMilliseconds))
            {
                TokenStatistics statistics = ReadStatistics(current, clock, timeoutMilliseconds);
                if (!snapshot.Statistics.SameVersion(statistics))
                    throw new InvalidOperationException(CallerActor);
            }
            CheckDeadline(clock, timeoutMilliseconds);
            RequireProcessActor();
            CheckDeadline(clock, timeoutMilliseconds);
            if (requiredSubjectSid != null && !snapshot.User.HasExactIdentity(requiredSubjectSid))
                throw new InvalidOperationException("prestart-observer:identity-subject-actor");
        }

        // The window owner supplies the subject of the proposed provisioning context.
        internal void VerifyProvisioningUser(Stopwatch clock, int timeoutMilliseconds, string expectedSubjectSid)
        {
            VerifyCurrent(clock, timeoutMilliseconds);
            if (!snapshot.User.HasExactIdentity(expectedSubjectSid))
                throw new InvalidOperationException("prestart-observer:provisioning-subject-actor");
        }

        // Classify the diagnostic role against the independently admitted subject.
        internal void VerifyStandardUser(Stopwatch clock, int timeoutMilliseconds, string expectedSubjectSid)
        {
            VerifyCurrent(clock, timeoutMilliseconds);
            // Exact equality with the observed SID also requires its canonical spelling.
            if (!snapshot.User.HasExactIdentity(expectedSubjectSid))
                throw new InvalidOperationException("prestart-observer:standard-user-subject-actor");

            // Membership includes disabled and deny-only groups.
            bool includesAdministrators = Array.Exists(snapshot.Groups,
                (SidEntry group) => group.HasExactIdentity(AdministratorsSid));
            CheckDeadline(clock, timeoutMilliseconds);
            if (includesAdministrators || snapshot.ElevationType != DefaultElevationType || snapshot.Elevated)
                throw new InvalidOperationException("prestart-observer:standard-user-token-profile");
        }

        internal ChildToken VerifyChild(SafeHandle process, Stopwatch clock, int timeoutMilliseconds)
        {
            if (process == null || process.IsInvalid || process.IsClosed)
                throw new InvalidOperationException(ChildActor);
            VerifyCurrent(clock, timeoutMilliseconds);
            SafeFileHandle child = OpenChild(process, clock, timeoutMilliseconds);
            try
            {
                Snapshot observed = ReadSnapshot(child, ChildActor, clock, timeoutMilliseconds);
                if (!snapshot.SameContext(observed))
                    throw new InvalidOperationException(ChildActor);

                // Re-open from the original suspended process. The previous token
                // handle can remain valid even if the process primary has changed.
                using (SafeFileHandle current = OpenChild(process, clock, timeoutMilliseconds))
                {
                    TokenStatistics statistics = ReadStatistics(current, clock, timeoutMilliseconds);
                    if (!observed.Statistics.SameVersion(statistics))
                        throw new InvalidOperationException(ChildActor);
                }
                VerifyCurrent(clock, timeoutMilliseconds);
                return new ChildToken(child, observed.Statistics.TokenId.Value, observed.Statistics.ModifiedId.Value);
            }
            catch
            {
                child.Dispose();
                throw;
            }
        }

        public void Dispose()
        {
            token.Dispose();
        }

        private static void CheckDeadline(Stopwatch clock, int timeoutMilliseconds)
        {
            if (clock == null || !clock.IsRunning || timeoutMilliseconds <= 0 ||
                timeoutMilliseconds > MaxTimeoutMilliseconds)
                throw new ArgumentException("prestart-observer:launch-clock");
            if (clock.ElapsedMilliseconds >= timeoutMilliseconds)
                throw new TimeoutException("prestart-observer:launch-deadline");
        }

        private static SafeFileHandle OpenCurrent(Stopwatch clock, int timeoutMilliseconds)
        {
            CheckDeadline(clock, timeoutMilliseconds);
            SafeFileHandle result;
            bool opened = OpenProcessToken(GetCurrentProcess(), TokenQuery, out result);
            int error = opened ? 0 : Marshal.GetLastWin32Error();
            return RequireOpenedToken(opened, result, "open-caller-token", error);
        }

        private static SafeFileHandle OpenChild(SafeHandle process, Stopwatch clock, int timeoutMilliseconds)
        {
            CheckDeadline(clock, timeoutMilliseconds);
            SafeFileHandle result;
            bool opened = OpenProcessToken(process, TokenQuery, out result);
            int error = opened ? 0 : Marshal.GetLastWin32Error();
            return RequireOpenedToken(opened, result, "open-child-token", error);
        }

        private static SafeFileHandle RequireOpenedToken(bool opened, SafeFileHandle token, string operation,
            int error)
        {
            if (opened && token != null && !token.IsInvalid && !token.IsClosed)
                return token;
            if (token != null)
                token.Dispose();
            if (!opened)
                throw new Win32Exception(error, "prestart-observer:" + operation);
            throw InvalidInformation();
        }

        private static TokenStatistics ReadStatistics(SafeFileHandle token, Stopwatch clock, int timeoutMilliseconds)
        {
            return ReadInformation(token, InformationClass.Statistics, StatisticsBytes, ReadStatisticsValue,
                clock, timeoutMilliseconds);
        }

        private static Snapshot ReadSnapshot(SafeFileHandle token, string actorError, Stopwatch clock,
            int timeoutMilliseconds)
        {
            TokenStatistics before = ReadStatistics(token, clock, timeoutMilliseconds);
            if (before.Type != PrimaryToken)
                throw new InvalidOperationException(actorError);

            var result = new Snapshot
            {
                Statistics = before,
                User = ReadInformation(token, InformationClass.User, 0, ReadSingleSid, clock, timeoutMilliseconds),
                Groups = ReadInformation(token, InformationClass.Groups, 0, ReadGroups, clock, timeoutMilliseconds),
                RestrictingSids = ReadInformation(token, InformationClass.RestrictedSids, 0, ReadGroups,
                    clock, timeoutMilliseconds),
                Privileges = ReadInformation(token, InformationClass.Privileges, 0, ReadPrivileges,
                    clock, timeoutMilliseconds),
                Session = ReadDword(token, InformationClass.SessionId, clock, timeoutMilliseconds),
                ElevationType = ReadDword(token, InformationClass.ElevationType, clock, timeoutMilliseconds),
                Elevated = ReadFlag(token, InformationClass.Elevation, clock, timeoutMilliseconds),
                WasFiltered = ReadInformation(token, InformationClass.HasRestrictions, sizeof(uint),
                    ReadFilteredValue, clock, timeoutMilliseconds),
                Integrity = ReadInformation(token, InformationClass.IntegrityLevel, 0, ReadSingleSid,
                    clock, timeoutMilliseconds),
                IsAppContainer = ReadFlag(token, InformationClass.IsAppContainer, clock, timeoutMilliseconds)
            };

            if (!before.SameVersion(ReadStatistics(token, clock, timeoutMilliseconds)))
                throw new InvalidOperationException(actorError);
            if (result.ElevationType < 1 || result.ElevationType > 3 ||
                (uint)result.Groups.Length != before.GroupCount ||
                (uint)result.Privileges.Length != before.PrivilegeCount)
                throw InvalidInformation();
            return result;
        }

        private static uint ReadDword(SafeFileHandle token, InformationClass informationClass, Stopwatch clock,
            int timeoutMilliseconds)
        {
            return ReadInformation(token, informationClass, sizeof(uint), ReadDwordValue, clock, timeoutMilliseconds);
        }

        private static bool ReadFlag(SafeFileHandle token, InformationClass informationClass, Stopwatch clock,
            int timeoutMilliseconds)
        {
            // Win32 reports these fields as zero or nonzero DWORD values.
            return ReadDword(token, informationClass, clock, timeoutMilliseconds) != 0;
        }

        private static T ReadInformation<T>(SafeFileHandle token, InformationClass informationClass, int fixedBytes,
            Func<IntPtr, int, T> read, Stopwatch clock, int timeoutMilliseconds)
        {
            uint capacity = (uint)fixedBytes;
            if (fixedBytes == 0)
            {
                CheckDeadline(clock, timeoutMilliseconds);
                if (GetTokenInformation(token, informationClass, IntPtr.Zero, 0, out capacity))
                    throw InvalidInformation();
                int error = Marshal.GetLastWin32Error();
                if (error != ErrorInsufficientBuffer)
                    throw InformationError(informationClass, error);
            }
            if (capacity == 0 || capacity > MaxInformationBytes)
                throw InvalidInformation();

            IntPtr buffer = Marshal.AllocHGlobal((int)capacity);
            try
            {
                CheckDeadline(clock, timeoutMilliseconds);
                uint returned;
                if (!GetTokenInformation(token, informationClass, buffer, capacity, out returned))
                    throw InformationError(informationClass, Marshal.GetLastWin32Error());
                // Windows build 26100 returns one byte for TokenHasRestrictions.
                // The documented DWORD layout remains valid for that class too.
                bool filteredByte = informationClass == InformationClass.HasRestrictions && returned == 1;
                if (returned == 0 || returned > capacity ||
                    (fixedBytes != 0 && returned != capacity && !filteredByte))
                    throw new InvalidDataException("prestart-observer:token-extent-" + informationClass +
                        " " + returned + "/" + capacity);
                CheckDeadline(clock, timeoutMilliseconds);
                T result = read(buffer, (int)returned);
                CheckDeadline(clock, timeoutMilliseconds);
                return result;
            }
            finally
            {
                Marshal.FreeHGlobal(buffer);
            }
        }

        private static TokenStatistics ReadStatisticsValue(IntPtr buffer, int bytes)
        {
            return (TokenStatistics)Marshal.PtrToStructure(buffer, typeof(TokenStatistics));
        }

        private static uint ReadDwordValue(IntPtr buffer, int bytes)
        {
            return unchecked((uint)Marshal.ReadInt32(buffer));
        }

        private static bool ReadFilteredValue(IntPtr buffer, int bytes)
        {
            if (bytes == 1)
                return Marshal.ReadByte(buffer) != 0;
            return ReadDwordValue(buffer, bytes) != 0;
        }

        private static SidEntry ReadSingleSid(IntPtr buffer, int bytes)
        {
            if (bytes < SidEntryBytes)
                throw InvalidInformation();
            return ReadSidEntry(buffer, bytes, 0, SidEntryBytes);
        }

        private static SidEntry[] ReadGroups(IntPtr buffer, int bytes)
        {
            int count = ReadCount(buffer, bytes, GroupEntryOffset, SidEntryBytes);
            int tableEnd = GroupEntryOffset + count * SidEntryBytes;
            var result = new SidEntry[count];
            for (int index = 0; index < count; index++)
                result[index] = ReadSidEntry(buffer, bytes, GroupEntryOffset + index * SidEntryBytes, tableEnd);
            Array.Sort(result);
            return result;
        }

        private static PrivilegeEntry[] ReadPrivileges(IntPtr buffer, int bytes)
        {
            const int first = sizeof(uint);
            int count = ReadCount(buffer, bytes, first, PrivilegeEntryBytes);
            if (bytes != first + count * PrivilegeEntryBytes)
                throw InvalidInformation();
            var result = new PrivilegeEntry[count];
            for (int index = 0; index < count; index++)
            {
                IntPtr entry = IntPtr.Add(buffer, first + index * PrivilegeEntryBytes);
                result[index] = (PrivilegeEntry)Marshal.PtrToStructure(entry, typeof(PrivilegeEntry));
            }
            Array.Sort(result);
            return result;
        }

        private static int ReadCount(IntPtr buffer, int bytes, int first, int entryBytes)
        {
            if (bytes < sizeof(uint))
                throw InvalidInformation();
            uint count = unchecked((uint)Marshal.ReadInt32(buffer));
            if (count != 0 && (bytes < first || count > (uint)((bytes - first) / entryBytes)))
                throw InvalidInformation();
            return (int)count;
        }

        private static SidEntry ReadSidEntry(IntPtr buffer, int bytes, int entryOffset, int firstSid)
        {
            // The caller bounds the entire entry table. Each pointed-to SID must
            // fit inside the same returned buffer, beyond that table.
            IntPtr address = IntPtr.Add(buffer, entryOffset);
            var row = (NativeSidEntry)Marshal.PtrToStructure(address, typeof(NativeSidEntry));
            ulong origin = unchecked((ulong)buffer.ToInt64());
            ulong sid = unchecked((ulong)row.Sid.ToInt64());
            if (bytes < SidHeaderBytes || sid < origin || sid - origin < (ulong)firstSid ||
                sid - origin > (ulong)(bytes - SidHeaderBytes) || (sid & 3) != 0)
                throw InvalidInformation();

            int offset = (int)(sid - origin);
            int count = Marshal.ReadByte(buffer, offset + 1);
            int length = SidHeaderBytes + count * sizeof(uint);
            if (Marshal.ReadByte(buffer, offset) != 1 || count > MaxSidSubAuthorities || length > bytes - offset)
                throw InvalidInformation();
            var binary = new byte[length];
            Marshal.Copy(IntPtr.Add(buffer, offset), binary, 0, length);
            return new SidEntry(new SecurityIdentifier(binary, 0), row.Attributes);
        }

        private static bool SameEntries<T>(T[] expected, T[] observed) where T : IComparable<T>
        {
            if (expected.Length != observed.Length)
                return false;
            for (int index = 0; index < expected.Length; index++)
            {
                if (expected[index].CompareTo(observed[index]) != 0)
                    return false;
            }
            return true;
        }

        private static InvalidDataException InvalidInformation()
        {
            return new InvalidDataException("prestart-observer:token-information");
        }

        private static Win32Exception InformationError(InformationClass informationClass, int error)
        {
            return new Win32Exception(error, "prestart-observer:query-token-" + informationClass);
        }

        internal sealed class ChildToken : IDisposable
        {
            private readonly SafeFileHandle token;
            private readonly long tokenId;
            private readonly long modifiedId;

            internal ChildToken(SafeFileHandle token, long tokenId, long modifiedId)
            {
                this.token = token;
                this.tokenId = tokenId;
                this.modifiedId = modifiedId;
            }

            internal void VerifyUnchanged(Stopwatch clock, int timeoutMilliseconds)
            {
                // The handle still identifies the original token after the child
                // exits. This checks that token's version across collection.
                if (token.IsClosed)
                    throw new ObjectDisposedException("ChildToken");
                TokenStatistics current = ReadStatistics(token, clock, timeoutMilliseconds);
                if (current.Type != PrimaryToken || current.TokenId.Value != tokenId ||
                    current.ModifiedId.Value != modifiedId)
                    throw new InvalidOperationException(ChildActor);
            }

            public void Dispose()
            {
                token.Dispose();
            }
        }

        private sealed class Snapshot
        {
            internal TokenStatistics Statistics;
            internal SidEntry User;
            internal SidEntry[] Groups;
            internal SidEntry[] RestrictingSids;
            internal PrivilegeEntry[] Privileges;
            internal uint Session;
            internal uint ElevationType;
            internal bool Elevated;
            internal bool WasFiltered;
            internal SidEntry Integrity;
            internal bool IsAppContainer;

            internal bool SameContext(Snapshot other)
            {
                // TokenId and ModifiedId describe each individual token object.
                return Statistics.AuthenticationId.Value == other.Statistics.AuthenticationId.Value &&
                    User.CompareTo(other.User) == 0 && Session == other.Session &&
                    ElevationType == other.ElevationType && Elevated == other.Elevated &&
                    WasFiltered == other.WasFiltered && Integrity.CompareTo(other.Integrity) == 0 &&
                    IsAppContainer == other.IsAppContainer && SameEntries(Groups, other.Groups) &&
                    SameEntries(RestrictingSids, other.RestrictingSids) && SameEntries(Privileges, other.Privileges);
            }
        }

        private struct SidEntry : IComparable<SidEntry>
        {
            private readonly SecurityIdentifier sid;
            private readonly uint attributes;

            internal SidEntry(SecurityIdentifier sid, uint attributes)
            {
                this.sid = sid;
                this.attributes = attributes;
            }

            internal bool HasExactIdentity(string expected)
            {
                return string.Equals(sid.Value, expected, StringComparison.Ordinal);
            }

            public int CompareTo(SidEntry other)
            {
                int identity = sid.CompareTo(other.sid);
                return identity == 0 ? attributes.CompareTo(other.attributes) : identity;
            }
        }

        private enum InformationClass
        {
            User = 1,
            Groups = 2,
            Privileges = 3,
            Statistics = 10,
            RestrictedSids = 11,
            SessionId = 12,
            ElevationType = 18,
            Elevation = 20,
            HasRestrictions = 21,
            IntegrityLevel = 25,
            IsAppContainer = 29
        }

        // GetTokenInformation populates these SDK layouts through an unmanaged buffer.
#pragma warning disable 0649
        [StructLayout(LayoutKind.Sequential)]
        private struct Luid
        {
            internal uint Low;
            internal int High;

            internal long Value { get { return ((long)High << 32) | Low; } }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct TokenStatistics
        {
            internal Luid TokenId;
            internal Luid AuthenticationId;
            internal long Expiration;
            internal int Type;
            internal int ImpersonationLevel;
            internal uint DynamicCharged;
            internal uint DynamicAvailable;
            internal uint GroupCount;
            internal uint PrivilegeCount;
            internal Luid ModifiedId;

            internal bool SameVersion(TokenStatistics other)
            {
                return Type == PrimaryToken && other.Type == Type && TokenId.Value == other.TokenId.Value &&
                    ModifiedId.Value == other.ModifiedId.Value && AuthenticationId.Value == other.AuthenticationId.Value &&
                    GroupCount == other.GroupCount && PrivilegeCount == other.PrivilegeCount;
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct NativeSidEntry
        {
            internal IntPtr Sid;
            internal uint Attributes;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct NativeGroups
        {
            internal uint Count;
            internal NativeSidEntry First;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct PrivilegeEntry : IComparable<PrivilegeEntry>
        {
            internal Luid Id;
            internal uint Attributes;

            public int CompareTo(PrivilegeEntry other)
            {
                int identity = Id.Value.CompareTo(other.Id.Value);
                return identity == 0 ? Attributes.CompareTo(other.Attributes) : identity;
            }
        }
#pragma warning restore 0649

        [DllImport("kernel32.dll", ExactSpelling = true)]
        private static extern IntPtr GetCurrentProcess();

        [DllImport("kernel32.dll", ExactSpelling = true)]
        private static extern IntPtr GetCurrentThread();

        [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool OpenThreadToken(IntPtr thread, uint access, bool openAsSelf,
            out SafeFileHandle token);

        [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool OpenProcessToken(IntPtr process, uint access, out SafeFileHandle token);

        [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool OpenProcessToken(SafeHandle process, uint access, out SafeFileHandle token);

        [DllImport("advapi32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern bool GetTokenInformation(SafeFileHandle token, InformationClass informationClass,
            IntPtr buffer, uint capacity, out uint returned);
    }
}
