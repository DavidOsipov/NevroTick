# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
Set-StrictMode -Version Latest

Import-Module (Join-Path $PSScriptRoot 'prestart_deployment_activation.psm1') -ErrorAction Stop

function Write-S06RelayBytes([string]$Folder, [string]$Leaf, [byte[]]$Bytes) {
    [IO.FileStream]$file = [IO.File]::Open((Join-Path $Folder $Leaf), [IO.FileMode]::CreateNew,
        [IO.FileAccess]::Write, [IO.FileShare]::None)
    try { $file.Write($Bytes, 0, $Bytes.Length); $file.Flush($true) }
    finally { $file.Dispose() }
}

function Publish-S06RelayFile([string]$Folder, [string]$Leaf, [byte[]]$Bytes,
        [Diagnostics.Stopwatch]$Clock, [int]$Cutoff) {
    if ($Clock.ElapsedMilliseconds -ge $Cutoff) { throw 'prestart-mapped-relay:publication-deadline' }
    Write-S06RelayBytes $Folder $Leaf $Bytes
    if ($Clock.ElapsedMilliseconds -ge $Cutoff) { throw 'prestart-mapped-relay:publication-deadline' }
    [IO.FileStream]$file = [IO.File]::Open((Join-Path $Folder ($Leaf + '.commit')), 'CreateNew',
        'Write', 'ReadWrite')
    try { $file.Flush($true) }
    finally { $file.Dispose() }
}

function Get-S06RelayDigest([byte[]]$Bytes) {
    [Security.Cryptography.SHA256]$hash = [Security.Cryptography.SHA256]::Create()
    try { return [BitConverter]::ToString($hash.ComputeHash($Bytes)).Replace('-', '').ToLowerInvariant() }
    finally { $hash.Dispose() }
}

function New-S06RelayStream([int]$Limit) {
    return @{ Stream = $null; Buffer = [byte[]]::new($Limit + 1); Used = 0; Pending = $null;
        Eof = $false }
}

function Start-S06RelayRead([hashtable]$Channel) {
    [int]$count = [Math]::Min(4096, $Channel.Buffer.Length - $Channel.Used)
    $Channel.Pending = $Channel.Stream.ReadAsync($Channel.Buffer, $Channel.Used, $count)
}

function Update-S06RelayRead([hashtable]$Channel) {
    while ($null -ne $Channel.Pending -and $Channel.Pending.IsCompleted) {
        [object]$pending = $Channel.Pending
        $Channel.Pending = $null
        [int]$count = $pending.GetAwaiter().GetResult()
        if ($count -eq 0) {
            $Channel.Eof = $true
            return
        }
        $Channel.Used += $count
        if ($Channel.Used -ge $Channel.Buffer.Length) { throw 'fixture-role-stream-limit' }
        Start-S06RelayRead $Channel
    }
}

# The enclosing native transaction owns actor admission and both mappings.
function Read-S06RelayCommand([string]$Directory, [int]$Sequence) {
    [string]$file = Join-Path $Directory ("request-$Sequence.bin")
    [string]$marker = $file + '.commit'
    if (![IO.File]::Exists($marker)) { return -1 }
    foreach ($leaf in @($marker, $file)) {
        [IO.FileAttributes]$attributes = [IO.File]::GetAttributes($leaf)
        if (($attributes -band ([IO.FileAttributes]::ReparsePoint -bor [IO.FileAttributes]::Directory)) -ne 0) {
            throw 'prestart-mapped-relay-input-kind'
        }
    }
    [IO.FileStream]$commit = [IO.File]::Open($marker, 'Open', 'Read', 'ReadWrite')
    try { if ($commit.Length -ne 0 -or $commit.ReadByte() -ne -1) { throw 'prestart-mapped-relay-input-commit' } }
    finally { $commit.Dispose() }
    [IO.FileStream]$request = [IO.File]::Open($file, 'Open', 'Read', 'Read')
    try {
        if ($request.Length -ne 1) { throw 'prestart-mapped-relay-input-extent' }
        [int]$value = $request.ReadByte()
        if ($request.ReadByte() -ne -1) { throw 'prestart-mapped-relay-input-extent' }
        return $value
    } finally { $request.Dispose() }
}

function Get-S06RelayBytes([hashtable]$Stream) {
    [byte[]]$result = [byte[]]::new($Stream.Used)
    [Array]::Copy($Stream.Buffer, $result, $result.Length)
    return ,$result
}

function Test-S06RelayRetirement([byte[]]$Raw, [string]$Run, [int]$WatchdogPid, [int]$Exit) {
    try {
        [string]$text = [Text.UTF8Encoding]::new($false, $true).GetString($Raw)
        if (!$text.EndsWith("`n", [StringComparison]::Ordinal)) { return $false }
        [string[]]$lines = $text.Substring(0, $text.Length - 1).Split([char]10)
        if ($lines.Length -lt 2 -or $lines.Length -gt 3) { return $false }
        [string]$outer = $lines[-1].TrimEnd([char]13)
        [string]$inner = $lines[-2].TrimEnd([char]13)
        if (!$inner.StartsWith('nevrotick-deployment-job ', [StringComparison]::Ordinal)) { return $false }
        [object]$row = ConvertFrom-Json -InputObject $inner.Substring('nevrotick-deployment-job '.Length)
        if ($row.run -cne $Run -or $row.job_empty -isnot [bool] -or !$row.job_empty -or
            $row.verified_process_limit -ne 2 -or $null -eq $row.owner_exit -or !$row.owner_pid -or
            $null -ne $row.recovery_failure) { return $false }
        [string[]]$pieces = $outer.Split(' ')
        if ($pieces[0] -cne 'nevrotick-deployment-watchdog') { return $false }
        [hashtable]$words = @{}
        foreach ($piece in $pieces[1..($pieces.Length - 1)]) {
            [string[]]$pair = $piece.Split('=', 2)
            if ($pair.Length -ne 2 -or $words.ContainsKey($pair[0])) { return $false }
            $words.Add($pair[0], $pair[1])
        }
        if ($words.run -cne $Run -or $words.job_empty -cne 'true' -or $words.verified_process_limit -cne '3' -or
            $words.guardian_pid -cnotmatch '^[1-9][0-9]{0,9}$' -or $words.guardian_exit -cnotmatch '^-?[0-9]{1,10}$' -or
            $words.recovery_failure -cne '-' -or
            @(@($WatchdogPid, [long]$words.guardian_pid, [long]$row.owner_pid) | Select-Object -Unique).Count -ne 3) {
            return $false
        }
        if ($Exit -eq 0) {
            return ($lines.Length -eq 2 -and $row.owner_exit -eq 0 -and !$row.recovered -and
                $null -eq $row.failure -and $words.guardian_exit -ceq '0' -and
                $words.recovered -ceq 'false' -and $words.failure -ceq '-')
        }
        return $true
    } catch { return $false }
}

function Invoke-PrestartMappedDeployment {
    <#
    .SYNOPSIS
    Run the full deployment command pipe inside its original guest relay job.
    .DESCRIPTION
    Only a pinned relay entry supplies the mappings and launch profile. The
    enclosing job owns forced recovery, even if file I/O or Process.Start stalls.
    It independently observes relay and watchdog retirement before removing either
    mapping or the staged entry. This worker forwards no access-role traffic.
    #>
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory)][string]$RecordsIn,
        [Parameter(Mandatory)][string]$RecordsOut,
        [Parameter(Mandatory)][guid]$RunId,
        [Parameter(Mandatory)][long]$WorkDeadlineTick,
        [Parameter(Mandatory)][long]$RetirementDeadlineTick,
        [Parameter(Mandatory)][hashtable]$Launch,
        [switch]$ReportNativeObservation
    )
    $ErrorActionPreference = 'Stop'
    $required = @('Stage', 'WatchdogBytes', 'WatchdogSha256', 'GuardianBytes', 'GuardianSha256',
        'PowerShellBytes', 'PowerShellSha256', 'GuardianWork', 'GuardianRetirement')
    if ($Launch.Count -ne $required.Length -or $RunId -eq [guid]::Empty -or ![Diagnostics.Stopwatch]::IsHighResolution) {
        throw 'prestart-mapped-relay:profile'
    }
    foreach ($key in $required) {
        if (!$Launch.ContainsKey($key)) { throw 'prestart-mapped-relay:profile' }
    }
    foreach ($directory in @($RecordsIn, $RecordsOut)) {
        if ($directory.Length -gt 512 -or $directory -cnotmatch '\A[A-Za-z]:\\[^\x00-\x1f"*/?<>|:]+\z' -or
            [IO.Path]::GetFullPath($directory).TrimEnd('\') -cne $directory) {
            throw 'prestart-mapped-relay:mapping'
        }
        $kind = [IO.File]::GetAttributes($directory)
        if (($kind -band [IO.FileAttributes]::Directory) -eq 0 -or
            ($kind -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw 'prestart-mapped-relay:mapping'
        }
    }
    if ($RecordsIn -ieq $RecordsOut) { throw 'prestart-mapped-relay:mapping' }
    $entries = [IO.Directory]::EnumerateFileSystemEntries($RecordsOut).GetEnumerator()
    try {
        if ($entries.MoveNext()) { throw 'prestart-mapped-relay:output-occupied' }
    } finally { $entries.Dispose() }

    $clock = [Diagnostics.Stopwatch]::StartNew()
    $now = [Diagnostics.Stopwatch]::GetTimestamp()
    $frequency = [Diagnostics.Stopwatch]::Frequency
    if ($WorkDeadlineTick -le $now -or $RetirementDeadlineTick -le $WorkDeadlineTick) {
        throw 'prestart-mapped-relay:deadline'
    }
    $work = [Math]::Floor(($WorkDeadlineTick - $now) * 1000.0 / $frequency)
    $retirement = [Math]::Floor(($RetirementDeadlineTick - $now) * 1000.0 / $frequency)
    if ($work -lt 1 -or $retirement -gt 120000 -or $retirement -le $work) {
        throw 'prestart-mapped-relay:deadline'
    }

    $marker = Join-Path $RecordsIn 'deployment-activation.bin.commit'
    while (![IO.File]::Exists($marker)) {
        if ($clock.ElapsedMilliseconds -ge $work) { throw 'prestart-mapped-relay:activation-expired' }
        Start-Sleep -Milliseconds 10
    }
    $process = [Diagnostics.Process]::new()
    $custody = @{}
    $wire = New-S06RelayStream (9 * 24004)
    $diag = New-S06RelayStream 32768
    $sent = ''
    $roster = 'SCICOAUFR'
    $offset = 0
    $emitted = 0
    $settled = $false
    try {
        Start-PrestartDeploymentActivation @Launch -RecordsIn $RecordsIn -RunId $RunId -Clock $clock `
            -WorkMilliseconds ([int]$work) -RetirementMilliseconds ([int]$retirement) -Process $process -Custody $custody
        $wire.Stream = $process.StandardOutput.BaseStream
        $diag.Stream = $process.StandardError.BaseStream
        Start-S06RelayRead $wire
        Start-S06RelayRead $diag
        while ($clock.ElapsedMilliseconds -lt $retirement) {
            Update-S06RelayRead $wire
            Update-S06RelayRead $diag
            while ($wire.Used - $offset -ge 4) {
                [uint32]$length = [BitConverter]::ToUInt32($wire.Buffer, $offset)
                if ($length -lt 1 -or $length -gt 24000 -or $emitted -ge $sent.Length) {
                    throw 'prestart-mapped-relay:frame'
                }
                if ($wire.Used - $offset -lt $length + 4) { break }
                $frame = [byte[]]::new($length + 4)
                [Array]::Copy($wire.Buffer, $offset, $frame, 0, $frame.Length)
                $emitted++
                Publish-S06RelayFile $RecordsOut "reply-$emitted.bin" $frame $clock $retirement
                $offset += $frame.Length
            }
            if ($wire.Eof -and $diag.Eof -and $process.WaitForExit(0)) { break }
            if (!$sent.EndsWith('R')) {
                $command = Read-S06RelayCommand $RecordsIn ($sent.Length + 1)
                if ($command -ne -1) {
                    if ($clock.ElapsedMilliseconds -ge $retirement -or $sent.Length -ge $roster.Length -or
                        ($command -ne 82 -and ($command -ne [int][char]$roster[$sent.Length] -or
                            $clock.ElapsedMilliseconds -ge $work -or $sent.Length -ne $emitted))) {
                        throw 'prestart-mapped-relay:command'
                    }
                    $process.StandardInput.BaseStream.WriteByte([byte]$command)
                    $process.StandardInput.BaseStream.Flush()
                    $sent += [char]$command
                    if ($command -eq 82) { $process.StandardInput.BaseStream.Dispose() }
                }
            }
            Start-Sleep -Milliseconds 10
        }
        if (!$wire.Eof -or !$diag.Eof -or !$process.WaitForExit(0) -or $clock.ElapsedMilliseconds -ge $retirement) {
            throw 'prestart-mapped-relay:retirement'
        }

        $stdout = Get-S06RelayBytes $wire
        $stderr = Get-S06RelayBytes $diag
        $settled = Test-S06RelayRetirement $stderr ($RunId.ToString('D')) $process.Id $process.ExitCode
        $clean = $sent.EndsWith('R') -and $emitted -eq $sent.Length -and $wire.Used -eq $offset
        $record = @{
            run = $RunId.ToString('D'); commands = $sent; reply_count = $emitted
            launch_attempted = $true; transport_ok = $clean; retired = $settled
            watchdog_pid = $process.Id; watchdog_exit = $process.ExitCode
            stdout_eof = $true; stderr_eof = $true
            stdout_sha256 = Get-S06RelayDigest $stdout; stderr_sha256 = Get-S06RelayDigest $stderr
        }
        Publish-S06RelayFile $RecordsOut 'watchdog.stdout.bin' $stdout $clock $retirement
        Publish-S06RelayFile $RecordsOut 'watchdog.stderr.bin' $stderr $clock $retirement
        $json = [Text.UTF8Encoding]::new($false).GetBytes((ConvertTo-Json -InputObject $record -Compress) + "`n")
        Publish-S06RelayFile $RecordsOut 'relay.json' $json $clock $retirement
        if ($clock.ElapsedMilliseconds -ge $retirement) { throw 'prestart-mapped-relay:publication-deadline' }
        if (!$settled -or !$clean -or $process.ExitCode -ne 0) { throw 'prestart-mapped-relay:refused' }
        if ($ReportNativeObservation) {
            # The enclosing native job passes this original process observation
            # through its own pipe. Do not reconstruct it from the mapped files.
            $observation = @{ run = $RunId.ToString('D'); pid = $process.Id
                diagnostic = [Convert]::ToBase64String($stderr) }
            [Console]::WriteLine((ConvertTo-Json -InputObject $observation -Compress))
        }
    } finally {
        # Unconfirmed work remains inside the enclosing native job. Do not
        # explicitly dispose its staging leases or call cleanup on its paths.
        if ($settled -or !$custody.ContainsKey('Attempted') -or !$custody.Attempted) {
            if ($custody.ContainsKey('Entry')) { $custody.Entry.Lease.Dispose() }
            if ($custody.ContainsKey('Watchdog')) { $custody.Watchdog.Dispose() }
            $process.Dispose()
        }
    }
}

Export-ModuleMember -Function Invoke-PrestartMappedDeployment
