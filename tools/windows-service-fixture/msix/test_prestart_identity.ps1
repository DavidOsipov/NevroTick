# SPDX-License-Identifier: GPL-2.0-only
# Run in the owned offline lab with an absent S0.6 package and a precompiled
# prestart_image_path.dll beside these exact source inputs.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = Join-Path ([IO.Path]::GetTempPath()) ('NevroTick identity controls ' + [guid]::NewGuid().ToString('N'))
$null = [IO.Directory]::CreateDirectory($root)
$module = $null
$actor = $null

function Get-ControlDigest {
    param([string]$Path)
    return (Microsoft.PowerShell.Utility\Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Assert-InputsReleased {
    param([string]$Directory)
    foreach ($leaf in @('observe_installed_identity.ps1', 'installed_identity.psm1', 'prestart_image_path.dll')) {
        $stream = [IO.File]::Open((Join-Path $Directory $leaf), [IO.FileMode]::Open,
            [IO.FileAccess]::ReadWrite, [IO.FileShare]::None)
        $stream.Dispose()
    }
}

function Invoke-Control {
    param([hashtable]$Request)
    $Request.Clock = [Diagnostics.Stopwatch]::StartNew()
    $capture = Invoke-PrestartIdentity @Request
    if ($capture.InputSha256Before -cne $Request.ModuleSha256 -or
        $capture.InputSha256After -cne $Request.ModuleSha256) {
        throw 'Identity capture did not return the admitted companion digest at both bookends'
    }
    return $capture
}

function Assert-ChildRefusal {
    param([hashtable]$Request, [string]$Reason)
    $capture = Invoke-Control $Request
    $diagnostic = [Text.Encoding]::ASCII.GetString($capture.stderr)
    if ($capture.ProcessId -le 0 -or $capture.ExitCode -ne 1 -or $capture.Stdout.Length -ne 0 -or
        $diagnostic.IndexOf($Reason, [StringComparison]::Ordinal) -lt 0) {
        throw "Identity did not reach expected child refusal: expected=$Reason; exit=$($capture.ExitCode); stdout-bytes=$($capture.Stdout.Length); stderr=$diagnostic"
    }
    Assert-InputsReleased $Request.InputDirectory
}

function Assert-LaunchRefusal {
    param([hashtable]$Request, [string]$Reason)
    $cause = $null
    try { $null = Invoke-Control $Request } catch { $cause = $_.Exception }
    while ($null -ne $cause -and $cause.Message -cne $Reason) {
        $cause = $cause.InnerException
    }
    if ($null -eq $cause -or $cause.Data['PrestartObserverProcessCreated'] -ne $false -or
        $cause.Data.Contains('PrestartObserverProcessId') -or
        $cause.Data['PrestartObserverRetired'] -ne $true -or
        $cause.Data['PrestartObserverReleased'] -ne $true) {
        throw "Identity launch did not refuse before process creation: expected=$Reason"
    }
    Assert-InputsReleased $Request.InputDirectory
}

try {
    $testUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    try { [string]$sid = $testUser.User.Value } finally { $testUser.Dispose() }
    if (@(Appx\Get-AppxPackage -Name 'NevroTick.S06.ScmFixture' -PackageTypeFilter Main |
        Select-Object -First 1).Count -ne 0) {
        throw 'Identity control requires absence of the S0.6 fixture registration'
    }
    if ((Microsoft.PowerShell.Management\Get-Service -Name 'RpcSs').Status -ne 'Running') {
        throw 'Identity control requires the running local RPC service as its CIM readback neighbor'
    }
    $module = Import-Module -Name (Join-Path $PSScriptRoot 'prestart_child.psm1') -PassThru
    $actor = Open-PrestartObserverActor -Clock ([Diagnostics.Stopwatch]::StartNew()) -TimeoutMilliseconds 30000
    foreach ($leaf in @('observe_installed_identity.ps1', 'installed_identity.psm1', 'prestart_image_path.dll')) {
        [IO.File]::Copy((Join-Path $PSScriptRoot $leaf), (Join-Path $root $leaf))
    }
    $interpreter = Join-Path ([Environment]::SystemDirectory) 'WindowsPowerShell\v1.0\powershell.exe'
    $entry = Join-Path $root 'observe_installed_identity.ps1'
    $companion = Join-Path $root 'installed_identity.psm1'
    $helper = Join-Path $root 'prestart_image_path.dll'
    $request = @{
        PowerShellPath = $interpreter
        PowerShellBytes = (Get-Item -LiteralPath $interpreter).Length
        PowerShellSha256 = Get-ControlDigest $interpreter
        InputDirectory = $root
        EntryBytes = (Get-Item -LiteralPath $entry).Length
        EntrySha256 = Get-ControlDigest $entry
        ModuleBytes = (Get-Item -LiteralPath $companion).Length
        ModuleSha256 = Get-ControlDigest $companion
        HelperBytes = (Get-Item -LiteralPath $helper).Length
        HelperSha256 = Get-ControlDigest $helper
        RunId = [guid]'8381e9c5-0c17-4a99-8b47-d50bdf18d205'
        PackageFullName = 'NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc'
        ExpectedUserSid = $sid
        SignedPackageSha256 = ('1' * 64)
        EngineBytes = 3
        EngineSha256 = 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'
        BrokerBytes = 3
        BrokerSha256 = 'cb8379ac2098aa165029e3938a51da0bcecfc008fd6795f401178647f96c5b34'
        Actor = $actor
        TimeoutMilliseconds = 15000
    }
    Assert-ChildRefusal $request 'Expected exactly one S0.6 registration in the current user profile'
    's06-owned-identity absent-registration=observed'

    $request.ExpectedUserSid = 'S-1-0-0'
    $approvedHelper = $request.HelperSha256
    $request.HelperSha256 = ('0' * 64)
    Assert-LaunchRefusal $request 'prestart-observer:image-digest'
    $request.HelperSha256 = $approvedHelper
    Assert-LaunchRefusal $request 'prestart-observer:identity-subject-actor'
    $request.ExpectedUserSid = $sid
    's06-owned-identity wrong-subject=refused'

    # Inject fresh bytes into each supplied file identity and require launch
    # refusal before a capture can be published. Repeat the genuine neighbor.
    foreach ($key in @('PowerShellSha256', 'EntrySha256', 'ModuleSha256', 'HelperSha256')) {
        $approved = $request[$key]
        $request[$key] = ('0' * 64)
        Assert-LaunchRefusal $request 'prestart-observer:image-digest'
        $request[$key] = $approved
        "s06-owned-identity wrong-$key=refused"
    }
    Assert-ChildRefusal $request 'Expected exactly one S0.6 registration in the current user profile'
    's06-owned-identity restored-registration-control=observed'

    # Isolate output capture from the intentionally absent package. The exact
    # entry consumes these values through a bounded inert companion in this case.
    $inertModule = @'
function Write-S06InstalledIdentity {
    param([hashtable]$Inputs, [IO.Stream]$OutputStream)

    if ($Inputs.RunId -ne [guid]'8381e9c5-0c17-4a99-8b47-d50bdf18d205' -or
        $Inputs.PackageFullName -cne 'NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc' -or
        $Inputs.SignedPackageSha256 -cne ('1' * 64) -or
        $Inputs.EngineBytes -ne 3 -or $Inputs.BrokerBytes -ne 3 -or
        $Inputs.EngineSha256 -cne 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad' -or
        $Inputs.BrokerSha256 -cne 'cb8379ac2098aa165029e3938a51da0bcecfc008fd6795f401178647f96c5b34') {
        throw 'Fixed identity input transfer differed'
    }
    $actual = [Security.Principal.WindowsIdentity]::GetCurrent()
    try {
        if ($actual.User.Value -cne $Inputs.ExpectedUserSid) { throw 'Unexpected identity input subject' }
    } finally { $actual.Dispose() }
    if ($null -ne [Environment]::GetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL')) {
        throw 'Identity child inherited a caller environment variable'
    }
    $service = @(CimCmdlets\Get-CimInstance -ClassName Win32_Service -Filter "Name='RpcSs'")
    if ($service.Count -ne 1 -or $service[0].Name -cne 'RpcSs' -or $service[0].State -cne 'Running') {
        throw 'Identity child did not reach the expected local CIM service'
    }
    foreach ($leaf in @('observe_installed_identity.ps1', 'installed_identity.psm1', 'prestart_image_path.dll')) {
        try {
            $writer = [IO.File]::Open([IO.Path]::Combine($PSScriptRoot, $leaf), [IO.FileMode]::Open,
                [IO.FileAccess]::Write, [IO.FileShare]::ReadWrite)
        } catch {
            $cause = $_.Exception
            while ($null -ne $cause.InnerException -and $cause -isnot [IO.IOException]) {
                $cause = $cause.InnerException
            }
            if ($cause -is [IO.IOException] -and ($cause.HResult -band 0xffff) -eq 32) { continue }
            throw
        }
        $writer.Dispose()
        throw "Identity accepted a writer for held $leaf"
    }
    $assembly = [Reflection.Assembly]::LoadFile([IO.Path]::Combine($PSScriptRoot, 'prestart_image_path.dll'))
    [type]$pathReader = $assembly.GetType('NevroTick.S06ImagePath', $true, $false)
    $entry = [IO.Path]::Combine($PSScriptRoot, 'observe_installed_identity.ps1')
    $reader = [IO.File]::Open($entry, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        if ($pathReader::Read($reader.SafeFileHandle) -cne ('\\?\' + $entry)) {
            throw 'Precompiled path helper did not read the expected held path inside the child'
        }
    } finally { $reader.Dispose() }
    [byte[]]$bytes = @(73, 0, 255, 10)
    $OutputStream.Write($bytes, 0, $bytes.Length)
}
Export-ModuleMember -Function Write-S06InstalledIdentity
'@
    [IO.File]::WriteAllText($companion, $inertModule, [Text.Encoding]::ASCII)
    $request.ModuleBytes = (Get-Item -LiteralPath $companion).Length
    $request.ModuleSha256 = Get-ControlDigest $companion
    $prior = [Environment]::GetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL')
    [Environment]::SetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL', 'test-parent-only')
    try { $healthy = Invoke-Control $request } finally {
        [Environment]::SetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL', $prior)
    }
    if ($healthy.ProcessId -le 0 -or $healthy.ExitCode -ne 0 -or $healthy.Stderr.Length -ne 0 -or
        [Convert]::ToBase64String($healthy.stdout) -cne 'SQD/Cg==') {
        $diagnostic = [Text.Encoding]::ASCII.GetString($healthy.stderr)
        throw "Fixed child did not publish bounded raw output: exit=$($healthy.ExitCode); stderr=$diagnostic"
    }
    Assert-InputsReleased $root
    's06-owned-identity bounded-raw-output=observed'
} finally {
    if ($null -ne $actor) { $actor.Dispose() }
    if ($null -ne $module) { Remove-Module $module }
    [IO.Directory]::Delete($root, $true)
}
