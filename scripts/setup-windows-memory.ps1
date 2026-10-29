#requires -Version 5.1
<#
Install pinned Microsoft diagnostic components in an elevated x64 shell.
ConfigureOnly applies CDB defaults to the ordinary user who will run it;
DownloadOnly verifies acquisition. Neither needs elevation. No automatic
elevation, restart, PATH change or per-image verification settings.
#>
[CmdletBinding()]
param([switch]$DownloadOnly, [switch]$ConfigureOnly)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
if ($DownloadOnly -and $ConfigureOnly) { throw 'Select either DownloadOnly or ConfigureOnly.' }
if ($env:OS -ne 'Windows_NT' -or [IntPtr]::Size -ne 8) {
    throw 'Use native x64 Windows PowerShell.'
}
$principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
$elevated = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $DownloadOnly -and -not $ConfigureOnly -and -not $elevated) {
    throw 'Installation needs an elevated Windows PowerShell. DownloadOnly does not install or elevate.'
}
if ($ConfigureOnly -and $elevated) { throw 'Configure CDB from the ordinary-user account that will run the debugger.' }
$policy = Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot '..\policy\windows-memory.json') | ConvertFrom-Json
$pin = $policy.sdk_installer
if ($policy.schema -ne 1 -or $pin.sha256 -cnotmatch '^[a-f0-9]{64}$' -or
    $pin.url -cnotmatch '^https://download\.microsoft\.com/download/[a-zA-Z0-9/_-]+/winsdksetup\.exe$' -or
    (@($pin.features) -join ',') -cne 'OptionId.WindowsDesktopDebuggers,OptionId.AvrfExternal' -or
    $pin.signer_subject -cne 'CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US') {
    throw 'Installer policy does not match the reviewed Microsoft SDK component scope.'
}

function Install-DebuggerConfiguration {
    $source = Join-Path $PSScriptRoot '..\policy\windows-debugger.xml'
    $target = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'DBG\config.xml'
    $expected = $policy.debugger_config_sha256
    $bytes = [IO.File]::ReadAllBytes($source)
    $hasher = [Security.Cryptography.SHA256]::Create()
    try { $sourceHash = ([BitConverter]::ToString($hasher.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant() }
    finally { $hasher.Dispose() }
    if ($expected -cnotmatch '^[a-f0-9]{64}$' -or
        $sourceHash -cne $expected) {
        throw 'Authored debugger configuration hash differs from policy.'
    }
    if ((Get-FileHash -LiteralPath $policy.tools.cdb.path -Algorithm SHA256).Hash.ToLowerInvariant() -cne $policy.tools.cdb.sha256) {
        throw 'Install the admitted CDB before configuring its startup settings.'
    }
    $cursor = $target
    while ($cursor) {
        if ((Test-Path -LiteralPath $cursor) -and
            (((Get-Item -LiteralPath $cursor -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) {
            throw 'Debugger configuration ancestors must not be reparse points.'
        }
        $cursor = [IO.Path]::GetDirectoryName($cursor)
    }
    $parent = [IO.Path]::GetDirectoryName($target)
    if (-not (Test-Path -LiteralPath $parent)) {
        $acl = New-Object Security.AccessControl.DirectorySecurity
        $acl.SetAccessRuleProtection($true, $false)
        $userSid = [Security.Principal.WindowsIdentity]::GetCurrent().User
        $acl.SetOwner($userSid)
        foreach ($sid in @($userSid, (New-Object Security.Principal.SecurityIdentifier('S-1-5-18')), (New-Object Security.Principal.SecurityIdentifier('S-1-5-32-544')))) {
            $acl.AddAccessRule((New-Object Security.AccessControl.FileSystemAccessRule($sid, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')))
        }
        [IO.Directory]::CreateDirectory($parent, $acl) | Out-Null
    }
    $previous = $null
    if (Test-Path -LiteralPath $target) {
        $previous = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($previous -cne $expected -and $previous -cnotin @($policy.debugger_config_previous_owned_sha256)) {
            throw 'Existing debugger configuration differs; preserve it for explicit reconciliation.'
        }
    }
    if ($previous -cne $expected) {
        $temporary = $target + '.nevrotick-' + [Guid]::NewGuid().ToString('N') + '.tmp'
        $stream = [IO.File]::Open($temporary, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
        try { $stream.Write($bytes, 0, $bytes.Length); $stream.Flush($true) }
        finally { $stream.Dispose() }
        if ($null -eq $previous) { [IO.File]::Move($temporary, $target) }
        else {
            # Only a previously admitted, task-owned configuration may be replaced.
            # Keep its exact bytes beside the replacement for recovery/review.
            if ((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash.ToLowerInvariant() -cne $previous) {
                throw 'Debugger configuration changed during setup; replacement refused.'
            }
            [IO.File]::Replace($temporary, $target, ($temporary + '.previous.xml'))
        }
    }
    if ((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash.ToLowerInvariant() -cne $expected) {
        throw 'Installed debugger configuration hash mismatch.'
    }
}

if ($ConfigureOnly) {
    Install-DebuggerConfiguration
    $LASTEXITCODE = 0
    & (Join-Path $PSScriptRoot 'windows-memory.ps1') -Action Preflight
    if ($LASTEXITCODE -ne 0) { throw 'Configured-tool admission failed.' }
    [ordered]@{ status = 'configured'; per_image_settings_changed = $false } | ConvertTo-Json
    exit 0
}

# Create a fresh directory with explicit protected ACLs in one operation. An
# elevated install must not execute an installer from an ordinary-user cache.
$base = if ($elevated) { [Environment]::GetFolderPath('CommonApplicationData') } else { [Environment]::GetFolderPath('LocalApplicationData') }
$cursor = $base
while ($cursor) {
    if (((Get-Item -LiteralPath $cursor -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'Setup directory ancestors must not be reparse points.'
    }
    $cursor = [IO.Path]::GetDirectoryName($cursor)
}
$directory = Join-Path $base ('NevroTick-WindowsMemory-Setup-' + [Guid]::NewGuid().ToString('N'))
$security = New-Object Security.AccessControl.DirectorySecurity
$security.SetAccessRuleProtection($true, $false)
$owner = if ($elevated) { New-Object Security.Principal.SecurityIdentifier('S-1-5-32-544') } else { [Security.Principal.WindowsIdentity]::GetCurrent().User }
$security.SetOwner($owner)
foreach ($sid in @($owner, (New-Object Security.Principal.SecurityIdentifier('S-1-5-18')))) {
    $security.AddAccessRule((New-Object Security.AccessControl.FileSystemAccessRule($sid, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')))
}
if (Test-Path -LiteralPath $directory) { throw 'Fresh setup directory unexpectedly exists.' }
[IO.Directory]::CreateDirectory($directory, $security) | Out-Null
$installer = Join-Path $directory 'winsdksetup.exe'
Invoke-WebRequest -UseBasicParsing -Uri $pin.url -OutFile $installer -TimeoutSec 120
if ((Get-FileHash -LiteralPath $installer -Algorithm SHA256).Hash.ToLowerInvariant() -cne $pin.sha256) {
    throw "Installer hash mismatch; retained unexecuted download at $installer"
}
$signature = Get-AuthenticodeSignature -LiteralPath $installer
if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -cne $pin.signer_subject -or
    (Get-Item -LiteralPath $installer).VersionInfo.FileVersion -cne $pin.file_version) {
    throw 'Installer Microsoft signature or version mismatch; nothing executed.'
}
if ($DownloadOnly) {
    [ordered]@{ status = 'download-verified'; installer = $installer; sha256 = $pin.sha256; installed = $false } | ConvertTo-Json
    exit 0
}

$log = Join-Path $directory 'install.log'
$arguments = @('/features') + @($pin.features) + @('/quiet', '/norestart', '/ceip', 'off', '/log', ('"' + $log + '"'))
$process = Start-Process -FilePath $installer -WorkingDirectory $directory -ArgumentList $arguments -PassThru
$null = $process.Handle
if (-not $process.WaitForExit(1200000)) {
    throw "Installer still running (PID $($process.Id)); do not start another installation or terminate MSI blindly. Inspect $log"
}
$process.WaitForExit()
if ($process.ExitCode -notin @(0, 3010)) { throw "SDK installer failed: $($process.ExitCode). Retained log: $log" }
$reboot = $process.ExitCode -eq 3010
[ordered]@{ status = 'components-installed'; sdk_release = $policy.sdk_release; restart_required = $reboot; log = $log; per_image_settings_changed = $false; next_step = 'Run setup-windows-memory.ps1 -ConfigureOnly from the intended ordinary-user account, then Preflight.' } | ConvertTo-Json
