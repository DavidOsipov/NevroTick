# SPDX-License-Identifier: GPL-2.0-only
Set-StrictMode -Version Latest

function Receive-PrestartDeploymentActivation {
    <#
    .SYNOPSIS
    Admit one committed full-mode entry into protected guest-local staging.
    .DESCRIPTION
    The guest transaction owns both directories, input mapping origin, effective actor
    and the original clock. Call only once for the expected run, after readiness. The
    returned read lease excludes writers and deletion. Keep it through the watchdog's
    use of the pin, then dispose it before retiring the staging directory. On refusal
    the transaction owns any incomplete local leaf and must not start the watchdog.
    #>
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory)][string]$RecordsIn,
        [Parameter(Mandatory)][string]$Stage,
        [Parameter(Mandatory)][guid]$RunId,
        [Parameter(Mandatory)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory)][ValidateRange(1, 120000)][int]$WorkMilliseconds
    )
    $ErrorActionPreference = 'Stop'
    if (!$Clock.IsRunning -or $Clock.ElapsedMilliseconds -ge $WorkMilliseconds -or $RunId -eq [guid]::Empty) {
        throw 'prestart-activation:admission'
    }
    foreach ($directory in @($RecordsIn, $Stage)) {
        if ($directory.Length -gt 512 -or $directory -cnotmatch '\A[A-Za-z]:\\[^\x00-\x1f"*/?<>|:]+\z' -or
            [IO.Path]::GetFullPath($directory).TrimEnd('\') -cne $directory) {
            throw 'prestart-activation:directory-path'
        }
        foreach ($component in $directory.Substring(3).Split('\')) {
            if ($component.EndsWith('.') -or $component.EndsWith(' ') -or $component.Length -eq 0 -or
                $component.Length -gt 255) {
                throw 'prestart-activation:directory-path'
            }
        }
        [IO.FileAttributes]$attributes = [IO.File]::GetAttributes($directory)
        if (($attributes -band [IO.FileAttributes]::Directory) -eq 0 -or
            ($attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw 'prestart-activation:directory-kind'
        }
    }
    if ([IO.DriveInfo]::new($Stage.Substring(0, 3)).DriveType -ne [IO.DriveType]::Fixed -or
        $RecordsIn -ieq $Stage -or $Stage.StartsWith($RecordsIn + '\', [StringComparison]::OrdinalIgnoreCase) -or
        $RecordsIn.StartsWith($Stage + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw 'prestart-activation:staging-boundary'
    }
    for ($parent = [IO.Directory]::GetParent($Stage); $null -ne $parent; $parent = $parent.Parent) {
        if (($parent.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw 'prestart-activation:staging-boundary'
        }
    }

    [string]$entryPath = [IO.Path]::Combine($Stage, 'prestart_deployment_owner.ps1')
    if ($entryPath.Length -gt 512) { throw 'prestart-activation:directory-path' }
    [string]$inputPath = [IO.Path]::Combine($RecordsIn, 'deployment-activation.bin')
    [string]$commitPath = $inputPath + '.commit'
    foreach ($path in @($commitPath, $inputPath)) {
        if (([IO.File]::GetAttributes($path) -band
            ([IO.FileAttributes]::Directory -bor [IO.FileAttributes]::ReparsePoint)) -ne 0) {
            throw 'prestart-activation:record-kind'
        }
    }
    [IO.FileStream]$commit = [IO.File]::Open($commitPath, 'Open', 'Read', 'Read')
    try {
        if ($commit.Length -ne 0) { throw 'prestart-activation:commit' }
        [IO.FileStream]$inputStream = [IO.File]::Open($inputPath, 'Open', 'Read', 'Read')
        try {
            [int]$maxEntry = 2 * 1024 * 1024
            [int]$maxHeader = 144
            [long]$size = $inputStream.Length
            if ($size -le 0 -or $size -gt ($maxEntry + $maxHeader)) { throw 'prestart-activation:extent' }
            [IO.BinaryReader]$reader = [IO.BinaryReader]::new($inputStream, [Text.Encoding]::ASCII, $true)
            try { [byte[]]$frame = $reader.ReadBytes([int]$size) } finally { $reader.Dispose() }
            if ($frame.Length -ne $size -or $inputStream.ReadByte() -ne -1) { throw 'prestart-activation:extent' }
        } finally { $inputStream.Dispose() }
    } finally { $commit.Dispose() }

    [int]$newline = [Array]::IndexOf($frame, [byte]10, 0, [Math]::Min($maxHeader, $frame.Length))
    if ($newline -lt 0) { throw 'prestart-activation:header' }
    [string]$header = [Text.Encoding]::ASCII.GetString($frame, 0, $newline)
    if ($header -cnotmatch ('\Anevrotick-prestart-activation-v1 ' + $RunId.ToString('D') +
            ' ([1-9][0-9]{0,6}) ([0-9a-f]{64})\z')) {
        throw 'prestart-activation:header'
    }
    [int]$count = [int]$Matches[1]
    [string]$pin = $Matches[2]
    [int]$offset = $newline + 1
    if ($count -gt $maxEntry -or $frame.Length - $offset -ne $count) { throw 'prestart-activation:extent' }
    [Security.Cryptography.SHA256]$hash = [Security.Cryptography.SHA256CryptoServiceProvider]::new()
    try {
        [string]$digest = [BitConverter]::ToString($hash.ComputeHash($frame, $offset, $count)).Replace('-', '').ToLowerInvariant()
        if ($digest -cne $pin) { throw 'prestart-activation:digest' }
        if ($Clock.ElapsedMilliseconds -ge $WorkMilliseconds) { throw 'prestart-activation:deadline' }
        [IO.FileStream]$writer = [IO.File]::Open($entryPath, 'CreateNew', 'Write', 'None')
        try { $writer.Write($frame, $offset, $count); $writer.Flush($true) } finally { $writer.Dispose() }

        [IO.FileStream]$lease = [IO.File]::Open($entryPath, 'Open', 'Read', 'Read')
        try {
            $digest = [BitConverter]::ToString($hash.ComputeHash($lease)).Replace('-', '').ToLowerInvariant()
            if ($lease.Length -ne $count -or $digest -cne $pin) { throw 'prestart-activation:staged-entry' }
            if ($Clock.ElapsedMilliseconds -ge $WorkMilliseconds) { throw 'prestart-activation:deadline' }
            $lease.Position = 0
            [object]$result = [pscustomobject]@{ Path = $entryPath; Bytes = $count; Sha256 = $pin; Lease = $lease }
        } catch {
            $lease.Dispose()
            throw
        }
        return $result
    } finally { $hash.Dispose() }
}

function Start-PrestartDeploymentActivation {
    <#
    .SYNOPSIS
    Start the fixed watchdog on the exact locally staged entry.
    .DESCRIPTION
    The transaction supplies a fresh Process and empty hashtable and retains both on
    every return, including failure. An enclosing owner must enforce the original
    absolute deadlines and retain native custody even if Process.Start throws before
    attaching a handle. The worker owns the redirected pipes, observes watchdog exit
    and nested job retirement separately, then closes the two read leases in Custody.
    A return from Start establishes no settlement.
    #>
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory)][string]$RecordsIn,
        [Parameter(Mandatory)][string]$Stage,
        [Parameter(Mandatory)][guid]$RunId,
        [Parameter(Mandatory)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory)][ValidateRange(1, 119999)][int]$WorkMilliseconds,
        [Parameter(Mandatory)][ValidateRange(2, 120000)][int]$RetirementMilliseconds,
        [Parameter(Mandatory)][ValidateRange(1, 2097152)][long]$WatchdogBytes,
        [Parameter(Mandatory)][ValidatePattern('\A[0-9a-f]{64}\z')][string]$WatchdogSha256,
        [Parameter(Mandatory)][ValidateRange(1, 2097152)][long]$GuardianBytes,
        [Parameter(Mandatory)][ValidatePattern('\A[0-9a-f]{64}\z')][string]$GuardianSha256,
        [Parameter(Mandatory)][ValidateRange(1, 2097152)][long]$PowerShellBytes,
        [Parameter(Mandatory)][ValidatePattern('\A[0-9a-f]{64}\z')][string]$PowerShellSha256,
        [Parameter(Mandatory)][ValidateRange(1, 119999)][int]$GuardianWork,
        [Parameter(Mandatory)][ValidateRange(2, 120000)][int]$GuardianRetirement,
        [Parameter(Mandatory)][Diagnostics.Process]$Process,
        [Parameter(Mandatory)][AllowEmptyCollection()][hashtable]$Custody
    )
    $ErrorActionPreference = 'Stop'
    if ($Custody.Count -ne 0 -or !$Clock.IsRunning -or
        $RetirementMilliseconds -le $WorkMilliseconds -or $RetirementMilliseconds - $WorkMilliseconds -gt 30000 -or
        $GuardianRetirement -le $GuardianWork -or $GuardianRetirement - $GuardianWork -gt 30000 -or
        $WatchdogSha256 -cnotmatch '\A[0-9a-f]{64}\z' -or $GuardianSha256 -cnotmatch '\A[0-9a-f]{64}\z' -or
        $PowerShellSha256 -cnotmatch '\A[0-9a-f]{64}\z') {
        throw 'prestart-activation:launch-profile'
    }
    $Custody.Attempted = $false
    $Custody.Entry = Receive-PrestartDeploymentActivation -RecordsIn $RecordsIn -Stage $Stage -RunId $RunId `
        -Clock $Clock -WorkMilliseconds $WorkMilliseconds
    [string]$watchdog = [IO.Path]::Combine($Stage, 'prestart_deployment_watchdog.exe')
    [string]$guardian = [IO.Path]::Combine($Stage, 'prestart_deployment_guardian.ps1')
    if ($watchdog.Length -gt 512 -or $guardian.Length -gt 512 -or
        ([IO.File]::GetAttributes($watchdog) -band
            ([IO.FileAttributes]::Directory -bor [IO.FileAttributes]::ReparsePoint)) -ne 0) {
        throw 'prestart-activation:watchdog-path'
    }
    $Custody.Watchdog = [IO.File]::Open($watchdog, 'Open', 'Read', 'Read')
    if ($Custody.Watchdog.Length -ne $WatchdogBytes) { throw 'prestart-activation:watchdog-extent' }
    [Security.Cryptography.SHA256]$algorithm = [Security.Cryptography.SHA256CryptoServiceProvider]::new()
    try {
        [string]$digest = [BitConverter]::ToString($algorithm.ComputeHash($Custody.Watchdog)).Replace('-', '').ToLowerInvariant()
    } finally { $algorithm.Dispose() }
    if ($digest -cne $WatchdogSha256) { throw 'prestart-activation:watchdog-digest' }
    [Diagnostics.ProcessStartInfo]$start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $watchdog
    $start.WorkingDirectory = $Stage
    $start.UseShellExecute = $false
    $start.RedirectStandardInput = $true
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    $Process.StartInfo = $start
    [Text.Encoding]$originalEncoding = [Console]::InputEncoding
    [bool]$changeEncoding = $originalEncoding.GetPreamble().Length -ne 0
    try {
        if ($changeEncoding) { [Console]::InputEncoding = [Text.UTF8Encoding]::new($false) }
        [long]$elapsed = $Clock.ElapsedMilliseconds
        if ($elapsed -ge $WorkMilliseconds -or !$Clock.IsRunning) { throw 'prestart-activation:deadline' }
        [string[]]$arguments = @(('"' + $guardian + '"'), $GuardianBytes, $GuardianSha256, $PowerShellBytes,
            $PowerShellSha256, ('"' + $Custody.Entry.Path + '"'), $Custody.Entry.Bytes, $Custody.Entry.Sha256,
            $RunId.ToString('D'), $GuardianWork, $GuardianRetirement, ($WorkMilliseconds - $elapsed),
            ($RetirementMilliseconds - $elapsed))
        $start.Arguments = $arguments -join ' '
        $Custody.Attempted = $true
        if (!$Process.Start()) { throw 'prestart-activation:watchdog-start' }
        $Custody.Started = $true
    } finally { if ($changeEncoding) { [Console]::InputEncoding = $originalEncoding } }
    if ($Clock.ElapsedMilliseconds -ge $WorkMilliseconds) { throw 'prestart-activation:deadline' }
}

Export-ModuleMember -Function Receive-PrestartDeploymentActivation, Start-PrestartDeploymentActivation
