# SPDX-License-Identifier: GPL-2.0-only
# Exercise the actual S0.6 collector with fixed inert Windows PowerShell children.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (![Environment]::Is64BitProcess -or $PSVersionTable.PSEdition -cne 'Desktop') {
    throw 'These controls require a 64-bit Windows PowerShell parent'
}
$systemPowerShell = Join-Path ([Environment]::SystemDirectory) 'WindowsPowerShell\v1.0\powershell.exe'
$module = Import-Module (Join-Path $PSScriptRoot 'prestart_child.psm1') -Force -PassThru

function Assert-Bytes {
    param([object]$Actual, [byte[]]$Expected, [string]$Label)

    if ($Actual -isnot [byte[]] -or $Actual.Length -ne $Expected.Length) {
        throw "$Label did not return the expected raw byte-array extent"
    }
    for ($index = 0; $index -lt $Expected.Length; $index++) {
        if ($Actual[$index] -ne $Expected[$index]) {
            throw "$Label differs at byte $index"
        }
    }
}

function Close-InertChild {
    param([Diagnostics.Process]$Process, [Exception]$Refusal)

    if (!$Process.HasExited) {
        $Process.Kill()
    }
    if (!$Process.WaitForExit(5000)) {
        throw 'Test-owned child remained live after cleanup'
    }
    try {
        if ($null -ne $Refusal -and $Refusal.Data.Contains('PrestartChildPendingReads')) {
            $pending = @($Refusal.Data['PrestartChildPendingReads'])
            if ($pending.Count -gt 2) {
                throw 'Collector retained more than one pending read per stream'
            }
            foreach ($read in $pending) {
                if ($read -isnot [Threading.Tasks.Task[int]]) {
                    throw 'Collector did not retain a raw read task'
                }
            }
            if ($pending.Count -gt 0 -and ![Threading.Tasks.Task]::WaitAll([Threading.Tasks.Task[]]$pending, 5000)) {
                throw 'Pending reads did not settle after owned child retirement'
            }
        }
    } finally {
        $Process.StandardInput.Dispose()
        $Process.StandardOutput.Dispose()
        $Process.StandardError.Dispose()
        $Process.Dispose()
    }
}

function Invoke-InertControl {
    param(
        [string]$Label,
        [string]$Program,
        [int]$ExitCode = 0,
        [byte[]]$Stdout = [byte[]]::new(0),
        [byte[]]$Stderr = [byte[]]::new(0),
        [type]$ExpectedRefusal,
        [string]$ExpectedMessage,
        [int]$TimeoutMilliseconds = 12000,
        [Threading.EventWaitHandle]$Ready,
        [switch]$AlreadyExpired
    )

    $start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $systemPowerShell
    $start.Arguments = '-NoLogo -NoProfile -NonInteractive -EncodedCommand ' +
        [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($Program))
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.RedirectStandardInput = $true
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    $clock = [Diagnostics.Stopwatch]::StartNew()
    $child = [Diagnostics.Process]::Start($start)
    $refusal = $null
    $first = $null
    try {
        $pidBefore = $child.Id
        if ($null -ne $Ready -and !$Ready.WaitOne(15000)) {
            throw "$Label child failed to acknowledge its flushed prefix"
        }
        $cutoff = $TimeoutMilliseconds
        if ($null -ne $Ready) {
            $cutoff = [int]$clock.ElapsedMilliseconds + $TimeoutMilliseconds
            if ($AlreadyExpired) { $cutoff = [int]$clock.ElapsedMilliseconds }
        }
        $records = [Collections.Generic.List[object]]::new()
        try {
            Receive-PrestartChild -Process $child -Clock $clock -TimeoutMilliseconds $cutoff |
                ForEach-Object { $records.Add($_) }
        } catch {
            $refusal = $_.Exception
            if ($null -eq $ExpectedRefusal) {
                throw
            }
            $cause = $refusal
            while ($null -ne $cause.InnerException -and !$ExpectedRefusal.IsInstanceOfType($cause)) {
                $cause = $cause.InnerException
            }
            if (!$ExpectedRefusal.IsInstanceOfType($cause) -or $cause.Message -cne $ExpectedMessage) {
                throw
            }
        }
        if ($null -ne $ExpectedRefusal) {
            if ($null -eq $refusal -or $records.Count -ne 0 -or $child.HasExited) {
                throw "$Label failed to refuse without publication while leaving the held child to its owner"
            }
            if ($null -ne $Ready -and $clock.ElapsedMilliseconds -lt $cutoff) {
                throw "$Label refused before the owned deadline"
            }
            $pendingCount = 0
            if ($refusal.Data.Contains('PrestartChildPendingReads')) {
                $pendingCount = @($refusal.Data['PrestartChildPendingReads']).Count
            }
            if ($AlreadyExpired -and $pendingCount -ne 0) {
                throw "$Label started a read after the original deadline"
            }
            if ($null -ne $Ready -and !$AlreadyExpired -and $pendingCount -ne 2) {
                throw "$Label did not transfer both outstanding pipe reads to the owner"
            }
        } else {
            if ($records.Count -ne 1 -or !$child.HasExited) {
                throw "$Label did not return one completed child"
            }
            $result = $records[0]
            if ($result.ProcessId -isnot [int] -or $result.ProcessId -ne $pidBefore -or
                $result.ExitCode -isnot [int] -or $result.ExitCode -ne $ExitCode -or
                $child.ExitCode -ne $ExitCode) {
                throw "$Label did not preserve the owned process ID and exit code"
            }
            Assert-Bytes $result.Stdout $Stdout "$Label stdout"
            Assert-Bytes $result.Stderr $Stderr "$Label stderr"
        }
    } catch {
        $first = $_
        throw
    } finally {
        try {
            Close-InertChild $child $refusal
        } catch {
            $primary = $refusal
            if ($null -ne $first) {
                $primary = $first.Exception
            }
            if ($null -ne $primary) {
                throw [AggregateException]::new("$Label and its cleanup failed", [Exception[]]@($primary, $_.Exception))
            }
            throw
        }
    }
    "s06-prestart-child $Label passed"
}

try {
    Invoke-InertControl -Label 'empty' -Program 'exit 0'
    Invoke-InertControl -Label 'raw-nonzero' -ExitCode 23 -Stdout @(0, 255, 13, 10, 128) -Stderr @(254, 0, 10) -Program @'
$output = [Console]::OpenStandardOutput()
$errorOutput = [Console]::OpenStandardError()
$output.Write([byte[]]@(0, 255, 13, 10, 128), 0, 5)
$errorOutput.Write([byte[]]@(254, 0, 10), 0, 3)
exit 23
'@

    $capStdout = [byte[]]::new(8192)
    $capStderr = [byte[]]::new(8192)
    $capStdout[0] = 41
    $capStdout[8191] = 251
    $capStderr[0] = 53
    $capStderr[8191] = 253
    Invoke-InertControl -Label 'both-at-cap' -Stdout $capStdout -Stderr $capStderr -Program @'
$bytes = [byte[]]::new(8192)
$bytes[0] = 41
$bytes[8191] = 251
[Console]::OpenStandardOutput().Write($bytes, 0, $bytes.Length)
$bytes[0] = 53
$bytes[8191] = 253
[Console]::OpenStandardError().Write($bytes, 0, $bytes.Length)
exit 0
'@

    Invoke-InertControl -Label 'stdout-over-cap' -ExpectedRefusal ([IO.InvalidDataException]) `
        -ExpectedMessage 'prestart-child:stdout-limit' -Program @'
[Console]::OpenStandardOutput().Write([byte[]]::new(8193), 0, 8193)
$inputStream = [Console]::OpenStandardInput()
while ($inputStream.ReadByte() -ge 0) {}
exit 1
'@
    Invoke-InertControl -Label 'stderr-over-cap' -ExpectedRefusal ([IO.InvalidDataException]) `
        -ExpectedMessage 'prestart-child:stderr-limit' -Program @'
[Console]::OpenStandardError().Write([byte[]]::new(8193), 0, 8193)
$inputStream = [Console]::OpenStandardInput()
while ($inputStream.ReadByte() -ge 0) {}
exit 1
'@

    $readyName = 'Local\NevroTick-S06-Child-' + [Guid]::NewGuid().ToString('N')
    $created = $false
    $ready = [Threading.EventWaitHandle]::new($false, [Threading.EventResetMode]::ManualReset, $readyName, [ref]$created)
    try {
        if (!$created) {
            throw 'Timeout readiness event was already in use'
        }
        # The only interpolated value is this test's fixed prefix plus a fresh GUID.
        $program = '$ready = [Threading.EventWaitHandle]::OpenExisting(''' + $readyName + ''')' + [Environment]::NewLine + @'
$output = [Console]::OpenStandardOutput()
$output.Write([byte[]]@(3, 0, 255), 0, 3)
$output.Flush()
$null = $ready.Set()
$ready.Dispose()
$inputStream = [Console]::OpenStandardInput()
while ($inputStream.ReadByte() -ge 0) {}
exit 1
'@
        $timeout = @{
            Program = $program; ExpectedRefusal = [TimeoutException]; TimeoutMilliseconds = 250
            ExpectedMessage = 'prestart-child:collection-deadline'; Ready = $ready
        }
        Invoke-InertControl @timeout -Label 'timeout-after-prefix'
        if (!$ready.Reset()) { throw 'Could not reset the owned readiness event' }
        Invoke-InertControl @timeout -Label 'expired-before-collection' -AlreadyExpired
    } finally {
        $ready.Dispose()
    }
} finally {
    Remove-Module $module
}

's06-prestart-child controls=7'
