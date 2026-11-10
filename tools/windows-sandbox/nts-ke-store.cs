// SPDX-License-Identifier: GPL-2.0-only
// Guest-only fixture preparation. This file is not a product trust adapter.
// The caller admits exact synthetic DER/hash inputs and the prepared guest.
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Security.Cryptography;

namespace NevroTickGuestKe
{
    // One instance binds exactly one already-admitted fixture root and CRL.
    // No arbitrary store, location, flag, path, replacement or cache API exists.
    public sealed class StoreFixture
    {
        private const uint CurrentUser = 0x00010000;
        private const uint OpenExisting = 0x00004000;
        private const uint ReadOnly = 0x00008000;
        private const uint AddNew = 1;
        private const uint X509 = 1;
        private const uint CheckClose = 2;
        private const int MaxEntries = 1024;
        private readonly byte[] root;
        private readonly byte[] crl;
        private bool rootAdded;
        private bool crlAdded;

        // Native pointer alignment is part of the x64-only contract. Both SDK
        // structures have this prefix/full layout; no native info is followed.
        [StructLayout(LayoutKind.Sequential)]
        private struct EncodedContext
        {
            internal uint Encoding;
            internal IntPtr Bytes;
            internal uint Length;
            internal IntPtr Info;
            internal IntPtr Store;
        }

        [DllImport("crypt32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern IntPtr CertOpenStore(IntPtr provider, uint encoding,
            IntPtr cryptProvider, uint flags, string name);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertCloseStore(IntPtr store, uint flags);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern IntPtr CertEnumCertificatesInStore(IntPtr store, IntPtr previous);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        private static extern IntPtr CertEnumCRLsInStore(IntPtr store, IntPtr previous);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertFreeCertificateContext(IntPtr context);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertFreeCRLContext(IntPtr context);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertAddEncodedCertificateToStore(IntPtr store,
            uint encoding, byte[] der, uint length, uint disposition, IntPtr context);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertAddEncodedCRLToStore(IntPtr store,
            uint encoding, byte[] der, uint length, uint disposition, IntPtr context);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertDeleteCertificateFromStore(IntPtr context);
        [DllImport("crypt32.dll", ExactSpelling = true, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CertDeleteCRLFromStore(IntPtr context);

        // Fixed error labels and numeric OS causes contain no DER/key material.
        private static Exception NativeFailure(string operation, int error)
        {
            return new Win32Exception(error, "ke-store:" + operation);
        }

        private static void RequireGuest()
        {
            if (IntPtr.Size != 8 || Environment.OSVersion.Platform != PlatformID.Win32NT ||
                Environment.OSVersion.Version.Build != 26100 ||
                !String.Equals(Environment.UserName, "WDAGUtilityAccount", StringComparison.Ordinal))
                throw new InvalidOperationException("ke-store:guest-profile");
            if (Marshal.SizeOf(typeof(EncodedContext)) != 40)
                throw new InvalidOperationException("ke-store:context-layout");
        }

        private static string Hash(byte[] der)
        {
            using (SHA256 sha = SHA256.Create())
                return BitConverter.ToString(sha.ComputeHash(der)).Replace("-", "").ToLowerInvariant();
        }

        private static byte[] Admit(byte[] der, string expectedHash, int limit)
        {
            if (der == null || der.Length < 1 || der.Length > limit ||
                expectedHash == null || expectedHash.Length != 64)
                throw new ArgumentException("ke-store:input-bound");
            // Clone before hashing; subsequent caller mutation cannot change the
            // admitted object's bytes. A hash supplies identity, not trust.
            byte[] copy = (byte[])der.Clone();
            if (!String.Equals(Hash(copy), expectedHash, StringComparison.Ordinal))
                throw new ArgumentException("ke-store:input-identity");
            return copy;
        }

        // Construction has no store effect. Installation is separately explicit.
        public StoreFixture(byte[] rootDer, string rootHash, byte[] crlDer, string crlHash)
        {
            RequireGuest();
            root = Admit(rootDer, rootHash, 16384);
            crl = Admit(crlDer, crlHash, 65536);
        }

        private static IntPtr Open(bool certificate, bool writable)
        {
            RequireGuest();
            // SYSTEM_REGISTRY_W opens the physical user registry store itself,
            // rather than the logical collection with inherited siblings.
            IntPtr store = CertOpenStore(new IntPtr(13), 0, IntPtr.Zero,
                CurrentUser | OpenExisting | (writable ? 0u : ReadOnly),
                certificate ? "Root" : "CA");
            if (store == IntPtr.Zero)
                throw NativeFailure("open", Marshal.GetLastWin32Error());
            return store;
        }

        private static byte[] CopyDer(IntPtr context, bool certificate)
        {
            EncodedContext value = (EncodedContext)Marshal.PtrToStructure(context, typeof(EncodedContext));
            int limit = certificate ? 16384 : 65536;
            if (value.Bytes == IntPtr.Zero || value.Length < 1 || value.Length > limit)
                throw new InvalidOperationException("ke-store:encoded-bound");
            byte[] bytes = new byte[checked((int)value.Length)];
            Marshal.Copy(value.Bytes, bytes, 0, bytes.Length);
            return bytes;
        }

        private static bool Equal(byte[] left, byte[] right)
        {
            if (left.Length != right.Length) return false;
            for (int i = 0; i < left.Length; ++i)
                if (left[i] != right[i]) return false;
            return true; // Public certificates/CRLs; no secret comparison.
        }

        private static IntPtr Next(IntPtr store, ref IntPtr current, bool certificate)
        {
            // Enumeration consumes the previous context, including on final
            // NULL. Clear our owner before transferring it to the native API.
            IntPtr previous = current;
            current = IntPtr.Zero;
            IntPtr next = certificate ? CertEnumCertificatesInStore(store, previous) :
                CertEnumCRLsInStore(store, previous);
            int error = Marshal.GetLastWin32Error();
            current = next;
            // CRYPT_E_NOT_FOUND is the documented ordinary enumeration end.
            if (next == IntPtr.Zero && error != unchecked((int)0x80092004))
                throw NativeFailure("enumerate", error);
            return next;
        }

        private static void ReleaseContext(ref IntPtr context, bool certificate)
        {
            IntPtr owned = context;
            context = IntPtr.Zero;
            if (owned == IntPtr.Zero) return;
            bool success = certificate ? CertFreeCertificateContext(owned) : CertFreeCRLContext(owned);
            if (!success) throw NativeFailure("free", Marshal.GetLastWin32Error());
        }

        private static void Close(ref IntPtr store)
        {
            IntPtr owned = store;
            store = IntPtr.Zero;
            if (owned != IntPtr.Zero && !CertCloseStore(owned, CheckClose))
                throw NativeFailure("close", Marshal.GetLastWin32Error());
        }

        private sealed class ContextOwner
        {
            internal IntPtr Value = IntPtr.Zero;
        }

        // Always attempt both releases, retaining primary and secondary facts.
        // A cleanup exception cannot silently replace the operation's failure.
        private static void InStore(bool rootStore, bool writable, bool certificate,
            Action<IntPtr, ContextOwner> action)
        {
            ContextOwner context = new ContextOwner();
            // Allocate cleanup custody before acquiring any native resource.
            // Capacity covers the primary, context-free and store-close facts.
            List<Exception> cleanup = new List<Exception>(3);
            IntPtr store = Open(rootStore, writable);
            Exception primary = null;
            try { action(store, context); }
            catch (Exception failure) { primary = failure; throw; }
            finally
            {
                try { ReleaseContext(ref context.Value, certificate); }
                catch (Exception failure) { cleanup.Add(failure); }
                try { Close(ref store); }
                catch (Exception failure) { cleanup.Add(failure); }
                if (cleanup.Count != 0)
                {
                    if (primary != null) cleanup.Insert(0, primary);
                    throw new AggregateException("ke-store:cleanup", cleanup);
                }
            }
        }

        // Output consists only of sorted store/kind/SHA256 labels. Both kinds
        // are inventoried in both physical stores; counts/DER copies are finite.
        public string[] Snapshot()
        {
            RequireGuest();
            List<string> rows = new List<string>();
            foreach (bool rootStore in new bool[] { true, false })
            {
                foreach (bool certificate in new bool[] { true, false })
                {
                    InStore(rootStore, false, certificate, delegate(IntPtr store, ContextOwner context)
                    {
                        int count = 0;
                        while (Next(store, ref context.Value, certificate) != IntPtr.Zero)
                        {
                            if (++count > MaxEntries)
                                throw new InvalidOperationException("ke-store:entry-bound");
                            byte[] der = CopyDer(context.Value, certificate);
                            rows.Add((rootStore ? "Root" : "CA") + "|" +
                                (certificate ? "cert" : "crl") + "|" + Hash(der));
                        }
                    });
                }
            }
            rows.Sort(StringComparer.Ordinal);
            return rows.ToArray();
        }

        public void AssertAbsent()
        {
            string rootHash = "|" + Hash(root);
            string crlHash = "|" + Hash(crl);
            foreach (string row in Snapshot())
                if (row.EndsWith(rootHash, StringComparison.Ordinal) ||
                    row.EndsWith(crlHash, StringComparison.Ordinal))
                    throw new InvalidOperationException("ke-store:fixture-preexists");
        }

        private void Add(bool certificate)
        {
            RequireGuest();
            if (certificate ? rootAdded : crlAdded)
                throw new InvalidOperationException("ke-store:already-owned");
            byte[] der = certificate ? root : crl;
            InStore(certificate, true, certificate, delegate(IntPtr store, ContextOwner context)
            {
                bool success = certificate ? CertAddEncodedCertificateToStore(store, X509, der,
                    checked((uint)der.Length), AddNew, IntPtr.Zero) :
                    CertAddEncodedCRLToStore(store, X509, der,
                    checked((uint)der.Length), AddNew, IntPtr.Zero);
                if (!success) throw NativeFailure("add", Marshal.GetLastWin32Error());
                // Retain actual insertion even if later close fails. Cleanup may
                // remove only entries whose insertion this instance witnessed.
                if (certificate) rootAdded = true; else crlAdded = true;
            });
        }

        private void Remove(bool certificate)
        {
            RequireGuest();
            if (!(certificate ? rootAdded : crlAdded)) return;
            byte[] expected = certificate ? root : crl;
            InStore(certificate, true, certificate, delegate(IntPtr store, ContextOwner context)
            {
                int count = 0;
                while (Next(store, ref context.Value, certificate) != IntPtr.Zero)
                {
                    if (++count > MaxEntries)
                        throw new InvalidOperationException("ke-store:entry-bound");
                    if (!Equal(CopyDer(context.Value, certificate), expected)) continue;
                    // Delete consumes the context even on failure. Never free
                    // or pass it to enumeration afterward.
                    IntPtr owned = context.Value;
                    context.Value = IntPtr.Zero;
                    bool success = certificate ? CertDeleteCertificateFromStore(owned) :
                        CertDeleteCRLFromStore(owned);
                    if (!success) throw NativeFailure("delete", Marshal.GetLastWin32Error());
                    if (certificate) rootAdded = false; else crlAdded = false;
                    return;
                }
                throw new InvalidOperationException("ke-store:owned-entry-missing");
            });
        }

        public void InstallRoot() { Add(true); }
        public void InstallCrl() { Add(false); }
        public void RemoveRoot() { Remove(true); }
        public void RemoveCrl() { Remove(false); }
        public bool RootOwned { get { return rootAdded; } }
        public bool CrlOwned { get { return crlAdded; } }
    }
}
