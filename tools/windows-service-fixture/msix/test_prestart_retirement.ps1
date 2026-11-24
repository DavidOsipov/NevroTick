# SPDX-License-Identifier: GPL-2.0-only
# Exercise the owned collector without creating processes or touching Windows state.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

public enum PrestartRetirementFault
{
    None, AfterDisposalStart, Observation, BeforeDisposalStart, InitialObservation,
    CollectionEntry, CollectionAfterRead, CollectionAfterExit, ImageConfirmation, PublicationExpiry, PublicationArchiveRetired
}

public sealed class PrestartRetirementControl : IDisposable
{
    private sealed class ControlledStream : MemoryStream
    {
        private readonly Task<int> read;
        public int Reads { get; private set; }
        public Task<int> LastRead { get; private set; }

        public ControlledStream(Task<int> read) { this.read = read; }
        public ControlledStream(byte value) : base(new byte[] { value }) { }

        public override Task<int> ReadAsync(byte[] buffer, int offset, int count, CancellationToken cancellationToken)
        {
            Reads++;
            LastRead = read ?? base.ReadAsync(buffer, offset, count, cancellationToken);
            return LastRead;
        }
    }

    private readonly TaskCompletionSource<int> pending = new TaskCompletionSource<int>();
    private readonly PrestartRetirementFault fault;
    private bool exitObserved;
    public readonly Exception CollectionFailure = new IOException("controlled stream read failed");
    public readonly Exception TerminationFailure = new InvalidOperationException("controlled termination failed");
    public readonly Exception ObservationFailure = new IOException("controlled job observation failed");
    public readonly Exception ImageFailure = new IOException("controlled image confirmation failed");
    public readonly Exception DisposalFailure = new InvalidOperationException("controlled disposal failed");
    public readonly Exception ArchiveRetired = new InvalidOperationException("prestart-observer:archive-retired");
    public readonly TimeoutException CollectionExpiry = new TimeoutException("prestart-child:collection-deadline");
    public readonly int ProcessId = 413;
    public int ExitCode { get { exitObserved = true; return 23; } }
    public readonly object ConfirmedDigests = new { Before = "held-before", After = "held-after" };
    public readonly Stream Stdout;
    public readonly Stream Stderr;
    public int StdoutReads { get { return ((ControlledStream)Stdout).Reads; } }
    public int StderrReads { get { return ((ControlledStream)Stderr).Reads; } }
    public Task<int> StdoutReadTask { get { return ((ControlledStream)Stdout).LastRead; } }
    public int Confirms { get; private set; }
    public int PublicationChecks { get; private set; }
    public int Retirements { get; private set; }
    public int ExitChecks { get; private set; }
    public int JobChecks { get; private set; }
    public int Terminations { get; private set; }
    public int Disposals { get; private set; }
    public bool DisposalStarted { get; private set; }
    public bool ReleaseConfirmed { get; private set; }
    public Action<string> ObserveCapture { get; set; }
    public Task<int> PendingRead { get { return pending.Task; } }

    public PrestartRetirementControl(bool failCollection, PrestartRetirementFault fault)
    {
        this.fault = fault;
        if (failCollection)
        {
            var failed = new TaskCompletionSource<int>();
            failed.SetException(CollectionFailure);
            Stdout = new ControlledStream(failed.Task);
            Stderr = new ControlledStream(pending.Task);
        }
        else
        {
            pending.SetResult(0);
            Stdout = new ControlledStream((byte)255);
            Stderr = new ControlledStream((byte)128);
        }
    }

    public bool HasExited
    {
        get
        {
            if (fault == PrestartRetirementFault.InitialObservation)
            {
                ExitChecks++;
                throw ObservationFailure;
            }
            return Terminations == 0 ? pending.Task.IsCompleted : ++ExitChecks >= 2;
        }
    }

    public bool IsJobEmpty
    {
        get
        {
            if (Terminations == 0) return pending.Task.IsCompleted;
            JobChecks++;
            if (fault == PrestartRetirementFault.Observation) throw ObservationFailure;
            if (JobChecks == 3) pending.SetResult(0);
            return JobChecks >= 2;
        }
    }

    public void RequireCollectionTime()
    {
        if (fault == PrestartRetirementFault.CollectionEntry ||
            (fault == PrestartRetirementFault.CollectionAfterRead && StdoutReads != 0) ||
            (fault == PrestartRetirementFault.CollectionAfterExit && exitObserved) ||
            (fault == PrestartRetirementFault.PublicationExpiry && ReleaseConfirmed))
            throw CollectionExpiry;
    }

    public object ConfirmImage()
    {
        Confirms++;
        if (ObserveCapture != null) ObserveCapture("image");
        if (fault == PrestartRetirementFault.ImageConfirmation) throw ImageFailure;
        return ConfirmedDigests;
    }

    public void ConfirmPublication()
    {
        PublicationChecks++;
        if (!ReleaseConfirmed || Confirms != 1)
            throw new InvalidOperationException("publication checked before confirmed image and release");
        if (ObserveCapture != null) ObserveCapture("publication");
        if (fault == PrestartRetirementFault.PublicationExpiry)
            throw CollectionExpiry;
        if (fault == PrestartRetirementFault.PublicationArchiveRetired)
            throw ArchiveRetired;
    }

    public bool ObserveRetirement()
    {
        Retirements++;
        return HasExited && IsJobEmpty;
    }

    public void Terminate()
    {
        Terminations++;
        throw TerminationFailure;
    }

    public void Dispose()
    {
        Disposals++;
        if (ObserveCapture != null) ObserveCapture("disposal");
        if (!pending.Task.IsCompleted) throw new InvalidOperationException("disposed with a pending read");
        if (fault == PrestartRetirementFault.BeforeDisposalStart) throw DisposalFailure;
        DisposalStarted = true;
        Stdout.Dispose();
        Stderr.Dispose();
        if (fault == PrestartRetirementFault.AfterDisposalStart) throw DisposalFailure;
        ReleaseConfirmed = true;
    }
}
'@

function Test-ExceptionCause {
    param([Exception]$Actual, [Exception]$Expected)

    for ($cause = $Actual; $null -ne $cause; $cause = $cause.InnerException) {
        if ([object]::ReferenceEquals($cause, $Expected)) { return $true }
    }
    return $false
}

function Watch-ControlCapture {
    param([PrestartRetirementControl]$Child, [ref]$Slot)

    $observed = @{}
    $Child.ObserveCapture = [Action[string]]({
        param([string]$Phase)

        $evidence = $Slot.Value
        # Snapshot the current bytes and pins before this phase can change the slot.
        $observed[$Phase] = if ($null -ne $evidence) {
            [pscustomobject]@{
                ProcessId = $evidence.Capture.ProcessId; ExitCode = $evidence.Capture.ExitCode
                Stdout = $evidence.Capture.Stdout.Clone(); Stderr = $evidence.Capture.Stderr.Clone()
                InputDigests = $evidence.InputDigests
            }
        }
    }.GetNewClosure())
    return $observed
}

function Assert-OriginalCapture {
    param([object]$Capture, [string]$Label)

    if ($null -eq $Capture -or $Capture.ProcessId -ne 413 -or $Capture.ExitCode -ne 23 -or
        $Capture.Stdout -isnot [byte[]] -or $Capture.Stdout.Length -ne 1 -or $Capture.Stdout[0] -ne 255 -or
        $Capture.Stderr -isnot [byte[]] -or $Capture.Stderr.Length -ne 1 -or $Capture.Stderr[0] -ne 128) {
        throw "$Label did not preserve the original PID, exit code, and stream bytes"
    }
}

function Assert-CompleteControlCapture {
    param([object]$Evidence, [hashtable]$Observed, [PrestartRetirementControl]$Child, [bool]$ImageConfirmed)

    if ($null -eq $Evidence) { throw 'complete streams and exit were missing from the diagnostic slot' }
    Assert-OriginalCapture $Evidence.Capture 'diagnostic slot'
    if ($Child.Confirms -ne 0) {
        Assert-OriginalCapture $Observed['image'] 'image confirmation'
        if ($null -ne $Observed['image'].InputDigests) {
            throw 'image confirmation started with purportedly confirmed pins'
        }
    }
    $expectedDigests = if ($ImageConfirmed) { $Child.ConfirmedDigests }
    foreach ($phase in @('disposal', 'publication')) {
        if ($phase -eq 'publication' -and $Child.PublicationChecks -eq 0) { continue }
        $snapshot = $Observed[$phase]
        Assert-OriginalCapture $snapshot $phase
        if (![object]::ReferenceEquals($snapshot.InputDigests, $expectedDigests)) {
            throw "confirmed pins differed at $phase"
        }
    }
    if (![object]::ReferenceEquals($Evidence.InputDigests, $expectedDigests)) {
        throw 'diagnostic slot ended with different or unconfirmed pins'
    }
}

$module = Import-Module (Join-Path $PSScriptRoot 'prestart_child.psm1') -Force -PassThru
try {
    [hashtable[]]$failures = @(
        @{ Mode = [PrestartRetirementFault]::PublicationArchiveRetired; Stdout = 2; Stderr = 2
            Failure = 'ArchiveRetired'; Confirms = 1; PublicationChecks = 1
            Retirements = 1; Pending = $false }
        @{ Mode = [PrestartRetirementFault]::CollectionEntry; Stdout = 0; Stderr = 0
            Failure = 'CollectionExpiry'; Confirms = 0; PublicationChecks = 0
            Retirements = 2; Pending = $false }
        @{ Mode = [PrestartRetirementFault]::CollectionAfterRead; Stdout = 1; Stderr = 0
            Failure = 'CollectionExpiry'; Confirms = 0; PublicationChecks = 0
            Retirements = 2; Pending = $true }
        @{ Mode = [PrestartRetirementFault]::CollectionAfterExit; Stdout = 2; Stderr = 2
            Failure = 'CollectionExpiry'; Confirms = 0; PublicationChecks = 0
            Retirements = 2; Pending = $false }
        @{ Mode = [PrestartRetirementFault]::ImageConfirmation; Stdout = 2; Stderr = 2
            Failure = 'ImageFailure'; Confirms = 1; PublicationChecks = 0
            Retirements = 2; Pending = $false }
        @{ Mode = [PrestartRetirementFault]::PublicationExpiry; Stdout = 2; Stderr = 2
            Failure = 'CollectionExpiry'; Confirms = 1; PublicationChecks = 1
            Retirements = 1; Pending = $false }
        @{ Mode = [PrestartRetirementFault]::AfterDisposalStart; Stdout = 2; Stderr = 2
            Failure = 'DisposalFailure'; Confirms = 1; PublicationChecks = 0
            Retirements = 1; Pending = $false }
    )
    foreach ($case in $failures) {
        $fault = [PrestartRetirementControl]::new($false, $case.Mode)
        $collected = $null
        $slot = [ref]$collected
        $observed = Watch-ControlCapture $fault $slot
        $records = [Collections.Generic.List[object]]::new()
        $failure = $null
        try {
            & $module {
                param($child, [ref]$slot)
                Receive-OwnedPrestartChild $child -CollectedCapture $slot
            } $fault $slot |
                ForEach-Object { $records.Add($_) }
        } catch {
            $failure = $_.Exception
        }
        [Exception]$expectedFailure = $fault.PSObject.Properties[$case.Failure].Value
        if ($records.Count -ne 0 -or !(Test-ExceptionCause $failure $expectedFailure)) {
            throw "$($case.Mode) did not preserve the original failure without a capture (count=$($records.Count))"
        }
        if ($case.Failure -ceq 'CollectionExpiry' -and
            ![object]::ReferenceEquals($failure, $fault.CollectionExpiry)) {
            throw "$($case.Mode) did not preserve the typed cutoff"
        }
        [bool]$released = $case.Failure -cne 'DisposalFailure'
        $data = $failure.Data
        if ($data['PrestartObserverProcessId'] -ne 413 -or $data['PrestartObserverRetired'] -ne $true -or
            $data['PrestartObserverReleased'] -ne $released -or
            $data.Contains('PrestartObserverCleanupFailure') -or $data.Contains('PrestartObserverChild') -or
            $fault.StdoutReads -ne $case.Stdout -or $fault.StderrReads -ne $case.Stderr -or
            $fault.Confirms -ne $case.Confirms -or $fault.PublicationChecks -ne $case.PublicationChecks -or
            $fault.Retirements -ne $case.Retirements -or $fault.Terminations -ne 0 -or
            $fault.Disposals -ne 1 -or !$fault.DisposalStarted -or $fault.ReleaseConfirmed -ne $released) {
            throw "$($case.Mode) misreported read, confirmation, or retirement effects"
        }
        $reads = $data['PrestartChildPendingReads']
        if ($case.Pending) {
            if ($null -eq $reads -or $reads.Count -ne 1 -or
                ![object]::ReferenceEquals($reads[0], $fault.StdoutReadTask) -or !$reads[0].IsCompleted) {
                throw "$($case.Mode) failed to preserve the original read through retirement"
            }
        } elseif ($case.Mode -eq [PrestartRetirementFault]::CollectionAfterExit) {
            if ($reads -isnot [Array] -or $reads.Length -ne 0) {
                throw 'complete collection should have no pending reads at the cutoff'
            }
        } elseif ($null -ne $reads) {
            throw "$($case.Mode) reported an unexpected pending read"
        }
        if ($case.Confirms -eq 0 -and $case.Mode -ne [PrestartRetirementFault]::CollectionAfterExit) {
            if ($null -ne $collected -or !$observed.ContainsKey('disposal') -or $null -ne $observed['disposal']) {
                throw "$($case.Mode) retained a capture before both EOFs and exit"
            }
        } else {
            Assert-CompleteControlCapture $collected $observed $fault (
                $case.Confirms -ne 0 -and $case.Failure -cne 'ImageFailure')
        }
        "s06-prestart-retirement $($case.Mode) passed"
    }

    $healthy = [PrestartRetirementControl]::new($false, [PrestartRetirementFault]::None)
    $collected = $null
    $slot = [ref]$collected
    $observed = Watch-ControlCapture $healthy $slot
    $capture = @(& $module {
        param($child, [ref]$slot)
        Receive-OwnedPrestartChild $child -CollectedCapture $slot
    } $healthy $slot)
    if ($capture.Count -ne 1 -or
        $capture[0].InputSha256Before -cne 'held-before' -or $capture[0].InputSha256After -cne 'held-after' -or
        $healthy.Confirms -ne 1 -or $healthy.PublicationChecks -ne 1 -or $healthy.Retirements -ne 1 -or
        $healthy.Terminations -ne 0 -or $healthy.Disposals -ne 1 -or !$healthy.ReleaseConfirmed) {
        throw 'healthy collector did not return one retired capture'
    }
    Assert-OriginalCapture $capture[0] 'healthy collector output'
    Assert-CompleteControlCapture $collected $observed $healthy $true
    's06-prestart-retirement healthy passed'

    $cases = @(
        @{ Mode = [PrestartRetirementFault]::None; Retired = $true; Released = $true
            Terminations = 1; Disposals = 1; Started = $true; RetainChild = $false; Cleanup = $null
            Marker = 'termination-failure disposal-fails=False' }
        @{ Mode = [PrestartRetirementFault]::AfterDisposalStart; Retired = $true; Released = $false
            Terminations = 1; Disposals = 1; Started = $true; RetainChild = $false; Cleanup = 'DisposalFailure'
            Marker = 'termination-failure disposal-fails=True' }
        @{ Mode = [PrestartRetirementFault]::Observation; Retired = $false; Released = $false
            ExitChecks = 2; JobChecks = 1; Terminations = 1; Disposals = 0; Started = $false
            RetainChild = $true; Cleanup = 'ObservationFailure'
            Marker = 'retirement-observation-failure' }
        @{ Mode = [PrestartRetirementFault]::BeforeDisposalStart; Retired = $true; Released = $false
            Terminations = 1; Disposals = 1; Started = $false; RetainChild = $true; Cleanup = 'DisposalFailure'
            Marker = 'disposal-precheck-failure' }
        @{ Mode = [PrestartRetirementFault]::InitialObservation; Retired = $false; Released = $false
            ExitChecks = 1; JobChecks = 0; Terminations = 0; Disposals = 0; Started = $false
            RetainChild = $true; Cleanup = 'ObservationFailure'
            Marker = 'initial-retirement-observation-failure' }
    )
    foreach ($case in $cases) {
        $fault = [PrestartRetirementControl]::new($true, $case.Mode)
        $records = [Collections.Generic.List[object]]::new()
        $failure = $null
        try {
            & $module { param($child) Receive-OwnedPrestartChild $child } $fault |
                ForEach-Object { $records.Add($_) }
        } catch {
            $failure = $_.Exception
        }
        if ($records.Count -ne 0 -or !(Test-ExceptionCause $failure $fault.CollectionFailure)) {
            throw 'collector did not preserve the original read failure without a capture'
        }
        $data = $failure.Data
        $termination = $data['PrestartObserverTerminationFailure']
        if ($case.Terminations -eq 0) {
            if ($null -ne $termination) { throw 'collector reported a termination error before a request' }
        } elseif (!(Test-ExceptionCause $termination $fault.TerminationFailure)) {
            throw 'collector did not retain the original termination error'
        }
        $reads = $data['PrestartChildPendingReads']
        if ($null -eq $reads -or $reads.Count -ne 1 -or ![object]::ReferenceEquals($reads[0], $fault.PendingRead) -or
            $data['PrestartObserverProcessId'] -ne 413 -or $data['PrestartObserverRetired'] -ne $case.Retired -or
            $data['PrestartObserverReleased'] -ne $case.Released -or $fault.PendingRead.IsCompleted -ne $case.Retired -or
            $fault.Terminations -ne $case.Terminations -or $fault.Disposals -ne $case.Disposals -or
            $fault.DisposalStarted -ne $case.Started -or $fault.PublicationChecks -ne 0) {
            throw 'collector misreported original process, job, read, or disposal state'
        }
        if ($case.Retired) {
            if ($fault.ExitChecks -lt 4 -or $fault.JobChecks -lt 3) {
                throw 'collector omitted a retirement observation'
            }
        } elseif ($fault.ExitChecks -ne $case.ExitChecks -or $fault.JobChecks -ne $case.JobChecks) {
            throw 'collector did not stop at the first failed retirement observation'
        }
        if ($case.RetainChild) {
            $retained = $data['PrestartObserverChild']
            if ($retained -isnot [object[]] -or $retained.Length -ne 1 -or
                ![object]::ReferenceEquals($retained[0], $fault)) {
                throw 'collector did not retain the original child for local recovery'
            }
        } elseif ($data.Contains('PrestartObserverChild')) {
            throw 'collector retained a child after disposal started'
        }
        if ($null -ne $case.Cleanup) {
            $cleanup = $data['PrestartObserverCleanupFailure']
            $expectedCleanup = $fault.PSObject.Properties[$case.Cleanup].Value
            if ($null -eq $cleanup -or !(Test-ExceptionCause $cleanup.Exception $expectedCleanup)) {
                throw 'collector did not retain the separate cleanup error'
            }
        } elseif ($data.Contains('PrestartObserverCleanupFailure')) {
            throw 'collector recorded a cleanup failure after confirmed release'
        }
        "s06-prestart-retirement $($case.Marker) passed"
    }
} finally {
    Remove-Module $module
}

's06-prestart-retirement controls={0}' -f ($failures.Length + $cases.Length + 1)
