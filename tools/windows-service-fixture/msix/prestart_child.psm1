# SPDX-License-Identifier: GPL-2.0-only
Set-StrictMode -Version Latest

# The launch owner supplies the original, exclusively held Process without
# exit callbacks or other readers. The clock includes launch time and must
# run without reset through collection. On failure the owner keeps containment,
# settles outstanding reads and independently confirms retirement.
function Receive-PrestartChild {
    param(
        [Parameter(Mandatory = $true)][Diagnostics.Process]$Process,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 2147483647)][int]$TimeoutMilliseconds
    )

    if (-not $Clock.IsRunning) {
        throw [ArgumentException]::new('prestart-child:clock-not-running')
    }
    if ($Clock.ElapsedMilliseconds -ge $TimeoutMilliseconds) {
        throw [TimeoutException]::new('prestart-child:collection-deadline')
    }
    $limit = 8192
    $processId = $Process.Id
    $stdout = [pscustomobject]@{ Name = 'stdout'; Stream = $Process.StandardOutput.BaseStream;
        Bytes = [byte[]]::new($limit); Used = 0; Eof = $false; Pending = $null }
    $stderr = [pscustomobject]@{ Name = 'stderr'; Stream = $Process.StandardError.BaseStream;
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
                if ($channel.Used -eq $limit) {
                    # Probe EOF in place. A returned byte refuses the entire capture.
                    $channel.Pending = $channel.Stream.ReadAsync($channel.Bytes, 0, 1)
                } else {
                    $channel.Pending = $channel.Stream.ReadAsync(
                        $channel.Bytes, $channel.Used, $limit - $channel.Used)
                }
            }
            if ($stdout.Eof -and $stderr.Eof -and $Process.WaitForExit(0)) {
                foreach ($channel in $channels) {
                    if ($channel.Used -eq $limit) { continue }
                    $bytes = [byte[]]::new($channel.Used)
                    [Array]::Copy($channel.Bytes, $bytes, $channel.Used)
                    $channel.Bytes = $bytes
                }
                $exitCode = $Process.ExitCode
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

Export-ModuleMember -Function Receive-PrestartChild
