#requires -Version 5.1
<#
Admit native tools/debugger settings or enable/disable owned per-image diagnostics. No application
launch, installer, global flags, PATH or execution-policy changes. Enable/Disable
need an elevated x64 Windows PowerShell. Run the candidate/debugger separately
with its intended token. Configuration persists until explicitly disabled.

This guards cooperative development operations, not malicious administrators or
arbitrary code already running as this administrator. Review this script/policy
before elevation and reserve exclusive configuration custody of the image name.
Interrupted native mutations retain a protected recovery record. Exact admitted
per-step state must match before it is treated as owned; unexpected values,
providers or changes block cleanup. Other administrators must still keep exclusive
image custody: this helper is not an operating-system-wide registry transaction.
#>
[CmdletBinding()]
param(
    [ValidateSet('Preflight', 'Enable', 'Disable')]
    [string]$Action = 'Preflight',
    [string]$ImagePath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
$script:ImagePattern = '^NevroTick-[a-f0-9]{12,64}\.exe$'
$script:IfeoPath = 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options'
$script:PolicyPath = Join-Path $PSScriptRoot '..\policy\windows-memory.json'

function Test-Elevated {
    $principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Assert-LocalPath {
    param([string]$Path)
    if ($Path -notmatch '^[A-Za-z]:\\' -or $Path.Substring(2).Contains(':') -or
        $Path -match '[\x00-\x1f"<>|?*]' -or $Path -match '(?:^|\\)[^\\]*[. ](?:\\|$)') {
        throw 'Expected an absolute local Windows path without aliases or stream syntax.'
    }
    $full = [IO.Path]::GetFullPath($Path)
    if ($full -cne $Path -and $full -ine $Path) { throw 'Path must already be canonical; relative segments are rejected.' }
    if ((New-Object IO.DriveInfo([IO.Path]::GetPathRoot($full))).DriveType -ne [IO.DriveType]::Fixed) {
        throw 'Only a local fixed drive is admitted.'
    }
    return $full
}

function Assert-NoReparse {
    param([string]$Path)
    $cursor = $Path
    while ($cursor) {
        if (Test-Path -LiteralPath $cursor) {
            if (((Get-Item -Force -LiteralPath $cursor).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
                throw "Reparse point is not admitted: $cursor"
            }
        }
        $parent = [IO.Path]::GetDirectoryName($cursor)
        if ($parent -eq $cursor) { break }
        $cursor = $parent
    }
}

function Get-ImageIdentity {
    param([string]$Path, [bool]$RequireFile)
    $full = Assert-LocalPath $Path
    Assert-NoReparse $full
    $name = [IO.Path]::GetFileName($full)
    if ($name -cnotmatch $script:ImagePattern) {
        throw 'Use a reviewed unique NevroTick-<12-64 lowercase hex>.exe image name.'
    }
    $digest = $null
    if ($RequireFile) {
        if (-not (Test-Path -LiteralPath $full -PathType Leaf)) { throw 'Candidate executable is missing.' }
        $stream = [IO.File]::OpenRead($full)
        try {
            $reader = New-Object IO.BinaryReader($stream)
            if ($stream.Length -lt 64 -or $reader.ReadUInt16() -ne 0x5a4d) { throw 'Candidate is not a PE executable.' }
            $stream.Position = 0x3c
            $offset = $reader.ReadUInt32()
            if ($offset -gt ($stream.Length - 6)) { throw 'Invalid PE header offset.' }
            $stream.Position = $offset
            if ($reader.ReadUInt32() -ne 0x4550 -or $reader.ReadUInt16() -ne 0x8664) {
                throw 'Only AMD64 PE candidates are admitted.'
            }
        } finally { $stream.Dispose() }
        $digest = (Get-FileHash -LiteralPath $full -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    return [ordered]@{ path = $full; name = $name; sha256 = $digest }
}

function Assert-ImageStopped {
    param([string]$Name)
    $base = [IO.Path]::GetFileNameWithoutExtension($Name)
    if (@(Get-Process -Name $base -ErrorAction SilentlyContinue).Count -ne 0) {
        throw 'Stop every process with the selected image name before changing persistent diagnostics.'
    }
}

function Assert-Policy {
    param($Policy)
    if ($Policy.schema -ne 1 -or $Policy.status -ne 'admitted' -or
        $Policy.profile.page_heap -ne 'full' -or
        (@($Policy.profile.application_verifier) -join ',') -ne 'Basics' -or
        $Policy.profile.image_name_pattern -cne $script:ImagePattern -or
        $Policy.profile.tool_timeout_seconds -ne 30) {
        throw 'Native-memory policy is unadmitted or changes the reviewed profile.'
    }
    if ($null -eq $Policy.PSObject.Properties['debugger_config_sha256'] -or
        $Policy.debugger_config_sha256 -cnotmatch '^[a-f0-9]{64}$') {
        throw 'Missing exact debugger configuration admission.'
    }
    if ($null -eq $Policy.PSObject.Properties['debugger_config_location'] -or
        $Policy.debugger_config_location -cne 'current-user:LocalApplicationData/DBG/config.xml') {
        throw 'Debugger configuration must use the current user KnownFolder owner.'
    }
    foreach ($name in @('appverif', 'gflags', 'cdb')) {
        $tool = $Policy.tools.$name
        if (-not $tool.path -or $tool.sha256 -cnotmatch '^[a-f0-9]{64}$' -or
            -not $tool.file_version -or -not $tool.signer_subject) {
            throw "Missing exact native-tool admission: $name"
        }
        $path = Assert-LocalPath $tool.path
        if ([IO.Path]::GetFileName($path) -ine ($name + '.exe')) { throw 'Tool basename mismatch.' }
    }
    $libraries = @($Policy.supporting_libraries | ForEach-Object { [IO.Path]::GetFileName($_.path) } | Sort-Object)
    if (($libraries -join ',') -cne 'dbgcore.dll,dbgeng.dll,dbghelp.dll,vfbasics.dll,vfcompat.dll,vrfcore.dll') {
        throw 'Required debugger/verifier library admission is incomplete.'
    }
    if (@($Policy.profile.registry_provider_keys).Count -ne 10 -or
        (@($Policy.profile.registry_after_step.PSObject.Properties.Name | Sort-Object) -join ',') -cne
        'disable-appverif,disable-pageheap,enable-appverif,enable-pageheap') {
        throw 'Exact native registry profile admission is incomplete.'
    }
}

function Assert-DebuggerConfiguration {
    param([string]$Path, [string]$ExpectedHash)
    $null = Assert-LocalPath $Path
    Assert-NoReparse $Path
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw 'Missing debugger config.xml; run the reviewed ConfigureOnly setup first.'
    }
    if ($ExpectedHash -cnotmatch '^[a-f0-9]{64}$' -or
        (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant() -cne $ExpectedHash) {
        throw 'Installed debugger configuration differs from the admitted offline settings.'
    }
}

function Get-UserDebuggerConfigurationPath {
    # CDB's effective settings belong to the account running the debugger, not
    # the SDK directory or the administrator configuring per-image diagnostics.
    return (Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'DBG\config.xml')
}

function Get-AdmittedTools {
    param([bool]$RequireDebuggerConfiguration = $true)
    $policy = Get-Content -Raw -LiteralPath $script:PolicyPath | ConvertFrom-Json
    Assert-Policy $policy
    $script:RegistryProfile = $policy.profile
    $result = @{}
    $payloads = @('appverif', 'gflags', 'cdb') | ForEach-Object { $policy.tools.$_ }
    $payloads += @($policy.supporting_libraries)
    foreach ($tool in $payloads) {
        $name = [IO.Path]::GetFileNameWithoutExtension($tool.path)
        if ($tool.sha256 -cnotmatch '^[a-f0-9]{64}$' -or -not $tool.file_version -or -not $tool.signer_subject) {
            throw "Missing exact native payload admission: $name"
        }
        $null = Assert-LocalPath $tool.path
        Assert-NoReparse $tool.path
        if (-not (Test-Path -LiteralPath $tool.path -PathType Leaf)) { throw "Missing tool: $name" }
        if ((Get-FileHash -LiteralPath $tool.path -Algorithm SHA256).Hash.ToLowerInvariant() -cne $tool.sha256) {
            throw "Native tool hash changed: $name"
        }
        $signature = Get-AuthenticodeSignature -LiteralPath $tool.path
        if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -cne $tool.signer_subject -or
            $signature.SignerCertificate.Subject -notmatch '(^|, )O=Microsoft Corporation(,|$)') {
            throw "Native tool signature is not the admitted Microsoft signature: $name"
        }
        if ((Get-Item -LiteralPath $tool.path).VersionInfo.FileVersion -cne $tool.file_version) {
            throw "Native tool version changed: $name"
        }
        if ($name -in @('appverif', 'gflags', 'cdb')) { $result[$name] = $tool.path }
    }
    if ($RequireDebuggerConfiguration) {
        Assert-DebuggerConfiguration (Get-UserDebuggerConfigurationPath) $policy.debugger_config_sha256
    }
    return $result
}

function Read-RegistryNode {
    param([Microsoft.Win32.RegistryKey]$Key)
    if ($null -eq $Key) { return [ordered]@{ exists = $false; values = @(); children = @() } }
    $values = @()
    foreach ($name in @($Key.GetValueNames() | Sort-Object)) {
        $kind = $Key.GetValueKind($name).ToString()
        $data = $Key.GetValue($name, $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
        if ($data -is [byte[]]) { $data = [Convert]::ToBase64String($data) }
        $values += [ordered]@{ name = $name; kind = $kind; data = $data }
    }
    $children = @()
    foreach ($name in @($Key.GetSubKeyNames() | Sort-Object)) {
        $child = $Key.OpenSubKey($name, $false)
        try { $children += [ordered]@{ name = $name; node = (Read-RegistryNode $child) } }
        finally { if ($null -ne $child) { $child.Dispose() } }
    }
    return [ordered]@{ exists = $true; values = $values; children = $children }
}

function Get-ImageState {
    param([string]$Name)
    $state = [ordered]@{}
    foreach ($view in @('Registry64', 'Registry32')) {
        $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine, $view)
        $key = $null
        try {
            $key = $base.OpenSubKey(($script:IfeoPath + '\' + $Name), $false)
            $state[$view] = Read-RegistryNode $key
        } finally { if ($null -ne $key) { $key.Dispose() }; $base.Dispose() }
    }
    return $state
}

function Convert-State {
    param($State)
    return ($State | ConvertTo-Json -Depth 32 -Compress)
}

function Assert-StateMatch {
    param($Actual, $Expected)
    if ((Convert-State $Actual) -cne (Convert-State $Expected)) {
        throw 'Per-image state changed outside the recorded step. Stop and inspect; no cleanup was attempted.'
    }
}

function Assert-InitiallyAbsent {
    param($State)
    if ($State.Registry64.exists -or $State.Registry32.exists) {
        throw 'Pre-existing image settings are not owned by this workflow and will not be overwritten.'
    }
}

function Get-ExpectedStepState {
    param([string]$Step)
    $profile = $script:RegistryProfile.registry_after_step.$Step
    $children = @()
    if ($profile.provider_children) {
        foreach ($name in $script:RegistryProfile.registry_provider_keys) {
            $children += [ordered]@{ name = $name; node = [ordered]@{ exists = $true; values = @(); children = @() } }
        }
    }
    $values = @($profile.values | ForEach-Object { [ordered]@{ name = $_.name; kind = $_.kind; data = $_.data } })
    $node = [ordered]@{ exists = $profile.exists; values = $values; children = $children }
    return [ordered]@{ Registry64 = $node; Registry32 = $node }
}

function Assert-StepState {
    param([string]$Step, $State)
    $expected = Get-ExpectedStepState $Step
    if ((Convert-State $State) -cne (Convert-State $expected)) {
        throw ('Native step produced an unadmitted registry state; it was not adopted for cleanup. Step: ' +
            $Step + '; observed: ' + (Convert-State $State))
    }
}

function Assert-ProtectedAcl {
    param([string]$Path, [bool]$RequireProtected = $true)
    Assert-NoReparse $Path
    $acl = Get-Acl -LiteralPath $Path
    $owner = $acl.GetOwner([Security.Principal.SecurityIdentifier]).Value
    $trusted = @('S-1-5-18', 'S-1-5-32-544')
    if ($owner -notin $trusted -or ($RequireProtected -and -not $acl.AreAccessRulesProtected)) {
        throw 'Custody must have an Administrator/SYSTEM owner and protected permissions.'
    }
    foreach ($rule in $acl.GetAccessRules($true, $true, [Security.Principal.SecurityIdentifier])) {
        if ($rule.AccessControlType -eq [Security.AccessControl.AccessControlType]::Allow -and
            $rule.IdentityReference.Value -notin $trusted) {
            throw 'Custody grants access to an untrusted identity.'
        }
    }
}

function Get-CustodyDirectory {
    param([bool]$Create)
    $root = Assert-LocalPath (Join-Path ([Environment]::GetFolderPath('CommonApplicationData')) 'NevroTickWindowsMemory')
    Assert-NoReparse $root
    if (-not (Test-Path -LiteralPath $root)) {
        if (-not $Create) { throw 'No owned native-memory configuration directory exists.' }
        $acl = New-Object Security.AccessControl.DirectorySecurity
        $acl.SetAccessRuleProtection($true, $false)
        $admin = New-Object Security.Principal.SecurityIdentifier('S-1-5-32-544')
        $acl.SetOwner($admin)
        foreach ($sid in @($admin, (New-Object Security.Principal.SecurityIdentifier('S-1-5-18')))) {
            $rule = New-Object Security.AccessControl.FileSystemAccessRule($sid, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
            $acl.AddAccessRule($rule)
        }
        [IO.Directory]::CreateDirectory($root, $acl) | Out-Null
    }
    if (-not (Test-Path -LiteralPath $root -PathType Container)) { throw 'Custody path is not a directory.' }
    Assert-ProtectedAcl $root
    return $root
}

function New-OwnedFileSecurity {
    $acl = New-Object Security.AccessControl.FileSecurity
    $acl.SetAccessRuleProtection($true, $false)
    $admin = New-Object Security.Principal.SecurityIdentifier('S-1-5-32-544')
    $acl.SetOwner($admin)
    foreach ($sid in @($admin, (New-Object Security.Principal.SecurityIdentifier('S-1-5-18')))) {
        $acl.AddAccessRule((New-Object Security.AccessControl.FileSystemAccessRule($sid, 'FullControl', 'Allow')))
    }
    return $acl
}

function Protect-OwnedFile {
    param([string]$Path)
    Set-Acl -LiteralPath $Path -AclObject (New-OwnedFileSecurity)
}

function Open-CustodyLock {
    param([string]$Path, [Security.AccessControl.FileSecurity]$Security)
    # FileAccess.ReadWrite does not request WRITE_DAC/WRITE_OWNER. Request the
    # explicit rights needed to establish/verify custody on this held handle.
    $stream = New-Object IO.FileStream($Path, [IO.FileMode]::OpenOrCreate,
        [Security.AccessControl.FileSystemRights]::FullControl, [IO.FileShare]::None,
        4096, [IO.FileOptions]::None, $Security)
    try { $stream.SetAccessControl($Security) }
    catch { $stream.Dispose(); throw }
    return $stream
}

function Write-Receipt {
    param([string]$Path, $Receipt)
    $temporary = $Path + '.' + [Guid]::NewGuid().ToString('N') + '.tmp'
    $stream = [IO.File]::Open($temporary, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
    try {
        $bytes = [Text.Encoding]::UTF8.GetBytes(($Receipt | ConvertTo-Json -Depth 40))
        $stream.Write($bytes, 0, $bytes.Length)
        $stream.Flush($true)
    } finally { $stream.Dispose() }
    Protect-OwnedFile $temporary
    if (Test-Path -LiteralPath $Path) { [IO.File]::Replace($temporary, $Path, [NullString]::Value) }
    else { [IO.File]::Move($temporary, $Path) }
}

function Invoke-BoundedTool {
    param([string]$Path, [string[]]$Arguments)
    foreach ($argument in $Arguments) {
        if ($argument -notmatch '^[A-Za-z0-9_./-]+$') { throw 'Unexpected native tool argument.' }
    }
    $start = New-Object Diagnostics.ProcessStartInfo
    $start.FileName = $Path
    $start.Arguments = $Arguments -join ' '
    $start.WorkingDirectory = [IO.Path]::GetDirectoryName($Path)
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    $process = New-Object Diagnostics.Process
    $process.StartInfo = $start
    try {
        if (-not $process.Start()) { throw 'Could not start admitted native tool.' }
        $stdout = $process.StandardOutput.ReadToEndAsync()
        $stderr = $process.StandardError.ReadToEndAsync()
        if (-not $process.WaitForExit(30000)) {
            $process.Kill()
            if (-not $process.WaitForExit(5000)) { throw 'Native configuration process could not be stopped; inspect custody.' }
            throw 'Native configuration command exceeded 30 seconds; inspect custody before recovery.'
        }
        if (-not [Threading.Tasks.Task]::WaitAll([Threading.Tasks.Task[]]@($stdout, $stderr), 5000)) {
            throw 'Native output pipes did not close within five seconds; inspect custody.'
        }
        $output = $stdout.Result + $stderr.Result
        # This limits accepted evidence after collection, not live allocation.
        if ($output.Length -gt 65536) { throw 'Native configuration output exceeded the admitted bound.' }
        return [ordered]@{ exit = $process.ExitCode; output = $output }
    } finally { $process.Dispose() }
}

function Invoke-OwnedStep {
    param([string]$ReceiptPath, $Receipt, [string]$Step, [string]$Tool, [string[]]$Arguments)
    Assert-StateMatch (Get-ImageState $Receipt.image.name) $Receipt.expected
    $Receipt.status = 'pending-' + $Step
    Write-Receipt $ReceiptPath $Receipt
    # An interruption here leaves a pending record. A new invocation will refuse
    # automatic deletion unless it can match the last observed state exactly.
    $result = Invoke-BoundedTool $Tool $Arguments
    $observed = Get-ImageState $Receipt.image.name
    $Receipt.status = 'observed-unadmitted-' + $Step
    $Receipt.steps += [ordered]@{ step = $Step; exit = $result.exit; output = $result.output; observed_state = $observed }
    Write-Receipt $ReceiptPath $Receipt
    Assert-StepState $Step $observed
    $Receipt.expected = $observed
    $Receipt.status = 'observed-' + $Step
    Write-Receipt $ReceiptPath $Receipt
    if ($result.exit -ne 0) { throw "Native step $Step exited $($result.exit); retained recovery record: $ReceiptPath" }
}

function Remove-OwnedEmptyKeys {
    param([string]$Name)
    foreach ($view in @('Registry64', 'Registry32')) {
        $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine, $view)
        $parent = $null; $key = $null
        try {
            $parent = $base.OpenSubKey($script:IfeoPath, $true)
            $key = $parent.OpenSubKey($Name, $false)
            if ($null -ne $key) {
                if ($key.ValueCount -ne 0 -or $key.SubKeyCount -ne 0) { throw 'Cleanup left nonempty image settings; retained custody.' }
                $key.Dispose(); $key = $null
                $parent.DeleteSubKey($Name, $false)
            }
        } finally {
            if ($null -ne $key) { $key.Dispose() }
            if ($null -ne $parent) { $parent.Dispose() }
            $base.Dispose()
        }
    }
}

function Disable-OwnedProfile {
    param([string]$ReceiptPath, $Receipt, $Tools)
    Assert-StateMatch (Get-ImageState $Receipt.image.name) $Receipt.expected
    if (-not $Receipt.expected.Registry64.exists -and -not $Receipt.expected.Registry32.exists) {
        $Receipt.status = 'disabled'
        Write-Receipt $ReceiptPath $Receipt
        return
    }
    Invoke-OwnedStep $ReceiptPath $Receipt 'disable-pageheap' $Tools.gflags @('/p', '/disable', $Receipt.image.name)
    Invoke-OwnedStep $ReceiptPath $Receipt 'disable-appverif' $Tools.appverif @('-delete', 'settings', '-for', $Receipt.image.name)
    Assert-StateMatch (Get-ImageState $Receipt.image.name) $Receipt.expected
    $Receipt.status = 'pending-empty-key-cleanup'
    Write-Receipt $ReceiptPath $Receipt
    Remove-OwnedEmptyKeys $Receipt.image.name
    $Receipt.expected = Get-ImageState $Receipt.image.name
    Assert-InitiallyAbsent $Receipt.expected
    $Receipt.status = 'disabled'
    Write-Receipt $ReceiptPath $Receipt
}

function Assert-EnabledProfile {
    param($State)
    $node = $State.Registry64
    $flags = @($node.values | Where-Object { $_.name -eq 'GlobalFlag' })
    $pageheap = @($node.values | Where-Object { $_.name -eq 'PageHeapFlags' })
    $verifier = @($node.values | Where-Object { $_.name -eq 'VerifierFlags' })
    if (-not $node.exists -or $flags.Count -ne 1 -or $pageheap.Count -ne 1 -or $verifier.Count -ne 1) {
        throw ('Expected per-image verifier/PageHeap configuration is missing. Observed: ' + (Convert-State $State))
    }
    # REG_SZ GlobalFlag is hexadecimal; DWORD values are already numeric.
    $global = Convert-RegistryDword $flags[0]
    $heap = Convert-RegistryDword $pageheap[0]
    $checks = Convert-RegistryDword $verifier[0]
    # This SDK uses GlobalFlag=0x100, PageHeapFlags=0x3 and its full Basics
    # VerifierFlags/provider set. Do not invent an extra legacy GlobalFlag bit.
    try { Assert-StepState 'enable-pageheap' $State }
    catch { throw ('Exact full PageHeap/Basics profile was not established. GlobalFlag=0x{0:x8}; PageHeapFlags=0x{1:x8}; VerifierFlags=0x{2:x8}. {3}' -f $global, $heap, $checks, $_.Exception.Message) }
}

function Convert-RegistryDword {
    param($Value)
    if ($Value.kind -eq 'String') { return [Convert]::ToUInt32(($Value.data -replace '^0x', ''), 16) }
    if ($Value.kind -ne 'DWord') { throw 'Unexpected registry flag value type.' }
    # RegistryKey.GetValue represents a high-bit DWORD as signed Int32.
    return [BitConverter]::ToUInt32([BitConverter]::GetBytes([int32]$Value.data), 0)
}

function Invoke-MemoryWorkflow {
    if ($env:OS -ne 'Windows_NT' -or [IntPtr]::Size -ne 8) { throw 'Use native x64 Windows PowerShell.' }
    # Elevated configuration never launches CDB. Its administrator account's
    # debugger settings cannot establish readiness for ordinary-user execution.
    $tools = Get-AdmittedTools ($Action -eq 'Preflight')
    if ($Action -eq 'Preflight') {
        return [ordered]@{ status = 'admitted'; elevated = (Test-Elevated); tools = $tools; scope = 'File/signature admission only; no native diagnostic or product test executed.' }
    }
    if (-not (Test-Elevated)) { throw 'Enable/Disable needs an elevated configuration shell. Run tests separately with the intended token.' }
    $identity = Get-ImageIdentity $ImagePath ($Action -eq 'Enable')
    Assert-ImageStopped $identity.name
    $directory = Get-CustodyDirectory ($Action -eq 'Enable')
    $receiptPath = Join-Path $directory ($identity.name + '.json')
    $lockPath = Join-Path $directory ($identity.name + '.lock')
    # OpenOrCreate + exclusive sharing serializes our own operations. Existing
    # files live only under verified administrator-owned, non-reparse custody.
    Assert-NoReparse $receiptPath
    Assert-NoReparse $lockPath
    foreach ($existing in @($receiptPath, $lockPath)) {
        if (Test-Path -LiteralPath $existing) { Assert-ProtectedAcl $existing $false }
    }
    $lock = Open-CustodyLock $lockPath (New-OwnedFileSecurity)
    try {
        if ($Action -eq 'Enable') {
            if (Test-Path -LiteralPath $receiptPath) { throw 'A custody receipt already exists. Inspect it and use a new unique fixture image.' }
            $state = Get-ImageState $identity.name
            Assert-InitiallyAbsent $state
            $receipt = [ordered]@{ schema = 1; image = $identity; status = 'prepared'; expected = $state; steps = @(); policy_sha256 = (Get-FileHash -LiteralPath $script:PolicyPath -Algorithm SHA256).Hash; created_utc = [DateTime]::UtcNow.ToString('o') }
            Write-Receipt $receiptPath $receipt
            try {
                Invoke-OwnedStep $receiptPath $receipt 'enable-appverif' $tools.appverif @('/verify', $identity.name)
                Invoke-OwnedStep $receiptPath $receipt 'enable-pageheap' $tools.gflags @('/p', '/enable', $identity.name, '/full')
                Assert-EnabledProfile $receipt.expected
                $receipt.status = 'enabled'
                Write-Receipt $receiptPath $receipt
            } catch {
                $original = $_
                try {
                    Assert-StateMatch (Get-ImageState $identity.name) $receipt.expected
                    Disable-OwnedProfile $receiptPath $receipt $tools
                } catch {
                    Write-Warning "Automatic recovery incomplete: $($_.Exception.Message). Inspect protected receipt: $receiptPath"
                }
                throw $original
            }
        } else {
            if (-not (Test-Path -LiteralPath $receiptPath -PathType Leaf)) { throw 'No owned custody receipt exists for this image.' }
            if ((Get-Item -LiteralPath $receiptPath).Length -gt 1048576) { throw 'Custody receipt exceeds limit.' }
            $receipt = Get-Content -Raw -LiteralPath $receiptPath | ConvertFrom-Json
            if ($receipt.schema -ne 1 -or $receipt.image.path -ine $identity.path -or $receipt.image.name -cne $identity.name -or
                $receipt.policy_sha256 -cne (Get-FileHash -LiteralPath $script:PolicyPath -Algorithm SHA256).Hash -or
                $receipt.status -notmatch '^(prepared|enabled|pending-[a-z-]+|observed-[a-z-]+|disabled)$') {
                throw 'Receipt identity, policy or state does not match this operation.'
            }
            Assert-StateMatch (Get-ImageState $identity.name) $receipt.expected
            if ($receipt.status -eq 'disabled') { Assert-InitiallyAbsent $receipt.expected }
            else { Disable-OwnedProfile $receiptPath $receipt $tools }
        }
        return [ordered]@{ status = $receipt.status; image = $identity.name; custody = $receiptPath; scope = 'Persistent configuration only; no application launched.' }
    } finally { $lock.Dispose() }
}

try { Invoke-MemoryWorkflow | ConvertTo-Json -Depth 8 }
catch { Write-Error $_; exit 1 }
