# GUEST ONLY: compile the fixed reviewed helper and exercise read-only controls.
# No Install/Remove call, certificate/trust mutation, listener or native client.
param(
    [Parameter(Mandatory = $true)][string]$RunId,
    [Parameter(Mandatory = $true)][string]$StoreSha256
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ($RunId -cnotmatch '\A[A-Za-z0-9_-]{1,48}\z' -or
    $StoreSha256 -cnotmatch '\A[0-9a-f]{64}\z' -or
    [Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or
    [Environment]::UserName -cne 'WDAGUtilityAccount' -or
    [Environment]::OSVersion.Version.Build -ne 26100 -or
    -not [Environment]::Is64BitProcess) { throw 'store-control-admission' }

$inputRoot = 'C:\NevroTickKeInput_' + $RunId
$outputRoot = 'C:\NevroTickKeOutput_' + $RunId
$source = $inputRoot + '\nts-ke-store.cs'
$scratch = 'C:\NevroTickKeStoreScratch_' + $RunId
$done = $outputRoot + '\store-control.json'
$temporary = $outputRoot + '\store-control.tmp'
if (-not [IO.Directory]::Exists($inputRoot) -or -not [IO.Directory]::Exists($outputRoot) -or
    [IO.Directory]::Exists($scratch) -or [IO.File]::Exists($scratch) -or
    [IO.File]::Exists($done) -or [IO.File]::Exists($temporary) -or
    ([IO.File]::GetAttributes($source) -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
    throw 'store-control-paths'
}
$hold = [IO.FileStream]::new($source, [IO.FileMode]::Open,
    [IO.FileAccess]::Read, [IO.FileShare]::Read)
$oldTemp = $env:TEMP
$oldTmp = $env:TMP
$created = $false
$phase = 'admission'
$failure = $null
$cleanup = [Collections.Generic.List[string]]::new()
$facts = [ordered]@{ schema = 1; run_id = $RunId; status = 'incomplete'; source_sha256 = $StoreSha256;
    store_mutation = $false; listener = $false; native_client = $false; controls = @(); snapshots_equal = $false;
    power_shell_version = $PSVersionTable.PSVersion.ToString(); execution_policy = (Get-ExecutionPolicy).ToString() }
$clock = [Diagnostics.Stopwatch]::StartNew()
function Check-Preparation {
    if ($clock.ElapsedMilliseconds -ge 60000) { throw 'store-control-deadline' }
}
function Read-ClosedFailures([Exception]$Failure) {
    $current = $Failure
    for ($depth = 0; $depth -lt 8; $depth++) {
        if ($current -is [AggregateException] -or $null -eq $current.InnerException) { break }
        $current = $current.InnerException
    }
    $items = @(if ($current -is [AggregateException]) { $current.InnerExceptions } else { $current })
    if ($items.Count -lt 1 -or $items.Count -gt 3) { throw 'store-control-failure-bound' }
    foreach ($item in $items) {
        $kind = $item.GetType().Name
        if ($kind -cnotin @('Win32Exception', 'InvalidOperationException', 'ArgumentException',
            'OutOfMemoryException', 'MethodInvocationException', 'RuntimeException')) { $kind = 'Other' }
        $operation = if ($item.Message -cmatch '\Ake-store:(open|enumerate|free|close|add|delete|context-layout|guest-profile|input-bound|input-identity|entry-bound|encoded-bound|fixture-preexists|already-owned|owned-entry-missing)\z') {
            $Matches[1]
        } else { 'unclassified' }
        $code = if ($item -is [ComponentModel.Win32Exception]) { $item.NativeErrorCode } else { $null }
        [pscustomobject]@{ type = $kind; operation = $operation; native_error = $code }
    }
}
try {
    Check-Preparation
    if ($hold.Length -lt 1 -or $hold.Length -gt 65536) { throw 'store-control-source-bound' }
    $sha = [Security.Cryptography.SHA256]::Create()
    try { $actual = ([BitConverter]::ToString($sha.ComputeHash($hold))).Replace('-', '').ToLowerInvariant() }
    finally { $sha.Dispose() }
    if ($actual -cne $StoreSha256) { throw 'store-control-source-identity' }
    $runtime = [Runtime.InteropServices.RuntimeEnvironment]::GetRuntimeDirectory()
    $compiler = [IO.Path]::Combine($runtime, 'csc.exe')
    if ($runtime -cne 'C:\Windows\Microsoft.NET\Framework64\v4.0.30319\' -or
        -not [IO.File]::Exists($compiler)) { throw 'store-control-compiler' }
    $facts.compiler = [ordered]@{ path = $compiler;
        sha256 = (Get-FileHash -LiteralPath $compiler -Algorithm SHA256).Hash.ToLowerInvariant();
        version = [Diagnostics.FileVersionInfo]::GetVersionInfo($compiler).FileVersion }
    $null = [IO.Directory]::CreateDirectory($scratch)
    $created = $true
    $env:TEMP = $scratch
    $env:TMP = $scratch
    $phase = 'compile'
    Check-Preparation
    Add-Type -Path $source -ErrorAction Stop
    Check-Preparation
    $phase = 'controls'
    $primary = [ComponentModel.Win32Exception]::new(5, 'ke-store:open')
    $secondary = [ComponentModel.Win32Exception]::new(6, 'ke-store:close')
    $aggregate = [AggregateException]::new('ke-store:cleanup', [Exception[]]@($primary, $secondary))
    $ordered = @(Read-ClosedFailures $aggregate)
    $single = @(Read-ClosedFailures $primary)
    if ($ordered.Count -ne 2 -or $ordered[0].operation -cne 'open' -or
        $ordered[0].native_error -ne 5 -or $ordered[1].operation -cne 'close' -or
        $ordered[1].native_error -ne 6 -or $single.Count -ne 1 -or
        $single[0].operation -cne 'open') { throw 'store-control-error-order' }
    $facts.controls += 'ordered-primary-cleanup-facts'
    [byte[]]$fixture = @(1, 2, 3)
    $sha = [Security.Cryptography.SHA256]::Create()
    try { $digest = ([BitConverter]::ToString($sha.ComputeHash($fixture))).Replace('-', '').ToLowerInvariant() }
    finally { $sha.Dispose() }
    # Deliberately inert constructor bytes; no install/remove method is called.
    $positive = [NevroTickGuestKe.StoreFixture]::new($fixture, $digest, $fixture, $digest)
    $facts.controls += 'valid-inert-construction'
    try {
        $null = [NevroTickGuestKe.StoreFixture]::new($fixture, ('0' * 64), $fixture, $digest)
        throw 'store-control-hash-accepted'
    } catch {
        if ($_.Exception.GetBaseException() -isnot [ArgumentException] -or
            $_.Exception.GetBaseException().Message -cne 'ke-store:input-identity') { throw }
        $facts.controls += 'wrong-hash-refused'
    }
    try {
        $null = [NevroTickGuestKe.StoreFixture]::new((New-Object byte[] 16385), $digest, $fixture, $digest)
        throw 'store-control-size-accepted'
    } catch {
        if ($_.Exception.GetBaseException() -isnot [ArgumentException] -or
            $_.Exception.GetBaseException().Message -cne 'ke-store:input-bound') { throw }
        $facts.controls += 'oversize-refused'
    }
    $phase = 'physical-store-readback'
    Check-Preparation
    $before = @($positive.Snapshot())
    Check-Preparation
    $after = @($positive.Snapshot())
    Check-Preparation
    $facts.physical_store_before = $before
    $facts.physical_store_after = $after
    $facts.snapshots_equal = ($before.Count -eq $after.Count -and
        [string]::Join("`n", $before) -ceq [string]::Join("`n", $after))
    if (-not $facts.snapshots_equal) { throw 'store-control-drift' }
    $facts.controls += 'bounded-readonly-snapshots'
    if ((Get-FileHash -LiteralPath $compiler -Algorithm SHA256).Hash.ToLowerInvariant() -cne
        $facts.compiler.sha256) { throw 'store-control-compiler-drift' }
    $facts.status = 'passed_readonly_store_controls'
} catch {
    $failure = $phase
    # Preserve ordered primary/secondary facts, without arbitrary error payloads.
    $facts.failure_facts = @(Read-ClosedFailures $_.Exception)
    if ($phase -ceq 'compile') {
        # Compiler diagnostics refer only to the fixed public C# source, never
        # runtime trust/key input. Retain a bounded setup error for repair.
        $text = $_.Exception.Message
        $facts.compile_diagnostic = $text.Substring(0, [Math]::Min(4096, $text.Length))
    }
} finally {
    $env:TEMP = $oldTemp
    $env:TMP = $oldTmp
    if ($created) {
        try {
            $rootAttributes = [IO.File]::GetAttributes($scratch)
            if (($rootAttributes -band [IO.FileAttributes]::Directory) -eq 0 -or
                ($rootAttributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'scratch-root' }
            $entries = [Collections.Generic.List[string]]::new()
            [long]$total = 0
            foreach ($path in [IO.Directory]::EnumerateFileSystemEntries($scratch)) {
                if ($entries.Count -ge 64) { throw 'scratch-count' }
                $attributes = [IO.File]::GetAttributes($path)
                if (($attributes -band ([IO.FileAttributes]::Directory -bor
                    [IO.FileAttributes]::ReparsePoint)) -ne 0) { throw 'scratch-kind' }
                $total += [IO.FileInfo]::new($path).Length
                if ($total -gt 8388608) { throw 'scratch-bytes' }
                $entries.Add($path)
            }
            foreach ($path in $entries) { [IO.File]::Delete($path) }
            [IO.Directory]::Delete($scratch, $false)
        } catch { $cleanup.Add('owned-compiler-scratch') }
    }
    $hold.Dispose()
}
$facts.elapsed_ms = $clock.ElapsedMilliseconds
if ($facts.elapsed_ms -ge 60000 -and $null -eq $failure) { $failure = 'completion-deadline' }
$facts.failure_phase = $failure
$facts.cleanup_gaps = @($cleanup.ToArray())
if ($null -ne $failure -or $cleanup.Count -ne 0) { $facts.status = 'incomplete' }
$encoded = [Text.UTF8Encoding]::new($false).GetBytes(($facts | ConvertTo-Json -Depth 7 -Compress))
if ($encoded.Length -gt 524288) { throw 'store-control-receipt-bound' }
$writer = [IO.FileStream]::new($temporary, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write,
    [IO.FileShare]::None)
try { $writer.Write($encoded, 0, $encoded.Length); $writer.Flush($true) }
finally { $writer.Dispose() }
[IO.File]::Move($temporary, $done)
if ($facts.status -cne 'passed_readonly_store_controls') { exit 1 }
exit 0
