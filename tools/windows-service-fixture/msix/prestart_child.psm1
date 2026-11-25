# SPDX-License-Identifier: GPL-2.0-only
Set-StrictMode -Version Latest

function Assert-PrestartDeadline {
    param([Diagnostics.Stopwatch]$Clock, [int]$TimeoutMilliseconds, [string]$Stage)

    if (-not $Clock.IsRunning) {
        throw [ArgumentException]::new('prestart-child:clock-not-running')
    }
    if ($Clock.ElapsedMilliseconds -ge $TimeoutMilliseconds) {
        throw [TimeoutException]::new($Stage)
    }
}

# Keep both existing Process callers and the native launcher on the same
# bounded raw stream reader. Only the owner can confirm original process exit.
function Receive-PrestartStreams {
    param(
        [int]$ProcessId, [IO.Stream]$StdoutStream, [IO.Stream]$StderrStream,
        [scriptblock]$ObserveExit, [scriptblock]$RequireTime, [ref]$CollectedCapture
    )

    $limit = 8192
    $stdout = [pscustomobject]@{ Name = 'stdout'; Stream = $StdoutStream;
        Bytes = [byte[]]::new($limit); Used = 0; Eof = $false; Pending = $null }
    $stderr = [pscustomobject]@{ Name = 'stderr'; Stream = $StderrStream;
        Bytes = [byte[]]::new($limit); Used = 0; Eof = $false; Pending = $null }
    $channels = @($stdout, $stderr)

    try {
        while ($true) {
            & $RequireTime
            $advanced = $false
            foreach ($channel in $channels) {
                if ($channel.Eof) { continue }
                if ($null -ne $channel.Pending) {
                    if (-not $channel.Pending.IsCompleted) { continue }
                    $completed = $channel.Pending
                    $channel.Pending = $null
                    $count = $completed.GetAwaiter().GetResult()
                    $advanced = $true
                    if ($count -eq 0) { $channel.Eof = $true; continue }
                    if ($count -gt ($limit - $channel.Used)) {
                        throw [IO.InvalidDataException]::new('prestart-child:' + $channel.Name + '-limit')
                    }
                    $channel.Used += $count
                }
                & $RequireTime
                if ($channel.Used -eq $limit) {
                    # Probe EOF in place. A returned byte refuses the entire capture.
                    $channel.Pending = $channel.Stream.ReadAsync($channel.Bytes, 0, 1)
                } else {
                    $channel.Pending = $channel.Stream.ReadAsync(
                        $channel.Bytes, $channel.Used, $limit - $channel.Used)
                }
            }
            $exitCode = $null
            if ($stdout.Eof -and $stderr.Eof) {
                & $RequireTime
                $exitCode = & $ObserveExit
            }
            if ($null -ne $exitCode) {
                foreach ($channel in $channels) {
                    if ($channel.Used -eq $limit) { continue }
                    $bytes = [byte[]]::new($channel.Used)
                    [Array]::Copy($channel.Bytes, $bytes, $channel.Used)
                    $channel.Bytes = $bytes
                }
                [object]$capture = [pscustomobject]@{ ProcessId = $ProcessId; ExitCode = $exitCode;
                    Stdout = $stdout.Bytes; Stderr = $stderr.Bytes }
                if ($null -ne $CollectedCapture) {
                    # Complete historical streams survive refusal of the final
                    # work, input confirmation and publication checks.
                    $CollectedCapture.Value = [pscustomobject]@{ Capture = $capture; InputDigests = $null }
                }
                & $RequireTime
                return $capture
            }
            if (-not $advanced) { Start-Sleep -Milliseconds 5 }
        }
    } catch {
        # The caller retires the original child and settles these reads before
        # closing the streams. A timeout does not cancel an anonymous-pipe read.
        $_.Exception.Data['PrestartChildPendingReads'] = @(
            foreach ($channel in $channels) {
                if ($null -ne $channel.Pending) { $channel.Pending }
            }
        )
        throw
    }
}

# The caller lends the original Process and is responsible for retirement on
# refusal, including any pending read tasks attached by the stream collector.
function Receive-PrestartChild {
    param(
        [Parameter(Mandatory = $true)][Diagnostics.Process]$Process,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 2147483647)][int]$TimeoutMilliseconds
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
    Receive-PrestartStreams $Process.Id $Process.StandardOutput.BaseStream $Process.StandardError.BaseStream {
        if ($Process.WaitForExit(0)) { $Process.ExitCode }
    } {
        Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
    }
}

# Preserve the original failure with its pending reads. On uncertainty the
# caller still owns the child through the exception, and receives no capture.
function Wait-PrestartObserverRetirement {
    param([object]$Child, [Management.Automation.ErrorRecord]$Failure)

    $pending = @()
    $retirementClock = [Diagnostics.Stopwatch]::StartNew()
    if ($null -ne $Failure) {
        if ($Failure.Exception.Data.Contains('PrestartChildPendingReads')) {
            $pending = @($Failure.Exception.Data['PrestartChildPendingReads'])
        }
        if (-not $Child.ObserveRetirement()) {
            try {
                $Child.Terminate()
            } catch {
                # An unsuccessful termination request does not establish that the child is still live.
                $Failure.Exception.Data['PrestartObserverTerminationFailure'] = $_.Exception
            }
        }
    }
    while (-not $Child.ObserveRetirement() -or
        @($pending | Where-Object { -not $_.IsCompleted }).Count -ne 0) {
        if ($retirementClock.ElapsedMilliseconds -ge 5000) {
            throw [TimeoutException]::new('prestart-observer:retirement-deadline')
        }
        Start-Sleep -Milliseconds 5
    }
}

$script:PrestartObserverType = $null
$script:PrestartActorType = $null
$script:PrestartArchiveLeaseType = $null
$script:PrestartScmWindowType = $null
$script:PrestartDeploymentImportAttempted = $false
$script:PrestartDeploymentAssemblyName = 'prestart_deployment_helpers'

function Initialize-PrestartObserverTypes {
    if ($null -ne $script:PrestartObserverType) { return }
    if ($script:PrestartDeploymentImportAttempted) { throw 'prestart-child:deployment-helper-unavailable' }
    foreach ($loaded in [AppDomain]::CurrentDomain.GetAssemblies()) {
        if ($loaded.GetName().Name -ieq $script:PrestartDeploymentAssemblyName) {
            throw 'prestart-child:deployment-helper-unavailable'
        }
    }
    [string[]]$sources = @((Join-Path $PSScriptRoot 'prestart_actor_token.cs'),
        (Join-Path $PSScriptRoot 'prestart_observer_child.cs'),
        (Join-Path $PSScriptRoot 'prestart_archive_facts.cs'))
    [string]$scmSource = Join-Path $PSScriptRoot 'prestart_scm_window.cs'
    [bool]$hasScmWindow = Test-Path -LiteralPath $scmSource -PathType Leaf
    if ($hasScmWindow) { $sources += $scmSource }
    [type[]]$types = @(Add-Type -LiteralPath $sources -ReferencedAssemblies @('System.dll', 'System.Core.dll', 'System.Security.dll') `
        -PassThru -ErrorAction Stop)
    Set-PrestartObserverTypes $types $hasScmWindow
}

function Set-PrestartObserverTypes {
    param([type[]]$Types, [bool]$HasScmWindow)

    [type[]]$launchers = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartObserverChild' })
    $actors = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartActor' })
    [type[]]$archives = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartArchiveFacts' })
    if ($launchers.Count -ne 1 -or $actors.Count -ne 1 -or $archives.Count -ne 1) {
        throw 'prestart-observer:launcher-type'
    }
    [type[]]$windows = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartScmWindow' })
    if ($hasScmWindow -and $windows.Count -ne 1) { throw 'prestart-scm-window:owner-type' }
    [type]$lease = $launchers[0].GetNestedType('ArchiveLease')
    if ($null -eq $lease) { throw 'prestart-observer:archive-lease-type' }
    if ($hasScmWindow) { $script:PrestartScmWindowType = $windows[0] }
    $script:PrestartArchiveLeaseType = $lease
    $script:PrestartActorType = $actors[0]
    $script:PrestartObserverType = $launchers[0]
}

# The dedicated deployment process loads its admitted sibling assembly
# without starting a compiler inside the outer job. The private staging owner
# supplies the independent pins and keeps control of the local namespace.
function Import-PrestartDeploymentHelpers {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][ValidateRange(1, 2097152)][int]$ExpectedBytes,
        [Parameter(Mandatory = $true)][string]$ExpectedSha256
    )

    if ($script:PrestartDeploymentImportAttempted -or $null -ne $script:PrestartObserverType) {
        throw 'prestart-child:deployment-helper-already-selected'
    }
    $script:PrestartDeploymentImportAttempted = $true

    [string]$fileName = $script:PrestartDeploymentAssemblyName + '.dll'
    if ($Path.Length -gt 512 -or $Path -cnotmatch '\A[A-Za-z]:\\[^\x00-\x1f"*/?<>|:]+\z' -or
        ![string]::Equals([IO.Path]::GetFullPath($Path), $Path, [StringComparison]::Ordinal) -or
        [IO.Path]::GetFileName($Path) -cne $fileName -or
        [IO.DriveInfo]::new($Path.Substring(0, 3)).DriveType -ne [IO.DriveType]::Fixed) {
        throw 'prestart-child:deployment-helper-path'
    }
    foreach ($component in $Path.Substring(3).Split('\')) {
        if ($component.Length -eq 0 -or $component.Length -gt 255 -or
            $component.EndsWith('.', [StringComparison]::Ordinal) -or
            $component.EndsWith(' ', [StringComparison]::Ordinal)) {
            throw 'prestart-child:deployment-helper-path'
        }
    }
    if ($ExpectedSha256.Length -ne 64 -or $ExpectedSha256 -cnotmatch '\A[0-9a-f]{64}\z') {
        throw 'prestart-child:deployment-helper-sha256'
    }

    [string[]]$helperNames = @('NevroTick.PrestartObserverChild', 'NevroTick.PrestartActor',
        'NevroTick.PrestartArchiveFacts', 'NevroTick.PrestartScmWindow')
    [string]$assemblyName = $script:PrestartDeploymentAssemblyName
    foreach ($loaded in [AppDomain]::CurrentDomain.GetAssemblies()) {
        if ($loaded.GetName().Name -ieq $assemblyName) {
            throw 'prestart-child:deployment-helper-already-loaded'
        }
        foreach ($name in $helperNames) {
            if ($null -ne $loaded.GetType($name, $false, $false)) {
                throw 'prestart-child:deployment-helper-already-loaded'
            }
        }
    }

    [IO.FileStream]$stream = [IO.File]::Open($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        if ($stream.Length -ne $ExpectedBytes) { throw 'prestart-child:deployment-helper-length' }
        [byte[]]$bytes = [byte[]]::new($ExpectedBytes)
        [int]$offset = 0
        while ($offset -lt $bytes.Length) {
            [int]$count = $stream.Read($bytes, $offset, $bytes.Length - $offset)
            if ($count -eq 0) { throw 'prestart-child:deployment-helper-length' }
            $offset += $count
        }
        if ($stream.ReadByte() -ne -1 -or $stream.Length -ne $ExpectedBytes) {
            throw 'prestart-child:deployment-helper-length'
        }

        [Security.Cryptography.SHA256]$sha256 = [Security.Cryptography.SHA256CryptoServiceProvider]::new()
        try {
            [string]$digest = [BitConverter]::ToString($sha256.ComputeHash($bytes)).Replace('-', '').ToLowerInvariant()
        } finally {
            $sha256.Dispose()
        }
        if ($digest -cne $ExpectedSha256) { throw 'prestart-child:deployment-helper-digest' }
        [Reflection.Assembly]$assembly = [Reflection.Assembly]::Load($bytes)
    } finally {
        $stream.Dispose()
    }
    if ($assembly.GetName().Name -cne $assemblyName -or $assembly.GlobalAssemblyCache -or
        $assembly.Location -cne '') {
        throw 'prestart-child:deployment-helper-assembly'
    }

    [type[]]$types = foreach ($name in $helperNames) {
        [type]$helper = $assembly.GetType($name, $false, $false)
        if ($null -eq $helper -or ![object]::ReferenceEquals($helper.Assembly, $assembly) -or
            !$helper.IsPublic -or !$helper.IsClass -or !$helper.IsSealed) {
            throw 'prestart-child:deployment-helper-type'
        }
        $helper
    }
    Set-PrestartObserverTypes $types $true
}

# Observe the actual current primary token. The caller owns the query-only
# reference and must separately establish the actor's eligibility for its role.
function Open-PrestartObserverActor {
    param(
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:launch-deadline'
    Initialize-PrestartObserverTypes
    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:launch-deadline'
    $actorType = $script:PrestartActorType
    return $actorType::CaptureCurrent($Clock, $TimeoutMilliseconds)
}

# Same-actor observation in a dedicated launcher process. The caller lends
# the live actor, admitted image and unreset deadline. The provisioning and
# standard-user selectors require their own independently admitted subject SIDs.
# Child launches are serialized to keep the transient inheritable handles private.
function Invoke-PrestartObserver {
    param(
        [Parameter(Mandatory = $true)][string]$ImagePath,
        [Parameter(Mandatory = $true)][long]$ExpectedBytes,
        [Parameter(Mandatory = $true)][string]$ExpectedSha256,
        [Parameter(Mandatory = $true)][string]$Selector,
        [Parameter(Mandatory = $true)][object]$Actor,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds,
        [string]$ExpectedStandardUserSid,
        [string]$ExpectedProvisioningUserSid,
        [ref]$CollectedCapture
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:launch-deadline'
    if ($null -eq $script:PrestartActorType -or -not $script:PrestartActorType.IsInstanceOfType($Actor)) {
        throw [ArgumentException]::new('prestart-observer:caller-actor')
    }
    if ($null -ne $CollectedCapture -and $null -ne $CollectedCapture.Value) {
        throw [ArgumentException]::new('prestart-observer:capture-slot-used')
    }
    $launcher = $script:PrestartObserverType
    $child = $launcher::Start($ImagePath, $ExpectedBytes, $ExpectedSha256, $Selector, $Actor, $Clock,
        $TimeoutMilliseconds, $ExpectedStandardUserSid, $ExpectedProvisioningUserSid)
    Receive-OwnedPrestartChild $child $CollectedCapture
}

# Hold a read lease on one admitted archive. The coordinator owns its disposal
# after observation, under the same clock and timeout used for acquisition.
function Open-PrestartArchive {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory = $true)][string]$ArchivePath,
        [Parameter(Mandatory = $true)][long]$ExpectedArchiveBytes,
        [Parameter(Mandatory = $true)][string]$ExpectedArchiveSha256,
        [string]$ExpectedPayloadSha256,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:archive-deadline'
    Initialize-PrestartObserverTypes
    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:archive-deadline'
    [type]$launcher = $script:PrestartObserverType
    if ($PSBoundParameters.ContainsKey('ExpectedPayloadSha256')) {
        return $launcher::OpenArchiveWithPayload($ArchivePath, $ExpectedArchiveBytes, $ExpectedArchiveSha256,
            $ExpectedPayloadSha256, $Clock, $TimeoutMilliseconds)
    }
    return $launcher::OpenArchive($ArchivePath, $ExpectedArchiveBytes, $ExpectedArchiveSha256,
        $Clock, $TimeoutMilliseconds)
}

# Keep the identity launch within the original actor and deadline. The native
# launcher derives every package argument from the same inspected archive.
function Invoke-PrestartIdentity {
    [CmdletBinding(DefaultParameterSetName = 'ArchiveObservation', PositionalBinding = $false)]
    param(
        [Parameter(Mandatory = $true)][string]$PowerShellPath,
        [Parameter(Mandatory = $true)][long]$PowerShellBytes,
        [Parameter(Mandatory = $true)][string]$PowerShellSha256,
        [Parameter(Mandatory = $true)][string]$InputDirectory,
        [Parameter(Mandatory = $true)][long]$EntryBytes,
        [Parameter(Mandatory = $true)][string]$EntrySha256,
        [Parameter(Mandatory = $true)][long]$ModuleBytes,
        [Parameter(Mandatory = $true)][string]$ModuleSha256,
        [Parameter(Mandatory = $true)][long]$HelperBytes,
        [Parameter(Mandatory = $true)][string]$HelperSha256,
        [Parameter(Mandatory = $true)][guid]$RunId,
        [Parameter(Mandatory = $true)][object]$Archive,
        [Parameter(Mandatory = $true, ParameterSetName = 'ArchiveObservation')]
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedArchive')][string]$ExpectedUserSid,
        [Parameter(Mandatory = $true)][string]$ExpectedArchiveSha256,
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedArchive')]
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedWindow')]
        [string]$ExpectedSignerSha256,
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedWindow')][object]$Window,
        [Parameter(Mandatory = $true, ParameterSetName = 'ArchiveObservation')]
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedArchive')][object]$Actor,
        [Parameter(Mandatory = $true, ParameterSetName = 'ArchiveObservation')]
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedArchive')][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true, ParameterSetName = 'ArchiveObservation')]
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedArchive')][ValidateRange(1, 120000)][int]$TimeoutMilliseconds
    )

    if ($PSCmdlet.ParameterSetName -eq 'VerifiedWindow') {
        if ($null -eq $script:PrestartScmWindowType -or !$script:PrestartScmWindowType.IsInstanceOfType($Window)) {
            throw [ArgumentException]::new('prestart-scm-window:deployment-scope')
        }
        if (!$script:PrestartArchiveLeaseType.IsInstanceOfType($Archive)) {
            throw [ArgumentException]::new('prestart-observer:archive-lease')
        }
        [object]$identityChild = $Window.StartVerifiedIdentity($PowerShellPath, $PowerShellBytes, $PowerShellSha256,
            $InputDirectory, $EntryBytes, $EntrySha256, $ModuleBytes, $ModuleSha256, $HelperBytes, $HelperSha256,
            $RunId, $Archive, $ExpectedArchiveSha256, $ExpectedSignerSha256)
        Receive-OwnedPrestartChild $identityChild
        return
    }

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:launch-deadline'
    if ($null -eq $script:PrestartActorType -or -not $script:PrestartActorType.IsInstanceOfType($Actor)) {
        throw [ArgumentException]::new('prestart-observer:caller-actor')
    }
    if (!$script:PrestartArchiveLeaseType.IsInstanceOfType($Archive)) {
        throw [ArgumentException]::new('prestart-observer:archive-lease')
    }
    [type]$launcher = $script:PrestartObserverType
    [object]$child = $null
    if ($PSCmdlet.ParameterSetName -eq 'VerifiedArchive') {
        # The enclosing laboratory process owns the deadline through the return of native verification.
        [void]$Archive.VerifySignature($ExpectedArchiveSha256, $ExpectedSignerSha256, $Actor, $Clock, $TimeoutMilliseconds)
        $child = $launcher::StartVerifiedIdentity($PowerShellPath, $PowerShellBytes, $PowerShellSha256,
            $InputDirectory, $EntryBytes, $EntrySha256, $ModuleBytes, $ModuleSha256, $HelperBytes, $HelperSha256,
            $RunId, $Archive, $ExpectedUserSid, $ExpectedArchiveSha256, $ExpectedSignerSha256,
            $Actor, $Clock, $TimeoutMilliseconds)
    } else {
        # Existing inert observation controls use the archive facts alone.
        $child = $launcher::StartIdentity($PowerShellPath, $PowerShellBytes, $PowerShellSha256,
            $InputDirectory, $EntryBytes, $EntrySha256, $ModuleBytes, $ModuleSha256, $HelperBytes, $HelperSha256,
            $RunId, $Archive, $ExpectedUserSid, $ExpectedArchiveSha256, $Actor, $Clock, $TimeoutMilliseconds)
    }
    Receive-OwnedPrestartChild $child
}

function Receive-OwnedPrestartChild {
    param([object]$Child, [ref]$CollectedCapture)

    $childId = $child.ProcessId
    $failure = $null
    $inputDigests = $null
    $requireCollectionTime = {
        try { $Child.RequireCollectionTime() } catch {
            if ($_.Exception -is [Management.Automation.MethodInvocationException] -and
                $_.Exception.InnerException -is [TimeoutException]) {
                throw $_.Exception.InnerException
            }
            throw
        }
    }
    try {
        & $requireCollectionTime
        $capture = Receive-PrestartStreams $childId $child.Stdout $child.Stderr {
            if ($child.HasExited) { $child.ExitCode }
        } $requireCollectionTime $CollectedCapture
        while ($true) {
            & $requireCollectionTime
            if ($child.IsJobEmpty) { break }
            Start-Sleep -Milliseconds 5
        }
        & $requireCollectionTime
        $inputDigests = $child.ConfirmImage()
        if ($null -ne $CollectedCapture) { $CollectedCapture.Value.InputDigests = $inputDigests }
    } catch {
        $failure = $_
    } finally {
        $retired = $false
        try {
            Wait-PrestartObserverRetirement $child $failure
            $retired = $true
            $child.Dispose()
        } catch {
            if ($null -eq $failure) { $failure = $_ }
            else { $failure.Exception.Data['PrestartObserverCleanupFailure'] = $_ }
            $failure.Exception.Data['PrestartObserverProcessId'] = $childId
            $failure.Exception.Data['PrestartObserverRetired'] = $retired
            $failure.Exception.Data['PrestartObserverReleased'] = $false
            if (-not $retired -or -not $child.DisposalStarted) {
                # Keep the original live reference for recovery in this process.
                $failure.Exception.Data['PrestartObserverChild'] = [object[]]@($child)
            }
            throw $failure
        }
    }
    if ($null -ne $failure) {
        $failure.Exception.Data['PrestartObserverProcessId'] = $childId
        $failure.Exception.Data['PrestartObserverRetired'] = $true
        $failure.Exception.Data['PrestartObserverReleased'] = $true
        throw $failure
    }
    try {
        $child.ConfirmPublication()
    } catch {
        [Exception]$publicationFailure = $_.Exception
        if ($publicationFailure -is [Management.Automation.MethodInvocationException] -and
            $null -ne $publicationFailure.InnerException) {
            $publicationFailure = $publicationFailure.InnerException
        }
        $publicationFailure.Data['PrestartObserverProcessId'] = $childId
        $publicationFailure.Data['PrestartObserverRetired'] = $true
        $publicationFailure.Data['PrestartObserverReleased'] = $child.ReleaseConfirmed
        throw $publicationFailure
    }
    return [pscustomobject]@{
        ProcessId = $capture.ProcessId; ExitCode = $capture.ExitCode;
        Stdout = $capture.Stdout; Stderr = $capture.Stderr;
        InputSha256Before = $inputDigests.Before; InputSha256After = $inputDigests.After
    }
}

# Acquire the two fixed SCM objects under the admitted deployment actor.
# The caller owns the resulting lease and must inspect its retirement result.
function Open-PrestartScmWindow {
    param(
        [Parameter(Mandatory = $true)][object]$Actor,
        [Parameter(Mandatory = $true)][string]$ExpectedDeploymentSid,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-scm-window:open-deadline'
    Initialize-PrestartObserverTypes
    if ($null -eq $script:PrestartScmWindowType) { throw 'prestart-scm-window:owner-type' }
    if (-not $script:PrestartActorType.IsInstanceOfType($Actor)) { throw 'prestart-scm-window:actor' }
    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-scm-window:open-deadline'
    [type]$windowType = $script:PrestartScmWindowType
    return $windowType::Open($Actor, $ExpectedDeploymentSid, $Clock, $TimeoutMilliseconds)
}

# Observe through the same window after the coordinator validates the identity receipt.
function Invoke-PrestartVerifiedScmObservation {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory = $true)][object]$Window,
        [Parameter(Mandatory = $true)][string]$ImagePath,
        [Parameter(Mandatory = $true)][long]$ExpectedBytes,
        [Parameter(Mandatory = $true)][string]$ExpectedSha256
    )

    if ($null -eq $script:PrestartScmWindowType -or !$script:PrestartScmWindowType.IsInstanceOfType($Window)) {
        throw [ArgumentException]::new('prestart-scm-window:deployment-scope')
    }
    [object]$child = $Window.StartVerifiedObservation($ImagePath, $ExpectedBytes, $ExpectedSha256)
    Receive-OwnedPrestartChild $child
}

Export-ModuleMember -Function Receive-PrestartChild, Open-PrestartObserverActor, Invoke-PrestartObserver, `
    Open-PrestartArchive, Invoke-PrestartIdentity, Open-PrestartScmWindow, Invoke-PrestartVerifiedScmObservation, `
    Import-PrestartDeploymentHelpers
