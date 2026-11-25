# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
#Requires -PSEdition Desktop
param([Parameter(Mandatory)][guid]$RunId, [Parameter(Mandatory)][string]$ExpectedSha256,
    [Parameter(Mandatory)][int]$ExpectedBytes, [Parameter(Mandatory)][string]$WatchdogSha256)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSScriptRoot 'prestart_deployment_activation.psm1')
$root = Join-Path $PSScriptRoot ([guid]::NewGuid().ToString('N'))
$null = [IO.Directory]::CreateDirectory($root)
$source = Join-Path $PSScriptRoot 'prestart_deployment_watchdog.exe'
$interpreter = Join-Path $PSHOME 'powershell.exe'
$guardian = @'
param([string]$EntryPath, [long]$EntryBytes, [string]$EntrySha256, [long]$PowerShellBytes,
    [string]$PowerShellSha256, [guid]$RunId, [int]$WorkMilliseconds, [int]$RetirementMilliseconds)
$ErrorActionPreference = 'Stop'
if ([IO.Path]::GetFileName($EntryPath) -cne 'prestart_deployment_owner.ps1' -or
    (Get-Item -LiteralPath $EntryPath).Length -ne $EntryBytes -or
    (Get-FileHash -LiteralPath $EntryPath).Hash.ToLowerInvariant() -cne $EntrySha256) { exit 4 }
if ([Console]::In.Read() -ne 82 -or [Console]::In.Read() -ne -1) { exit 5 }
[Console]::Out.WriteLine($RunId.ToString('D') + ' ' + $EntryBytes + ' ' + $EntrySha256)
exit 0
'@

$recordPath = Join-Path $root 'records'
$null = [IO.Directory]::CreateDirectory($recordPath)
[IO.File]::Copy((Join-Path $PSScriptRoot 'deployment-activation.bin'), (Join-Path $recordPath 'deployment-activation.bin'))
[IO.File]::WriteAllBytes((Join-Path $recordPath 'deployment-activation.bin.commit'), [byte[]]::new(0))
$state = @{ Attempted = $false }
$safe = $false
try {
    foreach ($healthy in @($false, $true)) {
        $stage = Join-Path $root ('local stage ' + $healthy)
        $null = [IO.Directory]::CreateDirectory((Join-Path $stage 'prestart-temp'))
        $local = Join-Path $stage 'prestart_deployment_watchdog.exe'
        [IO.File]::Copy($source, $local)
        $guardianPath = Join-Path $stage 'prestart_deployment_guardian.ps1'
        [IO.File]::WriteAllText($guardianPath, $guardian, [Text.Encoding]::ASCII)
        $state = @{}
        $process = [Diagnostics.Process]::new()
        $safe = $false
        $launch = @{
            RecordsIn = $recordPath; Stage = $stage; RunId = $RunId
            Clock = [Diagnostics.Stopwatch]::StartNew(); WorkMilliseconds = 12000; RetirementMilliseconds = 17000
            WatchdogBytes = (Get-Item -LiteralPath $local).Length
            WatchdogSha256 = $(if ($healthy) { $WatchdogSha256 } else { '0' * 64 })
            GuardianBytes = (Get-Item -LiteralPath $guardianPath).Length
            GuardianSha256 = (Get-FileHash -LiteralPath $guardianPath).Hash.ToLowerInvariant()
            PowerShellBytes = (Get-Item -LiteralPath $interpreter).Length
            PowerShellSha256 = (Get-FileHash -LiteralPath $interpreter).Hash.ToLowerInvariant()
            GuardianWork = 7000; GuardianRetirement = 10000; Process = $process; Custody = $state
        }
        try {
            if (!$healthy) {
                try { Start-PrestartDeploymentActivation @launch; throw 'wrong watchdog pin reached launch' }
                catch {
                    if ($_.Exception.Message -cne 'prestart-activation:watchdog-digest' -or $state.Attempted) { throw }
                    $safe = $true
                }
                continue
            }
            Start-PrestartDeploymentActivation @launch
            if (!$state.Attempted -or !$state.Started) { throw 'missing original process custody' }
            $wire = $process.StandardOutput.ReadToEndAsync()
            $diagnostic = $process.StandardError.ReadToEndAsync()
            $process.StandardInput.BaseStream.WriteByte(82)
            $process.StandardInput.Close()
            if (!$process.WaitForExit(18000)) { throw 'watchdog retirement unconfirmed' }
            if (!$wire.Wait(1000) -or !$diagnostic.Wait(1000) -or $process.ExitCode -ne 0 -or
                $diagnostic.Result -cnotmatch 'guardian_exit=0 job_empty=true recovered=false') {
                throw ('watchdog observation refused: ' + $diagnostic.Result)
            }
            $safe = $true
            if ($wire.Result.Trim() -cne "$($RunId.ToString('D')) $ExpectedBytes $ExpectedSha256") {
                throw 'watchdog staged-entry observation differs'
            }
        } finally {
            if (!$state.ContainsKey('Attempted') -or !$state.Attempted -or $safe) {
                if ($state.ContainsKey('Entry')) { $state.Entry.Lease.Dispose() }
                if ($state.ContainsKey('Watchdog')) { $state.Watchdog.Dispose() }
                $process.Dispose()
            }
        }
    }
    [Console]::WriteLine('activation-watchdog-controls=2 failed=0')
} finally {
    if (!$state.ContainsKey('Attempted') -or !$state.Attempted -or $safe) { [IO.Directory]::Delete($root, $true) }
}
