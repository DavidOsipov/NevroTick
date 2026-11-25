# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
#Requires -PSEdition Desktop
param([Parameter(Mandatory)][ValidateSet('healthy', 'expired', 'recover')][string]$Case)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$null = [Reflection.Assembly]::LoadFrom((Join-Path $PSScriptRoot 'relay-test.dll'))
$root = Join-Path $PSScriptRoot ('relay job ' + $Case)
$null = [IO.Directory]::CreateDirectory((Join-Path $root 'prestart-temp'))
$entry = Join-Path $root 'prestart_deployment_relay.ps1'
$interpreter = Join-Path $PSHOME 'powershell.exe'
$run = [guid]'3201d337-8836-4bf1-af6d-4a198ef5e44b'
$worker = @'
param([guid]$RunId, [long]$WorkDeadlineTick, [long]$RetirementDeadlineTick)
$ErrorActionPreference = 'Stop'
$now = [Diagnostics.Stopwatch]::GetTimestamp()
if ($now -ge $WorkDeadlineTick -or $RetirementDeadlineTick -le $WorkDeadlineTick) { exit 11 }
[Console]::Out.WriteLine('relay ' + $RunId.ToString('D') + ' ' + $WorkDeadlineTick + ' ' + $RetirementDeadlineTick)
if ([Console]::In.Read() -ne 82 -or [Console]::In.Read() -ne -1) { exit 12 }
exit 0
'@
[IO.File]::WriteAllText($entry, $worker, [Text.Encoding]::ASCII)
$clock = [Diagnostics.Stopwatch]::StartNew()
$latestStart = [Diagnostics.Stopwatch]::GetTimestamp()
if ($Case -ceq 'expired') { [Threading.Thread]::Sleep(80) }
if ($Case -ceq 'healthy') { [Threading.Thread]::Sleep(200) }
$work = if ($Case -ceq 'expired') { 40 } elseif ($Case -ceq 'recover') { 3000 } else { 8000 }
$retirement = $work + 4000
$report = [NevroTick.PrestartObserverChild]::RunDeploymentRelayJob(
    $entry, (Get-Item -LiteralPath $entry).Length, (Get-FileHash -LiteralPath $entry).Hash.ToLowerInvariant(),
    (Get-Item -LiteralPath $interpreter).Length, (Get-FileHash -LiteralPath $interpreter).Hash.ToLowerInvariant(),
    $run, $clock, $work, $retirement)
if ($Case -ceq 'healthy') {
    if (!$report.Pid -or $report.ExitCode -ne 0 -or !$report.JobEmpty -or $report.Recovered -or
        $report.VerifiedProcessLimit -ne 8 -or $null -ne $report.Failure -or $null -ne $report.RecoveryFailure) {
        throw ('healthy relay refused: ' + $report.Failure + ' ' + $report.RecoveryFailure)
    }
} elseif ($Case -ceq 'expired') {
    if ($report.Pid -or $report.JobEmpty -or $report.Recovered -or
        $report.Failure.Message -cne 'prestart-deployment-job:work-deadline' -or $null -ne $report.RecoveryFailure) {
        throw 'expired relay created or failed at the wrong boundary'
    }
} elseif (!$report.Pid -or !$report.JobEmpty -or !$report.Recovered -or $report.ExitCode -ne 1223 -or
    $report.VerifiedProcessLimit -ne 8 -or !$report.RecoveryStartTick -or !$report.JobEmptyTick -or
    $report.TerminationAcceptedTick -lt $report.RecoveryStartTick -or
    $report.JobEmptyTick -lt $report.TerminationAcceptedTick -or
    $report.JobEmptyTick -ge [Diagnostics.Stopwatch]::Frequency * 7 -or
    $report.Failure.Message -cne 'prestart-deployment-job:exit-deadline' -or
    $null -ne $report.RecoveryFailure) {
    throw ('blocked relay did not retire: ' + $report.Failure + ' ' + $report.RecoveryFailure)
}
[IO.Directory]::Delete($root, $true)
if ($Case -ceq 'healthy') {
    [Console]::Error.WriteLine('relay-control healthy ' + ($latestStart + [Diagnostics.Stopwatch]::Frequency * 8))
} else {
    [Console]::Error.WriteLine('relay-control ' + $Case)
}
