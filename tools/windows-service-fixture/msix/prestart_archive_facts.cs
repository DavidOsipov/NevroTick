// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;
using System.Threading;

namespace NevroTick
{
    // These observations describe a single archive snapshot. They confer no signature or deployment authority.
    public sealed class PrestartArchiveFacts
    {
        internal const int MaximumArchiveBytes = 8 * 1024 * 1024;
        private const int MaximumExpandedBytes = 8 * 1024 * 1024;
        private const int MaximumMemberBytes = 2 * 1024 * 1024;
        private const int MaximumManifestBytes = 64 * 1024;
        private const int MaximumIdentityUnits = 256;
        private const int ManifestFootprint = 0;
        private const uint InProcessServer = 1;
        private const uint MultiThreaded = 0;
        private const uint ApartmentThreaded = 2;
        private const int Success = 0;
        private const int EndOfStream = 1;
        private const string PackageName = "NevroTick.S06.ScmFixture";
        private const string PackagePublisher = "CN=NevroTick S06 Laboratory";
        private const string FullNamePrefix = PackageName + "_1.0.0.0_x64__";
        private const string ManifestMember = "AppxManifest.xml";
        private const string EngineMember = "nevrotick-lab-s02-engine.exe";
        private const string BrokerMember = "nevrotick-lab-s02-broker.exe";
        private static readonly string[] PayloadMembers = {
            "Assets/PackageLogo.png",
            "Assets/Square150x150Logo.png",
            "Assets/Square44x44Logo.png",
            "Inert.exe",
            BrokerMember,
            EngineMember,
        };

        public string PackageFullName { get; private set; }
        public int ArchiveBytes { get; private set; }
        public string ArchiveSha256 { get; private set; }
        public int EngineBytes { get; private set; }
        public string EngineSha256 { get; private set; }
        public int BrokerBytes { get; private set; }
        public string BrokerSha256 { get; private set; }
        public string PayloadSha256 { get; private set; }

        private PrestartArchiveFacts(string fullName, int size, string digest, Member engine, Member broker,
            string payloadSha256)
        {
            PackageFullName = fullName;
            ArchiveBytes = size;
            ArchiveSha256 = digest;
            EngineBytes = engine.Size;
            EngineSha256 = engine.Digest;
            BrokerBytes = broker.Size;
            BrokerSha256 = broker.Digest;
            PayloadSha256 = payloadSha256;
        }

        // Verify the expected digest of a private copy before handing those same bytes to Windows.
        // Returns facts only after the reader and all native objects have been released.
        public static PrestartArchiveFacts Inspect(byte[] archive, string expectedSha256)
        {
            return InspectArchive(archive, expectedSha256, null);
        }

        // Match the fixed manifest and complete authored payload with an independent catalog pin.
        public static PrestartArchiveFacts InspectWithPayload(byte[] archive, string expectedSha256,
            string expectedPayloadSha256)
        {
            if (!IsSha256(expectedPayloadSha256))
            {
                throw new InvalidDataException("prestart-archive:expected-payload-sha256");
            }
            return InspectArchive(archive, expectedSha256, expectedPayloadSha256);
        }

        private static PrestartArchiveFacts InspectArchive(byte[] archive, string expectedSha256,
            string expectedPayloadSha256)
        {
            if (archive == null || archive.Length == 0 || archive.Length > MaximumArchiveBytes)
            {
                throw new InvalidDataException("prestart-archive:archive-size");
            }
            if (!IsSha256(expectedSha256))
            {
                throw new InvalidDataException("prestart-archive:expected-sha256");
            }

            byte[] snapshot = (byte[])archive.Clone();
            using (SHA256 sha = SHA256.Create())
            {
                string digest = Digest(sha, snapshot);
                if (!string.Equals(digest, expectedSha256, StringComparison.Ordinal))
                {
                    throw new InvalidDataException("prestart-archive:archive-digest");
                }
                return ReadPackage(snapshot, digest, sha, expectedPayloadSha256);
            }
        }

        private static bool IsSha256(string digest)
        {
            if (digest == null || digest.Length != 64)
            {
                return false;
            }
            foreach (char character in digest)
            {
                if (!(character >= '0' && character <= '9') && !(character >= 'a' && character <= 'f'))
                {
                    return false;
                }
            }
            return true;
        }

        private static string Digest(SHA256 sha, byte[] bytes)
        {
            return BitConverter.ToString(sha.ComputeHash(bytes)).Replace("-", string.Empty).ToLowerInvariant();
        }

        private static PrestartArchiveFacts ReadPackage(byte[] snapshot, string digest, SHA256 sha,
            string expectedPayloadSha256)
        {
            uint apartment = Thread.CurrentThread.GetApartmentState() == ApartmentState.STA
                ? ApartmentThreaded : MultiThreaded;
            int initialized = CoInitializeEx(IntPtr.Zero, apartment);
            if (initialized != Success && initialized != EndOfStream)
            {
                ThrowNative(initialized, "apartment");
            }

            try
            {
                IArchiveStream stream = SHCreateMemStream(snapshot, (uint)snapshot.Length);
                if (stream == null)
                {
                    throw new OutOfMemoryException("prestart-archive:memory-stream");
                }
                try
                {
                    return ReadStream(stream, snapshot.Length, digest, sha, expectedPayloadSha256);
                }
                finally
                {
                    Release(stream);
                }
            }
            finally
            {
                CoUninitialize();
            }
        }

        private static PrestartArchiveFacts ReadStream(IArchiveStream stream, int size, string digest, SHA256 sha,
            string expectedPayloadSha256)
        {
            Guid classId = new Guid("5842a140-ff9f-4166-8f5c-62f5b7b0c781");
            Guid interfaceId = typeof(IAppxFactory).GUID;
            IAppxFactory factory;
            int created = CoCreateInstance(ref classId, IntPtr.Zero, InProcessServer, ref interfaceId, out factory);
            try
            {
                RequireObject(created, factory, "factory");
                IAppxPackageReader reader;
                int opened = factory.CreatePackageReader(stream, out reader);
                try
                {
                    RequireObject(opened, reader, "package-reader");
                    string fullName = ReadIdentity(reader);
                    if (expectedPayloadSha256 != null)
                    {
                        SortedDictionary<string, Member> members = ReadAuthoredMembers(reader, sha);
                        string payloadSha256 = DigestMembers(members, sha);
                        if (!string.Equals(payloadSha256, expectedPayloadSha256, StringComparison.Ordinal))
                        {
                            throw new InvalidDataException("prestart-archive:payload-digest");
                        }
                        return new PrestartArchiveFacts(fullName, size, digest, members[EngineMember],
                            members[BrokerMember], payloadSha256);
                    }
                    Member engine = ReadMember(reader, EngineMember, sha);
                    Member broker = ReadMember(reader, BrokerMember, sha);
                    return new PrestartArchiveFacts(fullName, size, digest, engine, broker, null);
                }
                finally
                {
                    Release(reader);
                }
            }
            finally
            {
                Release(factory);
            }
        }

        private static string ReadIdentity(IAppxPackageReader reader)
        {
            IAppxManifestReader manifest;
            int opened = reader.GetManifest(out manifest);
            try
            {
                RequireObject(opened, manifest, "manifest");
                IAppxManifestPackageId id;
                int identified = manifest.GetPackageId(out id);
                try
                {
                    RequireObject(identified, id, "package-id");
                    IntPtr namePointer;
                    int nameStatus = id.GetName(out namePointer);
                    string name = ReadString(nameStatus, namePointer, "package-name");
                    IntPtr publisherPointer;
                    int publisherStatus = id.GetPublisher(out publisherPointer);
                    string publisher = ReadString(publisherStatus, publisherPointer, "publisher");
                    IntPtr fullNamePointer;
                    int fullNameStatus = id.GetPackageFullName(out fullNamePointer);
                    string fullName = ReadString(fullNameStatus, fullNamePointer, "package-full-name");
                    if (!string.Equals(name, PackageName, StringComparison.Ordinal)
                        || !string.Equals(publisher, PackagePublisher, StringComparison.Ordinal)
                        || !IsFullName(fullName))
                    {
                        throw new InvalidDataException("prestart-archive:manifest-identity");
                    }
                    return fullName;
                }
                finally
                {
                    Release(id);
                }
            }
            finally
            {
                Release(manifest);
            }
        }

        private static bool IsFullName(string fullName)
        {
            if (!fullName.StartsWith(FullNamePrefix, StringComparison.Ordinal)
                || fullName.Length != FullNamePrefix.Length + 13)
            {
                return false;
            }
            for (int index = FullNamePrefix.Length; index < fullName.Length; ++index)
            {
                char character = fullName[index];
                if (!(character >= '0' && character <= '9') && !(character >= 'a' && character <= 'z'))
                {
                    return false;
                }
            }
            return true;
        }

        private static Member ReadMember(IAppxPackageReader reader, string expectedName, SHA256 sha)
        {
            IAppxFile file;
            int found = reader.GetPayloadFile(expectedName, out file);
            try
            {
                RequireObject(found, file, "payload-file");
                if (!string.Equals(ReadMemberName(file), expectedName, StringComparison.Ordinal))
                {
                    throw new InvalidDataException("prestart-archive:member-name");
                }
                return ReadMemberContents(file, MaximumMemberBytes, sha);
            }
            finally
            {
                Release(file);
            }
        }

        private static SortedDictionary<string, Member> ReadAuthoredMembers(IAppxPackageReader reader, SHA256 sha)
        {
            Member manifest = ReadManifestMember(reader, sha);
            SortedDictionary<string, Member> members = new SortedDictionary<string, Member>(StringComparer.Ordinal);
            members.Add(ManifestMember, manifest);
            int remainingBytes = MaximumExpandedBytes - manifest.Size;
            IAppxFilesEnumerator enumerator;
            int opened = reader.GetPayloadFiles(out enumerator);
            try
            {
                RequireObject(opened, enumerator, "payload-files");
                int hasCurrent;
                RequireSuccess(enumerator.GetHasCurrent(out hasCurrent), "payload-current");
                for (int index = 0; index < PayloadMembers.Length; ++index)
                {
                    if (hasCurrent == 0)
                    {
                        throw new InvalidDataException("prestart-archive:payload-roster");
                    }

                    IAppxFile file;
                    int found = enumerator.GetCurrent(out file);
                    try
                    {
                        RequireObject(found, file, "payload-file");
                        string name = ReadMemberName(file).Replace('\\', '/');
                        if (Array.IndexOf(PayloadMembers, name) < 0 || members.ContainsKey(name))
                        {
                            throw new InvalidDataException("prestart-archive:payload-roster");
                        }

                        Member member = ReadMemberContents(file, Math.Min(MaximumMemberBytes, remainingBytes), sha);
                        remainingBytes -= member.Size;
                        members.Add(name, member);
                    }
                    finally
                    {
                        Release(file);
                    }
                    RequireSuccess(enumerator.MoveNext(out hasCurrent), "payload-next");
                }
                // Observe that the enumeration ended without opening another member.
                if (hasCurrent != 0)
                {
                    throw new InvalidDataException("prestart-archive:payload-roster");
                }
                return members;
            }
            finally
            {
                Release(enumerator);
            }
        }

        private static Member ReadManifestMember(IAppxPackageReader reader, SHA256 sha)
        {
            IAppxFile file;
            int found = reader.GetFootprintFile(ManifestFootprint, out file);
            try
            {
                RequireObject(found, file, "manifest-footprint");
                if (!string.Equals(ReadMemberName(file), ManifestMember, StringComparison.Ordinal))
                {
                    throw new InvalidDataException("prestart-archive:member-name");
                }
                return ReadMemberContents(file, MaximumManifestBytes, sha);
            }
            finally
            {
                Release(file);
            }
        }

        private static string DigestMembers(SortedDictionary<string, Member> members, SHA256 sha)
        {
            StringBuilder catalog = new StringBuilder();
            foreach (KeyValuePair<string, Member> entry in members)
            {
                catalog.Append(entry.Key).Append('\t')
                    .Append(entry.Value.Size.ToString(CultureInfo.InvariantCulture)).Append('\t')
                    .Append(entry.Value.Digest).Append('\n');
            }
            return Digest(sha, Encoding.UTF8.GetBytes(catalog.ToString()));
        }

        private static string ReadMemberName(IAppxFile file)
        {
            IntPtr namePointer;
            int nameStatus = file.GetName(out namePointer);
            return ReadString(nameStatus, namePointer, "payload-name");
        }

        private static Member ReadMemberContents(IAppxFile file, int maximumBytes, SHA256 sha)
        {
            ulong size;
            RequireSuccess(file.GetSize(out size), "payload-size");
            if (size == 0 || size > (ulong)maximumBytes)
            {
                throw new InvalidDataException("prestart-archive:member-size");
            }
            int boundedSize = checked((int)size);
            IArchiveStream stream;
            int opened = file.GetStream(out stream);
            try
            {
                RequireObject(opened, stream, "payload-stream");
                return ReadMemberStream(stream, boundedSize, sha);
            }
            finally
            {
                Release(stream);
            }
        }

        private static Member ReadMemberStream(IArchiveStream stream, int size, SHA256 sha)
        {
            byte[] bytes = new byte[size];
            uint read;
            int status = stream.Read(bytes, (uint)size, out read);
            if (status != Success && status != EndOfStream)
            {
                ThrowNative(status, "payload-read");
            }
            if (status != Success || read != (uint)size)
            {
                throw new InvalidDataException("prestart-archive:member-length");
            }

            byte[] trailing = new byte[1];
            int eof = stream.Read(trailing, 1, out read);
            if (eof != Success && eof != EndOfStream)
            {
                ThrowNative(eof, "payload-eof");
            }
            if (read != 0)
            {
                throw new InvalidDataException("prestart-archive:member-length");
            }
            return new Member(size, Digest(sha, bytes));
        }

        // Success strings are NUL-terminated allocations owned by the caller per AppxPackaging.h.
        private static string ReadString(int status, IntPtr pointer, string stage)
        {
            try
            {
                RequireSuccess(status, stage);
                if (pointer == IntPtr.Zero)
                {
                    throw new InvalidDataException("prestart-archive:" + stage);
                }
                StringBuilder text = new StringBuilder();
                for (int index = 0; index < MaximumIdentityUnits; ++index)
                {
                    char unit = (char)Marshal.ReadInt16(pointer, index * sizeof(char));
                    if (unit == '\0')
                    {
                        return text.ToString();
                    }
                    text.Append(unit);
                }
                throw new InvalidDataException("prestart-archive:" + stage);
            }
            finally
            {
                Marshal.FreeCoTaskMem(pointer);
            }
        }

        private static void RequireObject(int status, object value, string stage)
        {
            RequireSuccess(status, stage);
            if (value == null)
            {
                throw new InvalidDataException("prestart-archive:" + stage);
            }
        }

        private static void RequireSuccess(int status, string stage)
        {
            if (status != Success)
            {
                ThrowNative(status, stage);
            }
        }

        private static void ThrowNative(int status, string stage)
        {
            throw new COMException("prestart-archive:" + stage, status);
        }

        private static void Release(object value)
        {
            if (value != null)
            {
                Marshal.ReleaseComObject(value);
            }
        }

        private sealed class Member
        {
            internal readonly int Size;
            internal readonly string Digest;

            internal Member(int size, string digest)
            {
                Size = size;
                Digest = digest;
            }
        }

        [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
        [DllImport("ole32.dll", ExactSpelling = true)]
        private static extern int CoInitializeEx(IntPtr reserved, uint apartment);

        [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
        [DllImport("ole32.dll", ExactSpelling = true)]
        private static extern void CoUninitialize();

        [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
        [DllImport("ole32.dll", ExactSpelling = true)]
        private static extern int CoCreateInstance(ref Guid classId, IntPtr outer, uint context, ref Guid interfaceId,
            [MarshalAs(UnmanagedType.Interface)] out IAppxFactory factory);

        [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
        [DllImport("shlwapi.dll", ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.Interface)]
        private static extern IArchiveStream SHCreateMemStream([In] byte[] bytes, uint size);

        // Windows SDK vtable prefixes. Keep each preceding slot in its declared order, including unused slots.
        [ComImport, Guid("beb94909-e451-438b-b5a7-d79e767b75d8"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IAppxFactory
        {
            [PreserveSig] int CreatePackageWriter(IArchiveStream output, IntPtr settings, out IntPtr writer);
            [PreserveSig] int CreatePackageReader(IArchiveStream input, out IAppxPackageReader reader);
        }

        [ComImport, Guid("b5c49650-99bc-481c-9a34-3d53a4106708"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IAppxPackageReader
        {
            [PreserveSig] int GetBlockMap(out IntPtr blockMap);
            [PreserveSig] int GetFootprintFile(int kind, out IAppxFile file);
            [PreserveSig] int GetPayloadFile([MarshalAs(UnmanagedType.LPWStr)] string name, out IAppxFile file);
            [PreserveSig] int GetPayloadFiles(out IAppxFilesEnumerator enumerator);
            [PreserveSig] int GetManifest(out IAppxManifestReader manifest);
        }

        [ComImport, Guid("f007eeaf-9831-411c-9847-917cdc62d1fe"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IAppxFilesEnumerator
        {
            [PreserveSig] int GetCurrent(out IAppxFile file);
            [PreserveSig] int GetHasCurrent(out int hasCurrent);
            [PreserveSig] int MoveNext(out int hasNext);
        }

        [ComImport, Guid("4e1bd148-55a0-4480-a3d1-15544710637c"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IAppxManifestReader
        {
            [PreserveSig] int GetPackageId(out IAppxManifestPackageId id);
        }

        [ComImport, Guid("283ce2d7-7153-4a91-9649-7a0f7240945f"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IAppxManifestPackageId
        {
            [PreserveSig] int GetName(out IntPtr name);
            [PreserveSig] int GetArchitecture(out int architecture);
            [PreserveSig] int GetPublisher(out IntPtr publisher);
            [PreserveSig] int GetVersion(out ulong version);
            [PreserveSig] int GetResourceId(out IntPtr resourceId);
            [PreserveSig] int ComparePublisher([MarshalAs(UnmanagedType.LPWStr)] string publisher, out int same);
            [PreserveSig] int GetPackageFullName(out IntPtr fullName);
        }

        [ComImport, Guid("91df827b-94fd-468f-827b-57f41b2f6f2e"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IAppxFile
        {
            [PreserveSig] int GetCompressionOption(out int compression);
            [PreserveSig] int GetContentType(out IntPtr contentType);
            [PreserveSig] int GetName(out IntPtr name);
            [PreserveSig] int GetSize(out ulong size);
            [PreserveSig] int GetStream(out IArchiveStream stream);
        }

        // IStream inherits ISequentialStream, whose first slot after IUnknown is Read.
        [ComImport, Guid("0000000c-0000-0000-c000-000000000046"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IArchiveStream
        {
            [PreserveSig]
            int Read([Out, MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] bytes, uint count, out uint read);
        }
    }
}
