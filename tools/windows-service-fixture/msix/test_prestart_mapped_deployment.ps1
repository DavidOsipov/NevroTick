# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
param([switch]$Relay, [switch]$NativeOwner, [switch]$AccessJoined, [string]$Root, [long]$WorkTick, [long]$RetireTick,
    [string]$MappedIn = '', [string]$MappedOut = '', [guid]$RunId = '0a4ce8df-cc42-4c8b-b6b1-744450d703e4')

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$run = $RunId
$roster = 'SCICOAUFR'
$mapped = $MappedIn.Length -gt 0 -and $MappedOut.Length -gt 0

if ($NativeOwner) {
    $parts = @('prestart_actor_token.cs', 'prestart_archive_facts.cs', 'prestart_observer_child.cs',
        'prestart_scm_window.cs', 'prestart_deployment_job.cs')
    $sources = @($parts | ForEach-Object { Join-Path $PSScriptRoot $_ })
    Add-Type -LiteralPath $sources -ReferencedAssemblies @('System.dll','System.Core.dll','System.Security.dll') -WarningAction Stop
    $interpreter = Join-Path $PSHOME 'powershell.exe'
    $entry = Join-Path $Root 'job\prestart_deployment_relay.ps1'
    $clock = [Diagnostics.Stopwatch]::StartNew()
    $now = [Diagnostics.Stopwatch]::GetTimestamp()
    $frequency = [Diagnostics.Stopwatch]::Frequency
    $report = [NevroTick.PrestartObserverChild]::RunDeploymentRelayJob(
        $entry, (Get-Item -LiteralPath $entry).Length, (Get-FileHash -LiteralPath $entry).Hash.ToLowerInvariant(),
        (Get-Item -LiteralPath $interpreter).Length, (Get-FileHash -LiteralPath $interpreter).Hash.ToLowerInvariant(),
        $run, $clock, [int][Math]::Floor(($WorkTick - $now) * 1000.0 / $frequency),
        [int][Math]::Floor(($RetireTick - $now) * 1000.0 / $frequency))
    if (!$report.Pid -or !$report.JobEmpty -or $report.Recovered -or $report.ExitCode -ne 0 -or
        $report.VerifiedProcessLimit -ne 8 -or $null -ne $report.Failure -or $null -ne $report.RecoveryFailure) {
        throw ('enclosing relay job not settled: ' + $report.Failure + ' ' + $report.RecoveryFailure)
    }
    [Console]::WriteLine('outer-relay-job-empty=true')
    exit 0
}

if ($Relay) {
    Import-Module (Join-Path $PSScriptRoot 'prestart_deployment_relay.psm1')
    $stage = Join-Path $Root 'stage'
    $exe = Join-Path $stage 'prestart_deployment_watchdog.exe'
    $guardian = Join-Path $stage 'prestart_deployment_guardian.ps1'
    $interpreter = Join-Path $PSHOME 'powershell.exe'
    $launch = @{
        Stage = $stage
        WatchdogBytes = (Get-Item -LiteralPath $exe).Length
        WatchdogSha256 = (Get-FileHash -LiteralPath $exe).Hash.ToLowerInvariant()
        GuardianBytes = (Get-Item -LiteralPath $guardian).Length
        GuardianSha256 = (Get-FileHash -LiteralPath $guardian).Hash.ToLowerInvariant()
        PowerShellBytes = (Get-Item -LiteralPath $interpreter).Length
        PowerShellSha256 = (Get-FileHash -LiteralPath $interpreter).Hash.ToLowerInvariant()
        GuardianWork = $(if ($AccessJoined) { 90000 } elseif ($mapped) { 64000 } else { 16500 })
        GuardianRetirement = $(if ($AccessJoined) { 100000 } elseif ($mapped) { 72000 } else { 19000 })
    }
    $incoming = if ($mapped) { $MappedIn } else { Join-Path $Root 'in' }
    $outgoing = if ($mapped) { $MappedOut } else { Join-Path $Root 'out' }
    Invoke-PrestartMappedDeployment -RecordsIn $incoming -RecordsOut $outgoing `
        -RunId $run -WorkDeadlineTick $WorkTick -RetirementDeadlineTick $RetireTick -Launch $launch `
        -ReportNativeObservation:$mapped
    exit 0
}

$root = Join-Path $PSScriptRoot ('full-' + [guid]::NewGuid().ToString('N'))
$stage = Join-Path $root 'stage'
$inbox = Join-Path $root 'in'
$outbox = Join-Path $root 'out'
foreach ($directory in @($inbox, $outbox, (Join-Path $stage 'prestart-temp'), (Join-Path $root 'job\prestart-temp'))) {
    $null = [IO.Directory]::CreateDirectory($directory)
}
foreach ($name in @('prestart_deployment_watchdog.exe', 'prestart_deployment_guardian.ps1',
        'prestart_actor_token.cs', 'prestart_archive_facts.cs', 'prestart_observer_child.cs',
        'prestart_scm_window.cs', 'prestart_deployment_job.cs')) {
    [IO.File]::Copy((Join-Path $PSScriptRoot $name), (Join-Path $stage $name))
}
$worker = @'
param([Parameter(Mandatory)][guid]$RunId)
$source = [Console]::OpenStandardInput()
$sink = [Console]::OpenStandardOutput()
foreach ($character in 'SCICOAUFR'.ToCharArray()) {
    if ($source.ReadByte() -ne [int]$character) { exit 11 }
    $body = [Text.Encoding]::ASCII.GetBytes('fixture-' + $character)
    $prefix = [BitConverter]::GetBytes([uint32]$body.Length)
    $sink.Write($prefix, 0, $prefix.Length)
    $sink.Write($body, 0, $body.Length)
    $sink.Flush()
}
if ($source.ReadByte() -ne -1) { exit 12 }
'@
$bytes = [Text.Encoding]::ASCII.GetBytes($worker)
$algorithm = [Security.Cryptography.SHA256]::Create()
try { $pin = [BitConverter]::ToString($algorithm.ComputeHash($bytes)).Replace('-', '').ToLowerInvariant() }
finally { $algorithm.Dispose() }
$header = [Text.Encoding]::ASCII.GetBytes("nevrotick-prestart-activation-v1 $run $($bytes.Length) $pin`n")
$record = Join-Path $inbox 'deployment-activation.bin'
[IO.File]::WriteAllBytes($record, [byte[]]($header + $bytes))
[IO.File]::WriteAllBytes("$record.commit", [byte[]]::new(0))

$entryScript = @'
param([guid]$RunId, [long]$WorkDeadlineTick, [long]$RetirementDeadlineTick)
if ($RunId -ne [guid]'__RUN__') { exit 13 }
& '__TEST__' -Relay -Root '__ROOT__' -WorkTick $WorkDeadlineTick -RetireTick $RetirementDeadlineTick `
    -RunId $RunId -MappedIn '__IN__' -MappedOut '__OUT__' __ACCESS__
'@
$entryScript = $entryScript.Replace('__TEST__', $PSCommandPath.Replace("'", "''")).Replace('__ROOT__', $root.Replace("'", "''"))
$entryScript = $entryScript.Replace('__RUN__', $run.ToString('D')).Replace('__IN__', $MappedIn.Replace("'", "''")).Replace('__OUT__', $MappedOut.Replace("'", "''"))
$entryScript = $entryScript.Replace('__ACCESS__', $(if ($AccessJoined) { '-AccessJoined' } else { '' }))
[IO.File]::WriteAllText((Join-Path $root 'job\prestart_deployment_relay.ps1'), $entryScript, [Text.Encoding]::ASCII)
$now = [Diagnostics.Stopwatch]::GetTimestamp()
$frequency = [Diagnostics.Stopwatch]::Frequency
$work = $now + $frequency * $(if ($AccessJoined) { 107 } elseif ($mapped) { 78 } else { 22 })
$retire = $work + $frequency * $(if ($AccessJoined) { 11 } elseif ($mapped) { 12 } else { 4 })
$info = [Diagnostics.ProcessStartInfo]::new()
$info.FileName = Join-Path $PSHOME 'powershell.exe'
$info.Arguments = '-NoProfile -NonInteractive -ExecutionPolicy RemoteSigned -File "' + $PSCommandPath +
    '" -NativeOwner -Root "' + $root + '" -WorkTick ' + $work + ' -RetireTick ' + $retire + ' -RunId ' + $run.ToString('D')
$info.UseShellExecute = $false
$info.CreateNoWindow = $true
$info.RedirectStandardInput = $true
$info.RedirectStandardOutput = $true
$info.RedirectStandardError = $true
$process = [Diagnostics.Process]::Start($info)
$safe = $false
try {
    $process.StandardInput.Close()
    $output = $process.StandardOutput.ReadToEndAsync()
    $diagnostic = $process.StandardError.ReadToEndAsync()
    if ($mapped) {
        $wait = if ($AccessJoined) { 120000 } else { 94000 }
        if (!$process.WaitForExit($wait) -or !$output.Wait(500) -or !$diagnostic.Wait(500)) {
            throw 'mapped enclosing owner retirement unconfirmed'
        }
        if ($process.ExitCode -ne 0 -or $diagnostic.Result.Length -ne 0) {
            throw ('mapped enclosing owner refused: ' + $diagnostic.Result)
        }
        $lines = $output.Result.TrimEnd([char[]]@([char]13, [char]10)).Split([char]10)
        if ($lines.Length -ne 2 -or $lines[1] -cne 'outer-relay-job-empty=true') { throw 'mapped job receipt' }
        $observation = $lines[0] | ConvertFrom-Json
        if ($observation.run -cne $run.ToString('D') -or !$observation.pid -or !$observation.diagnostic) {
            throw 'mapped native watchdog receipt'
        }
        $native = Join-Path ([IO.Directory]::GetParent($MappedOut).FullName) 'native-observation.json'
        $payload = [Text.UTF8Encoding]::new($false).GetBytes($lines[0] + "`n")
        if ($payload.Length -gt 50000) { throw 'mapped observation extent' }
        $file = [IO.File]::Open($native, 'CreateNew', 'Write', 'None')
        try { $file.Write($payload, 0, $payload.Length); $file.Flush($true) } finally { $file.Dispose() }
        $marker = [IO.File]::Open("$native.commit", 'CreateNew', 'Write', 'ReadWrite')
        try { $marker.Flush($true) } finally { $marker.Dispose() }
        $safe = $true
        [Console]::WriteLine('mapped-native-job-empty=true')
        exit 0
    }
    $expectedWire = [IO.MemoryStream]::new()
    for ($i = 0; $i -lt $roster.Length; $i++) {
        $index = $i + 1
        $request = Join-Path $inbox "request-$index.bin"
        [IO.File]::WriteAllBytes($request, [byte[]]@([byte]$roster[$i]))
        [IO.File]::WriteAllBytes("$request.commit", [byte[]]::new(0))
        $reply = Join-Path $outbox "reply-$index.bin"
        while (![IO.File]::Exists("$reply.commit")) {
            if ($process.WaitForExit(0)) {
                throw ('mapped relay exited at slot ' + $index + ': ' + $diagnostic.GetAwaiter().GetResult())
            }
            if ([Diagnostics.Stopwatch]::GetTimestamp() -ge $work) { throw 'mapped reply deadline' }
            Start-Sleep -Milliseconds 10
        }
        $raw = [IO.File]::ReadAllBytes($reply)
        $expectedWire.Write($raw, 0, $raw.Length)
        $length = [BitConverter]::ToUInt32($raw, 0)
        if ($length -ne $raw.Length - 4 -or
            [Text.Encoding]::ASCII.GetString($raw, 4, [int]$length) -cne ('fixture-' + $roster[$i])) {
            throw 'mapped reply transcript'
        }
    }
    if (!$process.WaitForExit(4500) -or !$output.Wait(500) -or !$diagnostic.Wait(500) -or
        $process.ExitCode -ne 0 -or $output.Result.Trim() -cne 'outer-relay-job-empty=true') {
        throw 'mapped relay exit or stream settlement'
    }
    $terminal = Get-Content -LiteralPath (Join-Path $outbox 'relay.json') -Raw | ConvertFrom-Json
    $capturedWire = [IO.File]::ReadAllBytes((Join-Path $outbox 'watchdog.stdout.bin'))
    if ([Convert]::ToBase64String($capturedWire) -cne [Convert]::ToBase64String($expectedWire.ToArray()) -or
        $terminal.run -cne $run.ToString('D') -or $terminal.commands -cne $roster -or $terminal.reply_count -ne 9 -or
        !$terminal.retired -or !$terminal.transport_ok -or $terminal.watchdog_exit -ne 0 -or !$terminal.stdout_eof -or
        !$terminal.stderr_eof -or ![IO.File]::Exists((Join-Path $outbox 'relay.json.commit')) -or
        (Get-FileHash -LiteralPath (Join-Path $outbox 'watchdog.stdout.bin')).Hash.ToLowerInvariant() -cne $terminal.stdout_sha256 -or
        (Get-FileHash -LiteralPath (Join-Path $outbox 'watchdog.stderr.bin')).Hash.ToLowerInvariant() -cne $terminal.stderr_sha256) {
        throw 'mapped relay terminal proof'
    }
    $safe = $true
    [Console]::WriteLine('native-mapped-deployment frames=9 settled=true')
} finally {
    if ($safe) {
        $process.Dispose()
        [IO.Directory]::Delete($root, $true)
    }
}
