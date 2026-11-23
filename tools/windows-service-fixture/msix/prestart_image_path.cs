// SPDX-License-Identifier: GPL-2.0-only
using System.ComponentModel;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.Win32.SafeHandles;

namespace NevroTick
{
    public static class S06ImagePath
    {
        [DefaultDllImportSearchPaths(DllImportSearchPath.System32)]
        [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        private static extern uint GetFinalPathNameByHandleW(
            SafeFileHandle handle, StringBuilder path, uint capacity, uint flags);

        // Borrow the existing open handle and return its normalized DOS volume path.
        public static string Read(SafeFileHandle handle)
        {
            StringBuilder path = new StringBuilder(1024);
            uint count = GetFinalPathNameByHandleW(handle, path, (uint)path.Capacity, 0);
            if (count == 0)
            {
                int error = Marshal.GetLastWin32Error();
                throw new Win32Exception(error);
            }
            if (count >= path.Capacity || count != path.Length)
            {
                throw new InvalidDataException("Final image path exceeds its output bound");
            }
            return path.ToString();
        }
    }
}
