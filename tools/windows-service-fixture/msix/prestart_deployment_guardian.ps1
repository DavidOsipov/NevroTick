# SPDX-License-Identifier: GPL-2.0-only
# Launch one pinned owner in a new native job for the S0.6 laboratory pair.
[CmdletBinding(PositionalBinding = $false)]
param(
    [Parameter(Mandatory)][string]$EntryPath,
    [Parameter(Mandatory)][long]$EntryBytes,
    [Parameter(Mandatory)][ValidatePattern('\A[0-9a-f]{64}\z')][string]$EntrySha256,
    [Parameter(Mandatory)][long]$PowerShellBytes,
    [Parameter(Mandatory)][ValidatePattern('\A[0-9a-f]{64}\z')][string]$PowerShellSha256,
    [Parameter(Mandatory)][guid]$RunId,
    [Parameter(Mandatory)][ValidateRange(1, 119999)][int]$WorkMilliseconds,
    [Parameter(Mandatory)][ValidateRange(2, 120000)][int]$RetirementMilliseconds
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Read-FailureType([Exception]$Failure) {
    while ($Failure -is [Management.Automation.MethodInvocationException] -and
        $null -ne $Failure.InnerException) {
        $Failure = $Failure.InnerException
    }
    if ($null -ne $Failure) {
        [string]$kind = $Failure.GetType().Name
        if ($Failure.Message -cmatch '\Aprestart-(deployment-job|observer):[a-z-]{1,48}\z') {
            return $kind + ' ' + $Failure.Message
        }
        return $kind
    }
}

[hashtable]$record = @{
    run = $RunId.ToString('D'); owner_pid = $null; owner_exit = $null
    job_empty = $false; recovered = $false; active_at_recovery = $null; total_processes = $null
    verified_process_limit = $null; total_at_recovery = $null
    limit_terminations_at_recovery = $null; limit_terminations_at_empty = $null
    recovery_start_tick = $null; termination_accepted_tick = $null; job_empty_tick = $null; tick_frequency = $null
    failure = $null; recovery_failure = $null; members_at_recovery = $null
}
try {
    if ($PSVersionTable.PSEdition -cne 'Desktop' -or !$PSVersionTable.PSVersion.ToString().StartsWith('5.1.') -or
        ![Environment]::Is64BitProcess) {
        throw [PlatformNotSupportedException]::new('prestart-deployment-guardian:runtime')
    }
    [string[]]$names = @('prestart_actor_token.cs', 'prestart_archive_facts.cs',
        'prestart_observer_child.cs', 'prestart_scm_window.cs', 'prestart_deployment_job.cs')
    [string[]]$sources = @($names | ForEach-Object { Join-Path $PSScriptRoot $_ })
    Add-Type -LiteralPath $sources -ReferencedAssemblies @('System.dll', 'System.Core.dll') -WarningAction Stop
    [object]$report = [NevroTick.PrestartObserverChild]::RunDeploymentOwnerJob($EntryPath, $EntryBytes, $EntrySha256,
        $PowerShellBytes, $PowerShellSha256, $RunId, $WorkMilliseconds, $RetirementMilliseconds)
    $record.owner_pid = $report.Pid
    $record.owner_exit = $report.ExitCode
    $record.job_empty = $report.JobEmpty
    $record.recovered = $report.Recovered
    $record.active_at_recovery = $report.ActiveAtRecovery
    $record.total_processes = $report.TotalProcesses
    $record.members_at_recovery = $report.MemberPidsAtRecovery
    $record.verified_process_limit = $report.VerifiedProcessLimit
    $record.total_at_recovery = $report.TotalAtRecovery
    $record.limit_terminations_at_recovery = $report.LimitTerminationsAtRecovery
    $record.limit_terminations_at_empty = $report.LimitTerminationsAtEmpty
    $record.recovery_start_tick = $report.RecoveryStartTick
    $record.termination_accepted_tick = $report.TerminationAcceptedTick
    $record.job_empty_tick = $report.JobEmptyTick
    $record.tick_frequency = $report.TickFrequency
    $record.failure = Read-FailureType $report.Failure
    $record.recovery_failure = Read-FailureType $report.RecoveryFailure
} catch {
    $record.failure = Read-FailureType $_.Exception
}
# This record is diagnostic. The independent guardian exit carries success.
[Console]::Error.WriteLine('nevrotick-deployment-job ' + (ConvertTo-Json -InputObject $record -Compress))
if ($record.owner_exit -eq 0 -and $record.job_empty -and !$record.recovered -and
    $null -eq $record.failure -and $null -eq $record.recovery_failure) {
    exit 0
}
exit 1
