# SPDX-License-Identifier: GPL-2.0-only
# Run in the owned offline lab with an absent S0.6 package and a precompiled
# prestart_image_path.dll beside these exact source inputs. Both archives are
# unsigned controls for archive custody, signature refusal and argument ownership.
# No package is installed.
[CmdletBinding(PositionalBinding = $false)]
param(
    [Parameter(Mandatory = $true)][string]$CurrentArchive,
    [Parameter(Mandatory = $true)][string]$PreviousArchive,
    [Parameter(Mandatory = $true)][string]$ReferencePath,
    [Parameter(Mandatory = $true)][ValidatePattern('\A[a-f0-9]{64}\z')][string]$ReferenceSha256
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = Join-Path ([IO.Path]::GetTempPath()) ('NevroTick identity controls ' + [guid]::NewGuid().ToString('N'))
$null = [IO.Directory]::CreateDirectory($root)
$module = $null
$actor = $null
$current = $null
$previous = $null
$retainInputDirectory = $false
$testFailure = $null

function Get-ControlDigest {
    param([string]$Path)
    return (Microsoft.PowerShell.Utility\Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

# Read one bounded snapshot for the independent reference or a private lab input.
function Read-ControlBytes {
    param([string]$Path, [int]$Limit)

    [IO.FileStream]$stream = [IO.File]::Open($Path, 'Open', 'Read', 'Read')
    try {
        [long]$length = $stream.Length
        if ($length -lt 1 -or $length -gt $Limit) { throw 'Identity control input extent differs' }
        [byte[]]$bytes = [byte[]]::new([int]$length)
        [int]$offset = 0
        while ($offset -lt $bytes.Length) {
            [int]$count = $stream.Read($bytes, $offset, $bytes.Length - $offset)
            if ($count -le 0) { throw 'Identity control input ended early' }
            $offset += $count
        }
        if ($stream.ReadByte() -ne -1) { throw 'Identity control input grew during capture' }
        ,$bytes
    } finally {
        $stream.Dispose()
    }
}

function Read-ControlReference {
    [byte[]]$bytes = Read-ControlBytes $ReferencePath 65536
    [Security.Cryptography.SHA256]$sha = [Security.Cryptography.SHA256]::Create()
    try {
        [string]$actual = [BitConverter]::ToString($sha.ComputeHash($bytes)).Replace('-', '').ToLowerInvariant()
        if ($actual -cne $ReferenceSha256) { throw 'Identity control reference digest differs' }
        [Text.UTF8Encoding]$utf8 = [Text.UTF8Encoding]::new($false, $true)
        return (ConvertFrom-Json -InputObject ($utf8.GetString($bytes)))
    } finally {
        $sha.Dispose()
    }
}

function Assert-InputsReleased {
    param([string]$Directory)
    foreach ($leaf in @('observe_installed_identity.ps1', 'installed_identity.psm1', 'prestart_image_path.dll')) {
        $stream = [IO.File]::Open((Join-Path $Directory $leaf), [IO.FileMode]::Open,
            [IO.FileAccess]::ReadWrite, [IO.FileShare]::None)
        $stream.Dispose()
    }
}

function Assert-ArchiveReleased {
    param([string]$Path)

    [IO.FileStream]$stream = [IO.File]::Open($Path, 'Open', 'ReadWrite', 'None')
    $stream.Dispose()
    [IO.File]::Move($Path, ($Path + '.renamed'))
    [IO.File]::Move(($Path + '.renamed'), $Path)
}

function Assert-ArchiveSharingRefusal {
    param([scriptblock]$Operation, [string]$Description)

    try {
        & $Operation
    } catch {
        [Exception]$cause = $_.Exception
        while ($null -ne $cause.InnerException -and $cause -isnot [IO.IOException]) {
            $cause = $cause.InnerException
        }
        if ($cause -is [IO.IOException] -and ($cause.HResult -band 0xffff) -eq 32) { return }
        throw
    }
    throw "Archive accepted $Description while its lease was live"
}

function Assert-ArchiveHeld {
    param([string]$Path, [string]$Digest)

    if ((Get-ControlDigest $Path) -cne $Digest) { throw 'Held archive readback digest differs' }
    Assert-ArchiveSharingRefusal {
        [IO.FileStream]$writer = [IO.File]::Open($Path, 'Open', 'Write', 'ReadWrite')
        $writer.Dispose()
    } 'write access'
    Assert-ArchiveSharingRefusal {
        [IO.File]::Move($Path, ($Path + '.renamed'))
    } 'rename'
}

function Assert-ArchiveOpenRefusal {
    param([hashtable]$Request, [type]$ExceptionType, [string]$Reason)

    try {
        [IDisposable]$unexpected = Open-PrestartArchive @Request
    } catch {
        [Exception]$cause = $_.Exception
        while ($null -ne $cause.InnerException) { $cause = $cause.InnerException }
        if ($cause.GetType() -ne $ExceptionType -or $cause.Message -cne $Reason) { throw }
        if ($cause -is [Runtime.InteropServices.COMException] -and $cause.ErrorCode -ge 0) { throw }
        Assert-ArchiveReleased $Request.ArchivePath
        return
    }
    if ($null -ne $unexpected) { $unexpected.Dispose() }
    throw "Archive opened despite expected refusal: $Reason"
}

function Start-ControlIdentity {
    param([hashtable]$Request)

    return [NevroTick.PrestartObserverChild]::StartIdentity(
        $Request.PowerShellPath, $Request.PowerShellBytes, $Request.PowerShellSha256,
        $Request.InputDirectory, $Request.EntryBytes, $Request.EntrySha256,
        $Request.ModuleBytes, $Request.ModuleSha256, $Request.HelperBytes, $Request.HelperSha256,
        $Request.RunId, $Request.Archive, $Request.ExpectedUserSid, $Request.ExpectedArchiveSha256,
        $Request.Actor, $Request.Clock, $Request.TimeoutMilliseconds)
}

function Assert-ArchivePublicationRefusal {
    param([hashtable]$Request)

    $script:retainInputDirectory = $true
    [object]$child = Start-ControlIdentity $Request
    [int]$childId = $child.ProcessId
    [pscustomobject]$proxy = [pscustomobject]@{
        Child = $child; Archive = $Request.Archive; ProcessId = $childId
        Stdout = $child.Stdout; Stderr = $child.Stderr
        ConfirmedImage = $null; ExitBeforeRelease = $null; PublicationChecks = 0
    }
    $proxy | Add-Member ScriptProperty HasExited { $this.Child.HasExited }
    $proxy | Add-Member ScriptProperty ExitCode { $this.Child.ExitCode }
    $proxy | Add-Member ScriptProperty IsJobEmpty { $this.Child.IsJobEmpty }
    $proxy | Add-Member ScriptProperty DisposalStarted { $this.Child.DisposalStarted }
    $proxy | Add-Member ScriptProperty ReleaseConfirmed { $this.Child.ReleaseConfirmed }
    $proxy | Add-Member ScriptMethod RequireCollectionTime { $this.Child.RequireCollectionTime() }
    $proxy | Add-Member ScriptMethod ObserveRetirement { $this.Child.ObserveRetirement() }
    $proxy | Add-Member ScriptMethod Terminate { $this.Child.Terminate() }
    $proxy | Add-Member ScriptMethod ConfirmImage {
        $this.ConfirmedImage = $this.Child.ConfirmImage()
        $this.ExitBeforeRelease = $this.Child.ExitCode
        return $this.ConfirmedImage
    }
    $proxy | Add-Member ScriptMethod Dispose {
        try {
            if ($null -ne $this.ConfirmedImage) { $this.Archive.Dispose() }
        } finally {
            $this.Child.Dispose()
        }
    }
    $proxy | Add-Member ScriptMethod ConfirmPublication {
        $this.PublicationChecks++
        $this.Child.ConfirmPublication()
    }

    [Collections.Generic.List[object]]$records = [Collections.Generic.List[object]]::new()
    [Management.Automation.ErrorRecord]$failure = $null
    [bool]$archiveRetired = $false
    try {
        & $module { param($Child) Receive-OwnedPrestartChild $Child } $proxy |
            ForEach-Object { $records.Add($_) }
        $script:retainInputDirectory = $false
    } catch {
        $failure = $_
        for ([Exception]$cause = $_.Exception; $null -ne $cause; $cause = $cause.InnerException) {
            if ($cause -is [InvalidOperationException] -and $cause.Message -ceq 'prestart-observer:archive-retired') {
                $archiveRetired = $true
            }
            if ($cause.Data['PrestartObserverProcessId'] -eq $childId -and
                $cause.Data['PrestartObserverRetired'] -eq $true -and
                $cause.Data['PrestartObserverReleased'] -eq $true -and
                !$cause.Data.Contains('PrestartObserverCleanupFailure')) {
                $script:retainInputDirectory = $false
            }
        }
        if ($retainInputDirectory -and !$child.DisposalStarted) {
            $_.Exception.Data['PrestartArchiveControlChild'] = [object[]]@($child)
        }
    }
    if ($records.Count -ne 0) {
        throw "Identity published $($records.Count) capture(s) after archive retirement (PID $childId)"
    }
    if ($retainInputDirectory -or !$archiveRetired -or !$child.ReleaseConfirmed -or $proxy.PublicationChecks -ne 1 -or
        $null -eq $proxy.ConfirmedImage -or $proxy.ExitBeforeRelease -ne 0 -or
        $proxy.ConfirmedImage.Before -cne $Request.ModuleSha256 -or
        $proxy.ConfirmedImage.After -cne $Request.ModuleSha256) {
        if ($null -ne $failure) { throw $failure }
        throw 'Identity did not refuse at publication after confirming the original inputs'
    }
    Assert-InputsReleased $Request.InputDirectory
}

function Invoke-Control {
    param([hashtable]$Request, [switch]$DirectVerified)
    try {
        if ($DirectVerified) {
            [object]$child = [NevroTick.PrestartObserverChild]::StartVerifiedIdentity(
                $Request.PowerShellPath, $Request.PowerShellBytes, $Request.PowerShellSha256,
                $Request.InputDirectory, $Request.EntryBytes, $Request.EntrySha256,
                $Request.ModuleBytes, $Request.ModuleSha256, $Request.HelperBytes, $Request.HelperSha256,
                $Request.RunId, $Request.Archive, $Request.ExpectedUserSid, $Request.ExpectedArchiveSha256,
                $Request.ExpectedSignerSha256, $Request.Actor, $Request.Clock, $Request.TimeoutMilliseconds)
            $capture = & $module {
                param($Child)
                Receive-OwnedPrestartChild $Child
            } $child
        } else {
            $capture = Invoke-PrestartIdentity @Request
        }
    } catch {
        [Exception]$cause = $_.Exception
        while ($null -ne $cause) {
            if ($cause.Data.Contains('PrestartObserverChild')) {
                $script:retainInputDirectory = $true
                break
            }
            $cause = $cause.InnerException
        }
        throw
    }
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
    param([hashtable]$Request, [string]$Reason, [switch]$DirectVerified)
    $cause = $null
    try { $null = Invoke-Control $Request -DirectVerified:$DirectVerified } catch {
        if ($retainInputDirectory) { throw }
        $cause = $_.Exception
    }
    while ($null -ne $cause -and $cause.Message -cne $Reason) {
        $cause = $cause.InnerException
    }
    if ($null -eq $cause -or $cause.Data['PrestartObserverJobCreated'] -ne $false -or
        $cause.Data['PrestartObserverProcessCreated'] -ne $false -or
        $cause.Data.Contains('PrestartObserverProcessId') -or
        $cause.Data['PrestartObserverRetired'] -ne $true -or
        $cause.Data['PrestartObserverReleased'] -ne $true) {
        throw "Identity launch did not refuse before process creation: expected=$Reason"
    }
    Assert-InputsReleased $Request.InputDirectory
}

function Assert-SignatureRefusal {
    param([scriptblock]$Operation, [type]$ExceptionType, [string]$Reason)

    try {
        $null = & $Operation
    } catch {
        if ($retainInputDirectory) { throw }
        [Exception]$cause = $_.Exception
        while ($null -ne $cause -and $cause.Message -cne $Reason) {
            $cause = $cause.InnerException
        }
        if ($null -eq $cause -or !$ExceptionType.IsInstanceOfType($cause) -or
            $cause.Data.Contains('PrestartArchiveTrustCleanupFailure')) { throw }
        return
    }
    throw "Archive signature admitted despite expected refusal: $Reason"
}

# Consume the actual fixed entry's bounded binary prefix and independently
# compare its subsequent member and identity projection.
function Assert-ControlArchive {
    param([hashtable]$Request, [object]$Expected)
    [object]$capture = Invoke-Control $Request
    if ($capture.ProcessId -le 0 -or $capture.ExitCode -ne 0 -or $capture.Stderr.Length -ne 0 -or
        $capture.Stdout.Length -le 4 -or [Convert]::ToBase64String($capture.Stdout, 0, 4) -cne 'SQD/Cg==') {
        [string]$diagnostic = [Text.Encoding]::ASCII.GetString($capture.Stderr)
        throw "Fixed child did not publish bounded raw output: exit=$($capture.ExitCode); stderr=$diagnostic"
    }
    [Text.UTF8Encoding]$utf8 = [Text.UTF8Encoding]::new($false, $true)
    [string]$json = $utf8.GetString($capture.Stdout, 4, $capture.Stdout.Length - 4)
    [object]$observed = ConvertFrom-Json -InputObject $json
    [string[]]$fields = @('PackageFullName', 'SignedPackageSha256', 'EngineBytes', 'EngineSha256',
        'BrokerBytes', 'BrokerSha256')
    if (@($observed.PSObject.Properties.Name).Count -ne $fields.Length) {
        throw 'Fixed child archive projection inventory differs'
    }
    foreach ($field in $fields) {
        [string]$referenceField = $field
        if ($field -ceq 'SignedPackageSha256') { $referenceField = 'ArchiveSha256' }
        if ($observed.$field -cne $Expected.$referenceField) {
            throw "Fixed child archive projection differs: $field"
        }
    }
    Assert-InputsReleased $Request.InputDirectory
}

try {
    [object]$reference = Read-ControlReference
    if ($reference.current.PackageFullName -cne $reference.previous.PackageFullName -or
        $reference.current.ArchiveSha256 -ceq $reference.previous.ArchiveSha256 -or
        $reference.current.EngineSha256 -ceq $reference.previous.EngineSha256 -or
        $reference.current.BrokerSha256 -ceq $reference.previous.BrokerSha256) {
        throw 'Identity control requires two distinct carriers with the same package full name'
    }
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
    [string]$currentPath = Join-Path $root 'current.msix'
    [string]$previousPath = Join-Path $root 'previous.msix'
    [string]$malformedPath = Join-Path $root 'malformed.msix'
    [IO.File]::WriteAllBytes($currentPath, (Read-ControlBytes $CurrentArchive 8MB))
    [IO.File]::WriteAllBytes($previousPath, (Read-ControlBytes $PreviousArchive 8MB))
    [byte[]]$malformed = [Text.Encoding]::ASCII.GetBytes('not a package')
    [IO.File]::WriteAllBytes($malformedPath, $malformed)

    [Diagnostics.Stopwatch]$clock = [Diagnostics.Stopwatch]::StartNew()
    [int]$budget = 60000
    $actor = Open-PrestartObserverActor -Clock $clock -TimeoutMilliseconds $budget
    [hashtable]$archiveRequest = @{
        ArchivePath = $currentPath
        ExpectedArchiveBytes = $reference.current.ArchiveBytes
        ExpectedArchiveSha256 = $reference.current.ArchiveSha256
        Clock = $clock
        TimeoutMilliseconds = $budget
    }
    [hashtable]$badArchive = $archiveRequest.Clone()
    $badArchive.ExpectedArchiveBytes = $reference.current.ArchiveBytes + 1
    Assert-ArchiveOpenRefusal $badArchive ([IO.InvalidDataException]) 'prestart-archive:archive-size'
    $badArchive.ExpectedArchiveBytes = $reference.current.ArchiveBytes
    $badArchive.ExpectedArchiveSha256 = $reference.previous.ArchiveSha256
    Assert-ArchiveOpenRefusal $badArchive ([IO.InvalidDataException]) 'prestart-archive:archive-digest'
    $badArchive.ArchivePath = $malformedPath
    $badArchive.ExpectedArchiveBytes = $malformed.Length
    $badArchive.ExpectedArchiveSha256 = Get-ControlDigest $malformedPath
    Assert-ArchiveOpenRefusal $badArchive ([Runtime.InteropServices.COMException]) 'prestart-archive:package-reader'
    's06-owned-identity archive-admission=refused-at-size-digest-and-reader'

    $current = Open-PrestartArchive @archiveRequest
    $archiveRequest.ArchivePath = $previousPath
    $archiveRequest.ExpectedArchiveBytes = $reference.previous.ArchiveBytes
    $archiveRequest.ExpectedArchiveSha256 = $reference.previous.ArchiveSha256
    $previous = Open-PrestartArchive @archiveRequest
    Assert-ArchiveHeld $currentPath $reference.current.ArchiveSha256
    Assert-ArchiveHeld $previousPath $reference.previous.ArchiveSha256
    's06-owned-identity archive-file-custody=observed'
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
        Archive = $current
        ExpectedUserSid = $sid
        ExpectedArchiveSha256 = $reference.current.ArchiveSha256
        Actor = $actor
        Clock = $clock
        TimeoutMilliseconds = $budget
    }
    [hashtable]$verifiedRequest = $request.Clone()
    [string]$testSigner = '0' * 64
    $verifiedRequest.ExpectedSignerSha256 = $testSigner
    Assert-LaunchRefusal $verifiedRequest 'prestart-observer:archive-signature-required' -DirectVerified
    [string]$unsignedReason = 'prestart-observer:archive-signature-verify-0x800b0100'
    Assert-SignatureRefusal {
        Invoke-Control $verifiedRequest
    } ([IO.InvalidDataException]) $unsignedReason
    Assert-SignatureRefusal {
        $previous.VerifySignature($reference.previous.ArchiveSha256, $testSigner, $actor, $clock, $budget)
    } ([IO.InvalidDataException]) $unsignedReason
    Assert-ArchiveHeld $currentPath $reference.current.ArchiveSha256
    Assert-ArchiveHeld $previousPath $reference.previous.ArchiveSha256
    's06-owned-identity archive-signature-unsigned=observed'
    Assert-LaunchRefusal $verifiedRequest 'prestart-observer:archive-signature-required' -DirectVerified
    's06-owned-identity archive-signature-required=observed'

    Assert-SignatureRefusal {
        $current.VerifySignature($reference.previous.ArchiveSha256, $testSigner, $actor, $clock, $budget)
    } ([IO.InvalidDataException]) 'prestart-observer:identity-archive-digest'
    Assert-SignatureRefusal {
        $current.VerifySignature($reference.current.ArchiveSha256, 'invalid', $actor, $clock, $budget)
    } ([ArgumentException]) 'prestart-observer:archive-signer-sha256'
    Assert-SignatureRefusal {
        $current.VerifySignature($reference.current.ArchiveSha256, $testSigner, $actor,
            [Diagnostics.Stopwatch]::StartNew(), $budget)
    } ([InvalidOperationException]) 'prestart-observer:archive-scope'
    Assert-SignatureRefusal {
        $current.VerifySignature($reference.current.ArchiveSha256, $testSigner, $actor, $clock, ($budget + 1))
    } ([InvalidOperationException]) 'prestart-observer:archive-scope'

    [Diagnostics.Stopwatch]$expiryClock = [Diagnostics.Stopwatch]::StartNew()
    [int]$expiryBudget = 5000
    Start-Sleep -Milliseconds 2000
    [Diagnostics.Stopwatch]$ageSinceOpen = [Diagnostics.Stopwatch]::StartNew()
    [IDisposable]$expiryLease = Open-PrestartArchive -ArchivePath $currentPath `
        -ExpectedArchiveBytes $reference.current.ArchiveBytes -ExpectedArchiveSha256 $reference.current.ArchiveSha256 `
        -Clock $expiryClock -TimeoutMilliseconds $expiryBudget
    try {
        Start-Sleep -Milliseconds ([int][Math]::Max(0, $expiryBudget - $expiryClock.ElapsedMilliseconds + 25))
        if ($expiryClock.ElapsedMilliseconds -lt $expiryBudget -or
            $ageSinceOpen.ElapsedMilliseconds -ge $expiryBudget) {
            throw 'Control did not reach the original cutoff within the shorter lease lifetime'
        }
        # Restart before observing expiry so a cached refusal cannot satisfy the control.
        $expiryClock.Restart()
        Assert-SignatureRefusal {
            $expiryLease.VerifySignature($reference.current.ArchiveSha256, 'invalid', $actor, $expiryClock,
                $expiryBudget)
        } ([TimeoutException]) 'prestart-observer:archive-deadline'
        [hashtable]$expiredRequest = $verifiedRequest.Clone()
        $expiredRequest.Archive = $expiryLease
        $expiredRequest.Clock = $expiryClock
        $expiredRequest.TimeoutMilliseconds = $expiryBudget
        Assert-LaunchRefusal $expiredRequest 'prestart-observer:archive-deadline' -DirectVerified
    } finally {
        $expiryLease.Dispose()
    }
    's06-owned-identity archive-signature-pins-and-scope=observed'

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

    if ($Inputs.RunId -ne [guid]'8381e9c5-0c17-4a99-8b47-d50bdf18d205') {
        throw 'Fixed identity run input differed'
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
    [hashtable]$projection = @{
        PackageFullName = $Inputs.PackageFullName; SignedPackageSha256 = $Inputs.SignedPackageSha256
        EngineBytes = $Inputs.EngineBytes; EngineSha256 = $Inputs.EngineSha256
        BrokerBytes = $Inputs.BrokerBytes; BrokerSha256 = $Inputs.BrokerSha256
    }
    [byte[]]$json = [Text.Encoding]::UTF8.GetBytes((ConvertTo-Json -InputObject $projection -Compress))
    $OutputStream.Write($json, 0, $json.Length)
}
Export-ModuleMember -Function Write-S06InstalledIdentity
'@
    [IO.File]::WriteAllText($companion, $inertModule, [Text.Encoding]::ASCII)
    $request.ModuleBytes = (Get-Item -LiteralPath $companion).Length
    $request.ModuleSha256 = Get-ControlDigest $companion
    [string]$prior = [Environment]::GetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL')
    [Environment]::SetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL', 'test-parent-only')
    try {
        Assert-ControlArchive $request $reference.current
        's06-owned-identity archive-current=observed'

        # Both leases hold genuine packages. Cross only the carrier
        # pin so the refusal must reach the launch owner rather than archive parsing.
        $request.Archive = $previous
        Assert-LaunchRefusal $request 'prestart-observer:identity-archive-digest'
        's06-owned-identity archive-pin-crossed=refused'

        $request.ExpectedArchiveSha256 = $reference.previous.ArchiveSha256
        Assert-ControlArchive $request $reference.previous
        's06-owned-identity archive-previous=observed'

        $request.Archive = $current
        $request.ExpectedArchiveSha256 = $reference.current.ArchiveSha256

        [hashtable]$wrongScope = $request.Clone()
        $wrongScope.Clock = [Diagnostics.Stopwatch]::StartNew()
        Assert-LaunchRefusal $wrongScope 'prestart-observer:archive-scope'
        $wrongScope.Clock = $clock
        $wrongScope.TimeoutMilliseconds = $budget + 1
        Assert-LaunchRefusal $wrongScope 'prestart-observer:archive-scope'
        's06-owned-identity archive-scope=refused'

        Assert-ArchiveHeld $currentPath $reference.current.ArchiveSha256
        Assert-ArchiveHeld $previousPath $reference.previous.ArchiveSha256

        # Retire the archive between actual image confirmation and final publication.
        Assert-ArchivePublicationRefusal $request
        Assert-ArchiveReleased $currentPath
        's06-owned-identity archive-publication-retirement=refused'

        $current = Open-PrestartArchive -ArchivePath $currentPath `
            -ExpectedArchiveBytes $reference.current.ArchiveBytes `
            -ExpectedArchiveSha256 $reference.current.ArchiveSha256 -Clock $clock -TimeoutMilliseconds $budget
        $request.Archive = $current
        Assert-ControlArchive $request $reference.current
        's06-owned-identity archive-publication-fresh=observed'
        's06-owned-identity archive-restored=observed'

        # Hand the real child to its original collector after retiring the
        # caller's archive lease. The child's borrow must hold the file until disposal.
        $retainInputDirectory = $true
        [object]$child = Start-ControlIdentity $request
        [int]$childId = $child.ProcessId
        $probeFailure = $null
        try {
            $request.Archive.Dispose()
            $current = $null
            Assert-ArchiveHeld $currentPath $reference.current.ArchiveSha256
        } catch {
            $probeFailure = $_
        }

        $collectionFailure = $null
        [bool]$archiveRetired = $false
        try {
            $null = & $module {
                param($Child)
                Receive-OwnedPrestartChild $Child
            } $child
            $retainInputDirectory = $false
        } catch {
            $collectionFailure = $_
            [Exception]$cause = $_.Exception
            while ($null -ne $cause) {
                if ($cause.Message -ceq 'prestart-observer:archive-retired') { $archiveRetired = $true }
                if ($cause.Data['PrestartObserverProcessId'] -eq $childId -and
                    $cause.Data['PrestartObserverRetired'] -eq $true -and
                    $cause.Data['PrestartObserverReleased'] -eq $true) {
                    $retainInputDirectory = $false
                }
                $cause = $cause.InnerException
            }
            if ($retainInputDirectory -and -not $child.DisposalStarted) {
                # Keep the original live reference for recovery in this process.
                $_.Exception.Data['PrestartArchiveControlChild'] = [object[]]@($child)
            }
        }
        if ($null -ne $probeFailure) {
            if ($null -ne $collectionFailure) {
                $probeFailure.Exception.Data['PrestartArchiveControlCollectionFailure'] = $collectionFailure
            }
            throw $probeFailure
        }
        if ($retainInputDirectory -or -not $archiveRetired) {
            if ($null -ne $collectionFailure) { throw $collectionFailure }
            throw 'Identity capture succeeded after the caller retired its archive lease'
        }
        Assert-ArchiveReleased $currentPath
        's06-owned-identity archive-retained-by-child=observed'
        Assert-LaunchRefusal $request 'prestart-observer:archive-retired'
        Assert-SignatureRefusal {
            $request.Archive.VerifySignature($request.ExpectedArchiveSha256, $testSigner, $actor, $clock, $budget)
        } ([InvalidOperationException]) 'prestart-observer:archive-retired'
        $previous.Dispose()
        $previous = $null
        Assert-ArchiveReleased $previousPath
        's06-owned-identity archive-retirement=observed'
    } finally {
        [Environment]::SetEnvironmentVariable('NEVROTICK_S06_IDENTITY_SENTINEL', $prior)
    }
    's06-owned-identity bounded-raw-output=observed'
} catch {
    $testFailure = $_
    if ($retainInputDirectory) { $_.Exception.Data['PrestartArchiveControlInputDirectory'] = $root }
    throw
} finally {
    try {
        if ($null -ne $previous) { $previous.Dispose() }
        if ($null -ne $current) { $current.Dispose() }
        if ($null -ne $actor) { $actor.Dispose() }
        if ($null -ne $module) { Remove-Module $module }
        if (-not $retainInputDirectory) { [IO.Directory]::Delete($root, $true) }
    } catch {
        if ($null -eq $testFailure) { throw }
        $testFailure.Exception.Data['PrestartArchiveControlCleanupFailure'] = $_
    }
}
