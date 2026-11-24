# SPDX-License-Identifier: GPL-2.0-only
# Exercise the owned collector without creating processes or touching Windows state.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

public enum PrestartRetirementFault { None, AfterDisposalStart, Observation, BeforeDisposalStart, InitialObservation }

public sealed class PrestartRetirementControl : IDisposable
{
    private sealed class ControlledStream : MemoryStream
    {
        private readonly Task<int> read;

        public ControlledStream(Task<int> read) { this.read = read; }

        public override Task<int> ReadAsync(byte[] buffer, int offset, int count, CancellationToken cancellationToken)
        {
            return read;
        }
    }

    private readonly TaskCompletionSource<int> pending = new TaskCompletionSource<int>();
    private readonly PrestartRetirementFault fault;
    public readonly Exception CollectionFailure = new IOException("controlled stream read failed");
    public readonly Exception TerminationFailure = new InvalidOperationException("controlled termination failed");
    public readonly Exception ObservationFailure = new IOException("controlled job observation failed");
    public readonly Exception DisposalFailure = new InvalidOperationException("controlled disposal failed");
    public readonly int ProcessId = 413;
    public readonly int ExitCode = 23;
    public readonly Stream Stdout;
    public readonly Stream Stderr;
    public int ExitChecks { get; private set; }
    public int JobChecks { get; private set; }
    public int Terminations { get; private set; }
    public int Disposals { get; private set; }
    public bool DisposalStarted { get; private set; }
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
            Stdout = new MemoryStream(new byte[] { 255 });
            Stderr = new MemoryStream(new byte[] { 128 });
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

    public object ConfirmImage() { return new { Before = "held-before", After = "held-after" }; }

    public bool ObserveRetirement() { return HasExited && IsJobEmpty; }

    public void Terminate()
    {
        Terminations++;
        throw TerminationFailure;
    }

    public void Dispose()
    {
        Disposals++;
        if (!pending.Task.IsCompleted) throw new InvalidOperationException("disposed with a pending read");
        if (fault == PrestartRetirementFault.BeforeDisposalStart) throw DisposalFailure;
        DisposalStarted = true;
        Stdout.Dispose();
        Stderr.Dispose();
        if (fault == PrestartRetirementFault.AfterDisposalStart) throw DisposalFailure;
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

$module = Import-Module (Join-Path $PSScriptRoot 'prestart_child.psm1') -Force -PassThru
try {
    $healthy = [PrestartRetirementControl]::new($false, [PrestartRetirementFault]::None)
    $capture = @(& $module { param($child) Receive-OwnedPrestartChild $child ([Diagnostics.Stopwatch]::StartNew()) 5000 } $healthy)
    if ($capture.Count -ne 1 -or $capture[0].ProcessId -ne 413 -or $capture[0].ExitCode -ne 23 -or
        $capture[0].Stdout.Length -ne 1 -or $capture[0].Stdout[0] -ne 255 -or
        $capture[0].Stderr.Length -ne 1 -or $capture[0].Stderr[0] -ne 128 -or
        $capture[0].InputSha256Before -cne 'held-before' -or $capture[0].InputSha256After -cne 'held-after' -or
        $healthy.Terminations -ne 0 -or $healthy.Disposals -ne 1 -or !$healthy.DisposalStarted) {
        throw 'healthy collector did not return one retired capture'
    }
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
            & $module { param($child) Receive-OwnedPrestartChild $child ([Diagnostics.Stopwatch]::StartNew()) 5000 } $fault |
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
            $fault.DisposalStarted -ne $case.Started) {
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

's06-prestart-retirement controls=6'
