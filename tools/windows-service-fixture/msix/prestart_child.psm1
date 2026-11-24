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
        [scriptblock]$ObserveExit, [Diagnostics.Stopwatch]$Clock, [int]$TimeoutMilliseconds
    )

    $limit = 8192
    $stdout = [pscustomobject]@{ Name = 'stdout'; Stream = $StdoutStream;
        Bytes = [byte[]]::new($limit); Used = 0; Eof = $false; Pending = $null }
    $stderr = [pscustomobject]@{ Name = 'stderr'; Stream = $StderrStream;
        Bytes = [byte[]]::new($limit); Used = 0; Eof = $false; Pending = $null }
    $channels = @($stdout, $stderr)

    try {
        while ($true) {
            if ($Clock.ElapsedMilliseconds -ge $TimeoutMilliseconds) {
                throw [TimeoutException]::new('prestart-child:collection-deadline')
            }
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
                Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
                if ($channel.Used -eq $limit) {
                    # Probe EOF in place. A returned byte refuses the entire capture.
                    $channel.Pending = $channel.Stream.ReadAsync($channel.Bytes, 0, 1)
                } else {
                    $channel.Pending = $channel.Stream.ReadAsync(
                        $channel.Bytes, $channel.Used, $limit - $channel.Used)
                }
            }
            $exitCode = $null
            if ($stdout.Eof -and $stderr.Eof) { $exitCode = & $ObserveExit }
            if ($null -ne $exitCode) {
                foreach ($channel in $channels) {
                    if ($channel.Used -eq $limit) { continue }
                    $bytes = [byte[]]::new($channel.Used)
                    [Array]::Copy($channel.Bytes, $bytes, $channel.Used)
                    $channel.Bytes = $bytes
                }
                if ($Clock.ElapsedMilliseconds -ge $TimeoutMilliseconds) {
                    throw [TimeoutException]::new('prestart-child:collection-deadline')
                }
                return [pscustomobject]@{ ProcessId = $processId; ExitCode = $exitCode;
                    Stdout = $stdout.Bytes; Stderr = $stderr.Bytes }
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
    } $Clock $TimeoutMilliseconds
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
function Initialize-PrestartObserverTypes {
    if ($null -ne $script:PrestartObserverType) { return }
    [string[]]$sources = @((Join-Path $PSScriptRoot 'prestart_actor_token.cs'),
        (Join-Path $PSScriptRoot 'prestart_observer_child.cs'),
        (Join-Path $PSScriptRoot 'prestart_archive_facts.cs'))
    $types = @(Add-Type -LiteralPath $sources -ReferencedAssemblies @('System.dll', 'System.Core.dll') `
        -PassThru -ErrorAction Stop)
    $launchers = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartObserverChild' })
    $actors = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartActor' })
    [type[]]$archives = @($types | Where-Object { $_.FullName -ceq 'NevroTick.PrestartArchiveFacts' })
    if ($launchers.Count -ne 1 -or $actors.Count -ne 1 -or $archives.Count -ne 1) {
        throw 'prestart-observer:launcher-type'
    }
    [type]$lease = $launchers[0].GetNestedType('ArchiveLease')
    if ($null -eq $lease) { throw 'prestart-observer:archive-lease-type' }
    $script:PrestartArchiveLeaseType = $lease
    $script:PrestartActorType = $actors[0]
    $script:PrestartObserverType = $launchers[0]
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
# the live actor, admitted image and unreset deadline. The standard-user
# selector also requires an independently admitted ExpectedStandardUserSid.
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
        [string]$ExpectedStandardUserSid
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:launch-deadline'
    if ($null -eq $script:PrestartActorType -or -not $script:PrestartActorType.IsInstanceOfType($Actor)) {
        throw [ArgumentException]::new('prestart-observer:caller-actor')
    }
    $launcher = $script:PrestartObserverType
    $child = $launcher::Start($ImagePath, $ExpectedBytes, $ExpectedSha256, $Selector, $Actor, $Clock,
        $TimeoutMilliseconds, $ExpectedStandardUserSid)
    Receive-OwnedPrestartChild $child $Clock $TimeoutMilliseconds
}

# Hold a read lease on one admitted archive. The coordinator owns its disposal
# after observation, under the same clock and timeout used for acquisition.
function Open-PrestartArchive {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory = $true)][string]$ArchivePath,
        [Parameter(Mandatory = $true)][long]$ExpectedArchiveBytes,
        [Parameter(Mandatory = $true)][string]$ExpectedArchiveSha256,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds
    )

    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:archive-deadline'
    Initialize-PrestartObserverTypes
    Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-observer:archive-deadline'
    [type]$launcher = $script:PrestartObserverType
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
        [Parameter(Mandatory = $true)][string]$ExpectedUserSid,
        [Parameter(Mandatory = $true)][string]$ExpectedArchiveSha256,
        [Parameter(Mandatory = $true, ParameterSetName = 'VerifiedArchive')]
        [string]$ExpectedSignerSha256,
        [Parameter(Mandatory = $true)][object]$Actor,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds
    )

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
    Receive-OwnedPrestartChild $child $Clock $TimeoutMilliseconds
}

function Receive-OwnedPrestartChild {
    param([object]$Child, [Diagnostics.Stopwatch]$Clock, [int]$TimeoutMilliseconds)

    $childId = $child.ProcessId
    $failure = $null
    $inputDigests = $null
    try {
        Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
        $capture = Receive-PrestartStreams $childId $child.Stdout $child.Stderr {
            if ($child.HasExited) { $child.ExitCode }
        } $Clock $TimeoutMilliseconds
        while (-not $child.IsJobEmpty) {
            Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
            Start-Sleep -Milliseconds 5
        }
        Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
        $inputDigests = $child.ConfirmImage()
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
        Assert-PrestartDeadline $Clock $TimeoutMilliseconds 'prestart-child:collection-deadline'
    } catch {
        $_.Exception.Data['PrestartObserverProcessId'] = $childId
        $_.Exception.Data['PrestartObserverRetired'] = $true
        $_.Exception.Data['PrestartObserverReleased'] = $true
        throw
    }
    return [pscustomobject]@{
        ProcessId = $capture.ProcessId; ExitCode = $capture.ExitCode;
        Stdout = $capture.Stdout; Stderr = $capture.Stderr;
        InputSha256Before = $inputDigests.Before; InputSha256After = $inputDigests.After
    }
}

Export-ModuleMember -Function Receive-PrestartChild, Open-PrestartObserverActor, Invoke-PrestartObserver, `
    Open-PrestartArchive, Invoke-PrestartIdentity
