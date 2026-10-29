<#
.SYNOPSIS
Observe one owned UI-M0 candidate without elevation or machine configuration.
.DESCRIPTION
Launches only the explicit local executable, exercises native controls and normal
window lifecycle, records bounded in-process resource samples and captures the
owned foreground window. Requires exclusive desktop interaction for this run.
Evidence is written to a fresh explicit directory, never to application state.
This is UI-M0 evidence, not DPI/accessibility, leak/soak or release qualification.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$ImagePath,
    [Parameter(Mandatory = $true)][string]$OutputDirectory,
    [ValidateRange(5, 40)][int]$Cycles = 20
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = [Security.Principal.WindowsPrincipal]::new($identity)
if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'Run the observation helper with an ordinary unelevated token.'
}
$image = Get-Item -LiteralPath $ImagePath
if ($image.PSIsContainer -or $image.Extension -ne '.exe' -or
    $image.FullName.StartsWith('\\') -or
    (($image.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) {
    throw 'ImagePath must name the owned regular executable on a local Windows drive.'
}
$destination = [IO.Path]::GetFullPath($OutputDirectory)
if ($destination.StartsWith('\\') -or (Test-Path -LiteralPath $destination)) {
    throw 'OutputDirectory must be a new local Windows directory.'
}
$null = New-Item -ItemType Directory -Path $destination
$initialHash = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash
$helperPath = $PSCommandPath
$helperHash = (Get-FileHash -LiteralPath $helperPath -Algorithm SHA256).Hash

Add-Type -AssemblyName System.Drawing
Add-Type -TypeDefinition @'
using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Text;

public static class NevroTickM0Observation {
    [StructLayout(LayoutKind.Sequential)] public struct Rect { public int Left, Top, Right, Bottom; }
    [StructLayout(LayoutKind.Sequential)] public struct Point { public int X, Y; }
    [StructLayout(LayoutKind.Sequential)] private struct MouseInput {
        public int dx, dy; public uint mouseData, dwFlags, time; public UIntPtr extra;
    }
    [StructLayout(LayoutKind.Sequential)] private struct Input { public uint type; public MouseInput mouse; }
    [DllImport("user32.dll", SetLastError = true)] public static extern bool SetProcessDPIAware();
    [DllImport("user32.dll")] public static extern uint GetDpiForSystem();
    [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr window);
    [DllImport("shcore.dll")] public static extern int GetProcessDpiAwareness(IntPtr process, out int awareness);
    [DllImport("user32.dll", SetLastError = true)] public static extern bool GetWindowRect(IntPtr window, out Rect rectangle);
    [DllImport("user32.dll", SetLastError = true)] public static extern bool GetClientRect(IntPtr window, out Rect rectangle);
    [DllImport("user32.dll", SetLastError = true)] public static extern bool ClientToScreen(IntPtr window, ref Point point);
    [DllImport("user32.dll")] public static extern IntPtr GetDlgItem(IntPtr window, int identifier);
    [DllImport("user32.dll")] private static extern int GetDlgCtrlID(IntPtr window);
    [DllImport("user32.dll")] private static extern IntPtr GetParent(IntPtr window);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] private static extern int GetClassName(IntPtr window, StringBuilder name, int capacity);
    [DllImport("user32.dll")] private static extern uint GetWindowThreadProcessId(IntPtr window, out uint process);
    [DllImport("user32.dll", SetLastError=true)] private static extern bool ScreenToClient(IntPtr window, ref Point point);
    [DllImport("user32.dll")] private static extern IntPtr RealChildWindowFromPoint(IntPtr parent, Point point);
    [DllImport("user32.dll")] public static extern bool IsWindowEnabled(IntPtr window);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr window);
    [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr window);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr window, int command);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr window);
    [DllImport("user32.dll", SetLastError=true)] private static extern bool SetWindowPos(IntPtr window, IntPtr after, int x, int y, int width, int height, uint flags);
    [DllImport("user32.dll", EntryPoint="GetWindowLongPtrW", SetLastError=true)] private static extern IntPtr GetWindowLongPtr(IntPtr window, int index);
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] public static extern IntPtr WindowFromPoint(Point point);
    [DllImport("user32.dll", SetLastError = true)] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll", SetLastError = true)] public static extern bool GetCursorPos(out Point point);
    [DllImport("user32.dll", SetLastError = true)] private static extern uint SendInput(uint count, Input[] inputs, int size);
    [DllImport("user32.dll", SetLastError = true)] private static extern uint GetGuiResources(IntPtr process, uint flags);
    [DllImport("user32.dll", EntryPoint="SendMessageTimeoutW", CharSet=CharSet.Unicode, SetLastError=true)]
    private static extern IntPtr SendNumber(IntPtr window, uint message, UIntPtr word, IntPtr value, uint flags, uint timeout, out UIntPtr result);
    [DllImport("user32.dll", EntryPoint="SendMessageTimeoutW", CharSet=CharSet.Unicode, SetLastError=true)]
    private static extern IntPtr SendText(IntPtr window, uint message, UIntPtr word, string value, uint flags, uint timeout, out UIntPtr result);
    [DllImport("user32.dll", EntryPoint="SendMessageTimeoutW", CharSet=CharSet.Unicode, SetLastError=true)]
    private static extern IntPtr ReadText(IntPtr window, uint message, UIntPtr word, StringBuilder value, uint flags, uint timeout, out UIntPtr result);
    [DllImport("advapi32.dll", SetLastError=true)] private static extern bool OpenProcessToken(IntPtr process, uint access, out IntPtr token);
    [DllImport("advapi32.dll", SetLastError=true)] private static extern bool GetTokenInformation(IntPtr token, int kind, out int elevation, int size, out int returned);
    [DllImport("kernel32.dll", SetLastError=true)] private static extern bool CloseHandle(IntPtr handle);
    [DllImport("kernel32.dll")] private static extern void SetLastError(uint code);

    public static uint GuiResources(IntPtr process, uint flags) {
        SetLastError(0);
        uint count = GetGuiResources(process, flags);
        int error = Marshal.GetLastWin32Error();
        if (count == 0 && error != 0) throw new Win32Exception(error, "GUI resource observation failed");
        return count;
    }

    public static ulong Number(IntPtr window, uint message, ulong word) {
        UIntPtr result;
        if (SendNumber(window, message, new UIntPtr(word), IntPtr.Zero, 2, 2000, out result) == IntPtr.Zero)
            throw new Win32Exception(Marshal.GetLastWin32Error(), "Bounded native message failed");
        return result.ToUInt64();
    }
    public static string Text(IntPtr window) {
        ulong length = Number(window, 14, 0);
        if (length > 4096) throw new InvalidOperationException("Unexpected native output length");
        var buffer = new StringBuilder(checked((int)length + 1));
        UIntPtr count;
        if (ReadText(window, 13, new UIntPtr(checked(length + 1)), buffer, 2, 2000, out count) == IntPtr.Zero)
            throw new Win32Exception(Marshal.GetLastWin32Error(), "Bounded native text read failed");
        if (count.ToUInt64() != length) throw new InvalidOperationException("Native text changed during read");
        return buffer.ToString();
    }
    public static void SetText(IntPtr window, string value) {
        if (value.Length > 1024) throw new ArgumentOutOfRangeException("value");
        UIntPtr result;
        if (SendText(window, 12, UIntPtr.Zero, value, 2, 2000, out result) == IntPtr.Zero || result == UIntPtr.Zero)
            throw new Win32Exception(Marshal.GetLastWin32Error(), "Bounded native text write failed");
    }
    public static bool Elevated(IntPtr process) {
        IntPtr token;
        if (!OpenProcessToken(process, 8, out token)) throw new Win32Exception(Marshal.GetLastWin32Error());
        try {
            int elevation, returned;
            if (!GetTokenInformation(token, 20, out elevation, 4, out returned) || returned != 4)
                throw new Win32Exception(Marshal.GetLastWin32Error());
            return elevation != 0;
        } finally {
            if (!CloseHandle(token)) throw new Win32Exception(Marshal.GetLastWin32Error());
        }
    }
    public static int TransparentGroupResolutions { get; private set; }
    private static bool MatchesMouseTarget(IntPtr expected, Point point) {
        IntPtr hit = WindowFromPoint(point);
        if (hit == expected) return true;
        IntPtr parent = GetParent(expected);
        if (parent == IntPtr.Zero || GetParent(hit) != parent || GetForegroundWindow() != parent) return false;
        uint expectedProcess, hitProcess, parentProcess;
        uint expectedThread = GetWindowThreadProcessId(expected, out expectedProcess);
        uint hitThread = GetWindowThreadProcessId(hit, out hitProcess);
        uint parentThread = GetWindowThreadProcessId(parent, out parentProcess);
        if (expectedThread == 0 || expectedThread != hitThread || expectedThread != parentThread ||
            expectedProcess != hitProcess || expectedProcess != parentProcess) return false;
        var className = new StringBuilder(64);
        if (GetClassName(hit, className, className.Capacity) == 0 ||
            !String.Equals(className.ToString(), "Button", StringComparison.OrdinalIgnoreCase) ||
            (GetWindowLongPtr(hit, -16).ToInt64() & 15) != 7) return false;
        // Cross-thread WindowFromPoint treats a group box as opaque. Resolve only
        // this verified owned sibling's transparent interior, never another app.
        Point local = point;
        if (!ScreenToClient(parent, ref local)) throw new Win32Exception(Marshal.GetLastWin32Error());
        if (RealChildWindowFromPoint(parent, local) != expected) return false;
        TransparentGroupResolutions++;
        return true;
    }
    private static void MouseAt(IntPtr expected, Point point) {
        if (!MatchesMouseTarget(expected, point)) {
            IntPtr hit = WindowFromPoint(point);
            throw new InvalidOperationException("Another window covers the target; refusing mouse input. Expected control=" +
                GetDlgCtrlID(expected) + ", hit control=" + GetDlgCtrlID(hit) +
                ", same parent=" + (GetParent(expected) == GetParent(hit)));
        }
        if (!SetCursorPos(point.X, point.Y)) throw new Win32Exception(Marshal.GetLastWin32Error());
        if (!MatchesMouseTarget(expected, point)) throw new InvalidOperationException("Desktop ownership changed; refusing mouse input");
        var events = new[] { new Input { mouse = new MouseInput { dwFlags = 2 } }, new Input { mouse = new MouseInput { dwFlags = 4 } } };
        uint inserted = SendInput(2, events, Marshal.SizeOf(typeof(Input)));
        if (inserted != 2) {
            int error = Marshal.GetLastWin32Error();
            if (inserted == 1) {
                // Balance only our successfully inserted down event. Do not
                // leave a mouse button held after a partial native insertion.
                var release = new[] { new Input { mouse = new MouseInput { dwFlags = 4 } } };
                uint released = SendInput(1, release, Marshal.SizeOf(typeof(Input)));
                throw new Win32Exception(error, "Owned mouse input partial; cleanup button-up count=" + released);
            }
            throw new Win32Exception(error, "Owned mouse input incomplete");
        }
    }
    public static void ActivateOwned(IntPtr owner) {
        SetForegroundWindow(owner);
        if (GetForegroundWindow() == owner) return;
        // Foreground locks may reject programmatic activation. Temporarily raise
        // only our fresh fixture HWND, click its inert lower-left client area,
        // then always restore ordinary Z-order. No foreign-window input or global
        // focus setting is changed. A locked/unavailable desktop still fails.
        // ASYNCWINDOWPOS avoids waiting indefinitely on another UI thread.
        if (!SetWindowPos(owner, new IntPtr(-1), 0, 0, 0, 0, 0x4013))
            throw new Win32Exception(Marshal.GetLastWin32Error(), "Owned window could not be raised");
        try {
            Rect rectangle;
            if (!GetWindowRect(owner, out rectangle)) throw new Win32Exception(Marshal.GetLastWin32Error());
            Point point = new Point { X=rectangle.Left + 40, Y=rectangle.Bottom - 40 };
            for (int attempt=0; attempt<20 && ((GetWindowLongPtr(owner, -20).ToInt64() & 8)==0 || WindowFromPoint(point)!=owner); attempt++)
                System.Threading.Thread.Sleep(25);
            if ((GetWindowLongPtr(owner, -20).ToInt64() & 8)==0)
                throw new InvalidOperationException("Owned topmost transition was not acknowledged");
            MouseAt(owner, point);
            for (int attempt=0; attempt<20 && GetForegroundWindow()!=owner; attempt++)
                System.Threading.Thread.Sleep(25);
            if (GetForegroundWindow()!=owner) throw new InvalidOperationException("Interactive desktop did not activate the owned window");
        } finally {
            if (!SetWindowPos(owner, new IntPtr(-2), 0, 0, 0, 0, 0x4013))
                throw new Win32Exception(Marshal.GetLastWin32Error(), "Owned ordinary Z-order restoration failed");
            for (int attempt=0; attempt<20 && (GetWindowLongPtr(owner, -20).ToInt64() & 8)!=0; attempt++)
                System.Threading.Thread.Sleep(25);
            if ((GetWindowLongPtr(owner, -20).ToInt64() & 8)!=0)
                throw new InvalidOperationException("Owned window remained topmost after bounded restoration");
        }
    }
    public static Rect ResizeOwned(IntPtr owner, int width, int height) {
        if (width < 1 || height < 1 || width > 8000 || height > 8000)
            throw new ArgumentOutOfRangeException("width");
        if (!SetWindowPos(owner, IntPtr.Zero, 0, 0, width, height, 0x4006))
            throw new Win32Exception(Marshal.GetLastWin32Error(), "Owned resize failed");
        for (int attempt=0; attempt<40; attempt++) {
            Rect rectangle;
            if (!GetWindowRect(owner, out rectangle)) throw new Win32Exception(Marshal.GetLastWin32Error());
            if (rectangle.Right-rectangle.Left==width && rectangle.Bottom-rectangle.Top==height)
                return rectangle;
            System.Threading.Thread.Sleep(25);
        }
        throw new InvalidOperationException("Owned resize did not complete within its budget");
    }
    public static void Click(IntPtr owner, IntPtr control, int row) {
        if (!IsWindowVisible(control) || !IsWindowEnabled(control)) throw new InvalidOperationException("Control not interactable");
        ActivateOwned(owner);
        if (GetForegroundWindow() != owner) throw new InvalidOperationException("Owned window is not foreground; refusing mouse input");
        Rect rectangle;
        if (!GetWindowRect(control, out rectangle)) throw new Win32Exception(Marshal.GetLastWin32Error());
        int x = rectangle.Left + (rectangle.Right - rectangle.Left) / 2;
        int y = rectangle.Top + (rectangle.Bottom - rectangle.Top) / 2;
        if (row >= 0) {
            ulong itemHeight = Number(control, 0x1a1, 0);
            if (itemHeight == 0 || itemHeight > 100) throw new InvalidOperationException("Unexpected list row height");
            y = checked(rectangle.Top + 2 + (int)itemHeight * row + (int)itemHeight / 2);
        }
        Point point = new Point { X=x, Y=y };
        if (GetForegroundWindow() != owner)
            throw new InvalidOperationException("Desktop ownership changed; refusing mouse input");
        MouseAt(control, point);
    }
}
'@

# This affects only the diagnostic process's coordinate system, not the UI or OS.
$observerDpiSet = [NevroTickM0Observation]::SetProcessDPIAware()
$observerAwareness = -1
$observerProcess = [Diagnostics.Process]::GetCurrentProcess()
try {
    if ([NevroTickM0Observation]::GetProcessDpiAwareness($observerProcess.Handle, [ref]$observerAwareness) -ne 0 -or
        $observerAwareness -notin @(1, 2)) {
        throw 'The observer must have an effective DPI-aware coordinate context.'
    }
} finally { $observerProcess.Dispose() }
$originalCursor = [NevroTickM0Observation+Point]::new()
$haveCursor = [NevroTickM0Observation]::GetCursorPos([ref]$originalCursor)
$ownedProcess = $null
$deadline = [DateTime]::UtcNow.AddSeconds(120)
$checks = [Collections.Generic.List[string]]::new()
$resourceSamples = [Collections.Generic.List[object]]::new()
$record = [ordered]@{
    scope = 'UI-M0 native controls; D1/D2/D3 Deferred; no leak/soak qualification'
    design_reference = 'docs/nevrotick-design-v0.2; scoped native adaptation'
    image_sha256 = $initialHash
    helper_path = $helperPath
    helper_sha256 = $helperHash
    windows_version = [Environment]::OSVersion.Version.ToString()
    observer_elevated = $false
    observer_dpi_setter_succeeded = $observerDpiSet
    observer_dpi_awareness = $observerAwareness
    observed = $false
    checks = $checks
    resource_samples = $resourceSamples
}

function Assert-Observation([bool]$Condition, [string]$Message) {
    if ([DateTime]::UtcNow -gt $deadline) { throw 'Observation wall-time budget exceeded.' }
    if (-not $Condition) { throw $Message }
}

function Get-Control([IntPtr]$Window, [int]$Identifier) {
    $control = [NevroTickM0Observation]::GetDlgItem($Window, $Identifier)
    Assert-Observation ($control -ne [IntPtr]::Zero) "Missing control $Identifier."
    return $control
}

function Click-Control([IntPtr]$Window, [int]$Identifier, [int]$Row = -1) {
    Assert-Observation (-not $ownedProcess.HasExited) 'Candidate exited before interaction.'
    [NevroTickM0Observation]::Click($Window, (Get-Control $Window $Identifier), $Row)
    Start-Sleep -Milliseconds 90
    if ($Identifier -in @(101, 102, 103)) {
        $pageControls = @{101 = 110; 102 = 201; 103 = 301}
        foreach ($entry in $pageControls.GetEnumerator()) {
            $visible = [NevroTickM0Observation]::IsWindowVisible((Get-Control $Window $entry.Value))
            Assert-Observation ($visible -eq ($entry.Key -eq $Identifier)) 'Native page did not change as requested.'
        }
        Assert-Observation ([NevroTickM0Observation]::IsWindowVisible((Get-Control $Window 2))) 'Page navigation hid the sample label.'
    }
}

function Get-ControlText([IntPtr]$Window, [int]$Identifier) {
    return [NevroTickM0Observation]::Text((Get-Control $Window $Identifier))
}

function Set-ControlText([IntPtr]$Window, [int]$Identifier, [string]$Value) {
    [NevroTickM0Observation]::SetText((Get-Control $Window $Identifier), $Value)
}

function Start-OwnedCandidate {
    $start = [Diagnostics.ProcessStartInfo]::new($image.FullName)
    $start.UseShellExecute = $false
    $start.WorkingDirectory = $image.DirectoryName
    $process = [Diagnostics.Process]::Start($start)
    $script:ownedProcess = $process
    Assert-Observation ($null -ne $process) 'Candidate did not start.'
    Assert-Observation ($process.WaitForInputIdle(10000)) 'Candidate did not enter its native message loop.'
    for ($attempt = 0; $attempt -lt 40; $attempt++) {
        $process.Refresh()
        if ($process.MainWindowHandle -ne [IntPtr]::Zero) { break }
        Assert-Observation (-not $process.HasExited) 'Candidate exited during startup.'
        Start-Sleep -Milliseconds 100
    }
    Assert-Observation ($process.MainWindowHandle -ne [IntPtr]::Zero) 'Candidate has no native window.'
    Assert-Observation (-not [NevroTickM0Observation]::Elevated($process.Handle)) 'Candidate unexpectedly runs elevated.'
    Assert-Observation ($process.MainModule.FileName -eq $image.FullName) 'Observed executable differs from admitted image.'
    $banner = [NevroTickM0Observation]::Text((Get-Control $process.MainWindowHandle 2))
    $record.caption_control_banner_valid = $banner.Contains('Prototype') -and $banner.Contains('sample data')
    Assert-Observation ($record.caption_control_banner_valid) 'Healthy sample-label control failed before caption observation.'
    $caption = [NevroTickM0Observation]::Text($process.MainWindowHandle)
    $record.window_caption = $caption
    Assert-Observation ($caption.Contains('NevroTick') -and $caption.Contains('Prototype') -and $caption.Contains('sample data')) 'Native window caption lost its application and sample context.'
    return $process.MainWindowHandle
}

function Save-OwnedWindow([IntPtr]$Window, [string]$Name) {
    [NevroTickM0Observation]::ActivateOwned($Window)
    Start-Sleep -Milliseconds 150
    Assert-Observation ([NevroTickM0Observation]::GetForegroundWindow() -eq $Window) 'Owned window lost foreground before capture.'
    $rectangle = [NevroTickM0Observation+Rect]::new()
    Assert-Observation ([NevroTickM0Observation]::GetWindowRect($Window, [ref]$rectangle)) 'Window rectangle unavailable.'
    $width = $rectangle.Right - $rectangle.Left
    $height = $rectangle.Bottom - $rectangle.Top
    Assert-Observation ($width -gt 0 -and $height -gt 0 -and $width -le 8000 -and $height -le 8000) 'Capture dimensions exceed budget.'
    $bitmap = [Drawing.Bitmap]::new($width, $height)
    try {
        $graphics = [Drawing.Graphics]::FromImage($bitmap)
        try { $graphics.CopyFromScreen($rectangle.Left, $rectangle.Top, 0, 0, $bitmap.Size) }
        finally { $graphics.Dispose() }
        $bitmap.Save((Join-Path $destination "$Name.png"), [Drawing.Imaging.ImageFormat]::Png)
    } finally { $bitmap.Dispose() }
}

function Read-Resources([int]$Cycle) {
    $ownedProcess.Refresh()
    Assert-Observation (-not $ownedProcess.HasExited) 'Candidate exited before resource observation.'
    return [ordered]@{
        cycle = $Cycle
        handles = $ownedProcess.HandleCount
        gdi_objects = [NevroTickM0Observation]::GuiResources($ownedProcess.Handle, 0)
        user_objects = [NevroTickM0Observation]::GuiResources($ownedProcess.Handle, 1)
        private_bytes = $ownedProcess.PrivateMemorySize64
    }
}

function Assert-VisibleLayout([IntPtr]$Window) {
    $outer = [NevroTickM0Observation+Rect]::new()
    Assert-Observation ([NevroTickM0Observation]::GetClientRect($Window, [ref]$outer)) 'Parent client rectangle unavailable.'
    $origin = [NevroTickM0Observation+Point]::new()
    Assert-Observation ([NevroTickM0Observation]::ClientToScreen($Window, [ref]$origin)) 'Client screen origin unavailable.'
    $outer.Left += $origin.X
    $outer.Right += $origin.X
    $outer.Top += $origin.Y
    $outer.Bottom += $origin.Y
    foreach ($identifier in @(1, 2, 101, 102, 103, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
        201, 202, 203, 204, 205, 206, 207, 208, 209, 211, 212, 213, 220, 221, 230, 231,
        301, 302, 303, 304, 305, 306, 307, 308, 309, 310)) {
        $control = Get-Control $Window $identifier
        if ([NevroTickM0Observation]::IsWindowVisible($control)) {
            $rectangle = [NevroTickM0Observation+Rect]::new()
            Assert-Observation ([NevroTickM0Observation]::GetWindowRect($control, [ref]$rectangle)) 'Child layout rectangle unavailable.'
            Assert-Observation ($rectangle.Right -gt $rectangle.Left -and $rectangle.Bottom -gt $rectangle.Top) "Visible control $identifier has no area."
            Assert-Observation ($rectangle.Left -ge $outer.Left -and $rectangle.Top -ge $outer.Top -and $rectangle.Right -le $outer.Right -and $rectangle.Bottom -le $outer.Bottom) "Visible control $identifier lies outside the window."
        }
    }
}

function Get-ControlRectangle([IntPtr]$Window, [int]$Identifier) {
    $rectangle = [NevroTickM0Observation+Rect]::new()
    Assert-Observation ([NevroTickM0Observation]::GetWindowRect((Get-Control $Window $Identifier), [ref]$rectangle)) 'Design region rectangle unavailable.'
    return $rectangle
}

function Assert-RegionOrder([IntPtr]$Window, [int]$Upper, [int]$Lower) {
    $first = Get-ControlRectangle $Window $Upper
    $second = Get-ControlRectangle $Window $Lower
    Assert-Observation ($first.Bottom -le $second.Top) "Design regions $Upper and $Lower overlap or reverse their reading order."
}

function Assert-DesignPage([IntPtr]$Window, [ValidateSet('Status', 'Sources', 'Settings')][string]$Page) {
    $required = @(1, 2, 101, 102, 103)
    switch ($Page) {
        'Status' {
            $required += @(110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122)
            Assert-Observation ((Get-ControlText $Window 110) -eq 'Clock status') 'Clock status heading is absent.'
            Assert-Observation ((Get-ControlText $Window 113) -match '(?=.*NTS only)(?=.*sample)') 'Sample NTS profile is absent.'
            Assert-Observation ((Get-ControlText $Window 114) -match '(?i)sample') 'Status headline lost sample context.'
            $offset = Get-ControlText $Window 117
            Assert-Observation ($offset -match '\+0\.42\s*ms' -and $offset -match '(?i)behind') 'Offset value, unit or direction is incorrect.'
            Assert-Observation ((Get-ControlText $Window 119) -match '\u00b11\.8\s*ms') 'Separate uncertainty value or unit is absent.'
            Assert-Observation ((Get-ControlText $Window 121) -match '(?s)Authentication.*NTS.*agreement.*3') 'Sample authentication and agreement are not presented together.'
            Assert-RegionOrder $Window 113 114
            Assert-RegionOrder $Window 114 120
            Assert-RegionOrder $Window 120 116
            $offsetRegion = Get-ControlRectangle $Window 116
            $uncertaintyRegion = Get-ControlRectangle $Window 118
            Assert-Observation ($offsetRegion.Right -le $uncertaintyRegion.Left -and
                [Math]::Abs($offsetRegion.Top - $uncertaintyRegion.Top) -le 2) 'Offset and uncertainty regions are not separate side-by-side regions.'
            Assert-RegionOrder $Window 116 111
        }
        'Sources' {
            $required += @(201, 202, 203, 204, 205, 206, 207, 208, 209, 211, 212, 213, 220, 221, 230, 231)
            Assert-Observation ((Get-ControlText $Window 207) -eq 'Time servers') 'Time servers heading is absent.'
            Assert-Observation ((Get-ControlText $Window 208) -match '(?=.*NTS only)(?=.*sample)') 'Sources sample profile is absent.'
            Assert-Observation ((Get-ControlText $Window 202).Contains((Get-ControlText $Window 212))) 'Details do not identify the selected editor host.'
            Assert-Observation ((Get-ControlText $Window 231) -match '(?i)(memory|discard)') 'Local preview lifetime is not explained.'
            Assert-RegionOrder $Window 208 201
            Assert-RegionOrder $Window 201 209
            $detailsRegion = Get-ControlRectangle $Window 209
            $editorRegion = Get-ControlRectangle $Window 203
            Assert-Observation ($detailsRegion.Right -le $editorRegion.Left) 'Source details and editor regions overlap.'
        }
        'Settings' {
            $required += @(301, 302, 303, 304, 305, 306, 307, 308, 309, 310)
            Assert-Observation ((Get-ControlText $Window 301) -eq 'Settings') 'Settings heading is absent.'
            Assert-Observation ((Get-ControlText $Window 304) -match 'Time sources.*trust') 'Time sources and trust group is absent.'
            Assert-Observation ((Get-ControlText $Window 306) -eq 'Clock correction') 'Clock correction group is absent.'
            $preferences = Get-ControlText $Window 308
            Assert-Observation ($preferences.StartsWith('Prototype preferences') -and $preferences -match 'read.only') 'Read-only prototype preferences group is absent.'
            Assert-Observation ((Get-ControlText $Window 305) -match '(?s)NTS.*UTC') 'Trust group lost sample profile/time scale.'
            Assert-RegionOrder $Window 304 306
            Assert-RegionOrder $Window 306 308
            Assert-RegionOrder $Window 308 302
        }
    }
    foreach ($identifier in $required) {
        Assert-Observation ([NevroTickM0Observation]::IsWindowVisible((Get-Control $Window $identifier))) "Required $Page control $identifier is hidden."
    }
    Assert-VisibleLayout $Window
}

function Close-OwnedCandidate {
    Assert-Observation ($ownedProcess.CloseMainWindow()) 'Normal Close request failed.'
    Assert-Observation ($ownedProcess.WaitForExit(5000)) 'Candidate did not exit after normal Close.'
    Assert-Observation ($ownedProcess.ExitCode -eq 0) 'Candidate returned a failing native exit.'
    $ownedProcess.Dispose()
    $script:ownedProcess = $null
}

try {
    $window = Start-OwnedCandidate
    $record.candidate_elevated = $false
    $record.system_dpi = [NevroTickM0Observation]::GetDpiForSystem()
    $record.window_dpi = [NevroTickM0Observation]::GetDpiForWindow($window)
    $awareness = 0
    Assert-Observation ([NevroTickM0Observation]::GetProcessDpiAwareness($ownedProcess.Handle, [ref]$awareness) -eq 0) 'DPI awareness observation failed.'
    $record.process_dpi_awareness = $awareness
    # Absent explicit preferences are recorded as null, never invented values.
    $record.text_scale_percent = [Microsoft.Win32.Registry]::GetValue('HKEY_CURRENT_USER\Software\Microsoft\Accessibility', 'TextScaleFactor', $null)
    $record.apps_use_light_theme = [Microsoft.Win32.Registry]::GetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize', 'AppsUseLightTheme', $null)
    Assert-Observation ((Get-ControlText $window 2) -match 'Prototype.*sample data') 'Persistent sample context is absent.'
    Assert-Observation (-not [NevroTickM0Observation]::IsWindowEnabled((Get-Control $window 111))) 'Sync now is enabled.'
    $record.clock_status_heading = Get-ControlText $window 110
    Assert-Observation ($record.clock_status_heading -eq 'Clock status') 'Design reference Clock status heading is absent.'
    Assert-DesignPage $window 'Status'
    Save-OwnedWindow $window '01-status'
    $checks.Add('unelevated launch, persistent sample label, disabled Sync now')

    $null = [NevroTickM0Observation]::ShowWindow($window, 6)
    Start-Sleep -Milliseconds 150
    Assert-Observation ([NevroTickM0Observation]::IsIconic($window)) 'Normal minimize failed.'
    $null = [NevroTickM0Observation]::ShowWindow($window, 9)
    Start-Sleep -Milliseconds 150
    Assert-Observation (-not [NevroTickM0Observation]::IsIconic($window)) 'Normal restore failed.'
    $checks.Add('native minimize and restore')

    Click-Control $window 122
    Assert-Observation ([NevroTickM0Observation]::IsWindowVisible((Get-Control $window 201)) -and
        -not [NevroTickM0Observation]::IsWindowVisible((Get-Control $window 110))) 'View sources did not navigate locally.'
    Assert-DesignPage $window 'Sources'
    $checks.Add('design hierarchy and separate sample metrics; local View sources navigation')
    $originalLabel = Get-ControlText $window 211
    $originalHost = Get-ControlText $window 212
    $originalDetails = Get-ControlText $window 202
    Set-ControlText $window 213 '0'
    Save-OwnedWindow $window '01b-sources-before-rejection'
    Click-Control $window 220
    Assert-Observation ((Get-ControlText $window 202) -eq $originalDetails) 'Invalid input changed accepted preview.'
    Assert-Observation ((Get-ControlText $window 230) -match '^Cannot update preview:') 'Invalid input has no visible error.'
    Assert-DesignPage $window 'Sources'
    Save-OwnedWindow $window '02-invalid-edit'
    $checks.Add('invalid port rejected; previous preview and visible error retained')

    Set-ControlText $window 211 'A&B sample'
    Set-ControlText $window 212 'preview.example.invalid'
    Set-ControlText $window 213 '4460'
    Click-Control $window 220
    $acceptedDetails = Get-ControlText $window 202
    Assert-Observation ($acceptedDetails.Contains('A&B sample') -and $acceptedDetails.Contains('preview.example.invalid:4460')) 'Valid neighboring edit was not displayed.'
    Assert-Observation ((Get-ControlText $window 230) -match '^Preview updated in memory only') 'Accepted edit is not labelled local.'
    Save-OwnedWindow $window '03-updated-preview'
    $checks.Add('valid ampersand-label edit updates only local preview; screenshot retains visible-text evidence')

    $originalRectangle = [NevroTickM0Observation+Rect]::new()
    Assert-Observation ([NevroTickM0Observation]::GetWindowRect($window, [ref]$originalRectangle)) 'Original size is unavailable.'
    $originalWidth = $originalRectangle.Right - $originalRectangle.Left
    $originalHeight = $originalRectangle.Bottom - $originalRectangle.Top
    try {
        $resized = [NevroTickM0Observation]::ResizeOwned($window, $originalWidth - 120, $originalHeight - 60)
        Start-Sleep -Milliseconds 150
        Assert-Observation (($resized.Right - $resized.Left) -lt $originalWidth -and ($resized.Bottom - $resized.Top) -lt $originalHeight) 'Owned window did not resize.'
        Assert-DesignPage $window 'Sources'
        Save-OwnedWindow $window '03b-resized-sources'
        Set-ControlText $window 213 '0'
        Click-Control $window 220
        Assert-Observation ((Get-ControlText $window 202) -eq $acceptedDetails) 'Invalid resized edit changed accepted preview.'
        Assert-Observation ((Get-ControlText $window 230) -match '^Cannot update preview:') 'Resized rejection feedback is absent.'
        Assert-DesignPage $window 'Sources'
        Save-OwnedWindow $window '03c-resized-invalid-edit'
        Click-Control $window 221
        Click-Control $window 101
        Assert-DesignPage $window 'Status'
        Save-OwnedWindow $window '03d-resized-status'
        Click-Control $window 103
        Assert-DesignPage $window 'Settings'
        Save-OwnedWindow $window '03e-resized-settings'
        Click-Control $window 102
        $record.resize_pixels = @($originalWidth, $originalHeight, ($resized.Right - $resized.Left), ($resized.Bottom - $resized.Top))
    } finally {
        $null = [NevroTickM0Observation]::ResizeOwned($window, $originalWidth, $originalHeight)
    }
    $checks.Add('all three design pages and rejection feedback fit the resized client area; original size restored')

    Set-ControlText $window 212 ('x' * 600)
    Click-Control $window 220
    Assert-Observation ((Get-ControlText $window 202) -eq $acceptedDetails) 'Programmatic oversized text changed preview.'
    Assert-Observation ((Get-ControlText $window 230) -match 'native text budget') 'Native input bypass did not reach independent read guard.'
    Click-Control $window 221
    Assert-Observation ((Get-ControlText $window 212) -eq 'preview.example.invalid') 'Cancel did not discard oversized edit.'
    Set-ControlText $window 211 'Unaccepted edit'
    Click-Control $window 221
    Assert-Observation ((Get-ControlText $window 211) -eq 'A&B sample') 'Cancel changed the accepted preview.'
    $checks.Add('WM_SETTEXT budget bypass rejected; Cancel discards unaccepted input')

    Click-Control $window 201 1
    Assert-Observation ((Get-ControlText $window 211) -ne 'A&B sample') 'Mouse source selection did not change details.'
    Click-Control $window 201 0
    Assert-Observation ((Get-ControlText $window 211) -eq 'A&B sample') 'Stable source ordering or accepted preview was lost.'
    Click-Control $window 103
    Assert-Observation (-not [NevroTickM0Observation]::IsWindowEnabled((Get-Control $window 302))) 'Machine policy action is enabled.'
    Assert-DesignPage $window 'Settings'
    Save-OwnedWindow $window '04-settings'
    $checks.Add('mouse source selection, stable rows, disabled machine policy')

    for ($warm = 0; $warm -lt 5; $warm++) {
        Click-Control $window 102
        Set-ControlText $window 211 "Warmup edit $warm"
        Click-Control $window 221
        Assert-Observation ((Get-ControlText $window 211) -eq 'A&B sample') 'Warmup Cancel failed.'
        Click-Control $window 103
        Click-Control $window 101
    }
    $resourceSamples.Add((Read-Resources 0))
    for ($cycle = 1; $cycle -le $Cycles; $cycle++) {
        Click-Control $window 102
        Set-ControlText $window 211 "Unaccepted cycle $cycle"
        Click-Control $window 221
        Assert-Observation ((Get-ControlText $window 211) -eq 'A&B sample') 'Repeated Cancel did not restore accepted state.'
        Click-Control $window 103
        Click-Control $window 101
        if (($cycle % 5) -eq 0 -or $cycle -eq $Cycles) { $resourceSamples.Add((Read-Resources $cycle)) }
    }
    $baseline = $resourceSamples[0]
    $last = $resourceSamples[$resourceSamples.Count - 1]
    Assert-Observation ($last.gdi_objects -eq $baseline.gdi_objects -and $last.user_objects -eq $baseline.user_objects -and $last.handles -eq $baseline.handles) 'Native resource counts grew after warmed repeated interactions.'
    $checks.Add("$Cycles warmed native navigation/cancel cycles; stable GDI, USER and handle counts before process exit")
    Close-OwnedCandidate
    $checks.Add('normal Close returns exit 0')

    $window = Start-OwnedCandidate
    Click-Control $window 102
    Assert-Observation ((Get-ControlText $window 211) -eq $originalLabel -and (Get-ControlText $window 212) -eq $originalHost) 'Restart retained preview edits.'
    Save-OwnedWindow $window '05-restarted-fixtures'
    Close-OwnedCandidate
    $checks.Add('fresh launch restores checked-in fixtures; second normal Close returns exit 0')
    Assert-Observation ((Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash -eq $initialHash) 'Candidate bytes changed during observation.'
    Assert-Observation ((Get-FileHash -LiteralPath $helperPath -Algorithm SHA256).Hash -eq $helperHash) 'Observation helper bytes changed during the run.'
    Assert-Observation ([NevroTickM0Observation]::TransparentGroupResolutions -gt 0) 'The owned transparent-group input path was not exercised.'
    $record.observed = $true
} catch {
    $record.failure = $_.Exception.Message
    throw
} finally {
    $record.transparent_group_hit_resolutions = [NevroTickM0Observation]::TransparentGroupResolutions
    if ($null -ne $ownedProcess) {
        if (-not $ownedProcess.HasExited) {
            $null = $ownedProcess.CloseMainWindow()
            if (-not $ownedProcess.WaitForExit(3000)) {
                $ownedProcess.Kill()
                $null = $ownedProcess.WaitForExit(3000)
                $record.forced_owned_cleanup = $true
            }
        }
        $ownedProcess.Dispose()
    }
    if ($haveCursor) { $null = [NevroTickM0Observation]::SetCursorPos($originalCursor.X, $originalCursor.Y) }
    $record | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $destination 'observations.json') -Encoding UTF8
}
$record | ConvertTo-Json -Depth 8
