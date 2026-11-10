# GUEST ONLY, one explicitly owned prepared Sandbox; never run on the host.
# CRYPTO-01/02/04, REQ-8/BND-2, AC-04, CHECK-6/8/9/17/21/25/26/27.
# Source preparation only until the coordinator admits the guest and headroom.
# Four cold pairs, fixed synthetic CurrentUser Root/CA entries, exact restoration.
param(
    [Parameter(Mandatory = $true)][string]$RunId,
    [Parameter(Mandatory = $true)][string]$Identity,
    [Parameter(Mandatory = $true)][string]$ControllerSha256,
    [Parameter(Mandatory = $true)][string]$ClientSha256,
    [Parameter(Mandatory = $true)][string]$PeerSha256,
    [Parameter(Mandatory = $true)][string]$RootSha256,
    [Parameter(Mandatory = $true)][string]$LeafSha256,
    [Parameter(Mandatory = $true)][string]$KeySha256,
    [Parameter(Mandatory = $true)][string]$CrlSha256
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$clock = [Diagnostics.Stopwatch]::StartNew()
$state = @{ holds = [Collections.Generic.List[object]]::new();
    children = [Collections.Generic.List[object]]::new();
    captures = [Collections.Generic.List[object]]::new();
    cleanup = [Collections.Generic.List[object]]::new(); poll_failures = @{}; progress = 0;
    phase = 'admission'; baseline = @(); baseline_hash = $null;
    insertion = 'none'; scratch_created = $false; output_admitted = $false }
$store = $null
$primary = $null
$oldTemp = $env:TEMP
$oldTmp = $env:TMP
$facts = [ordered]@{ schema = 1; scope = 'guest-controlled-healthy-ke'; run_id = $RunId;
    status = 'incomplete'; cases = @(); restoration = 'not-started';
    force_killed = $false; native_cleanup_qualified = $false;
    budget = @{ setup_ms = 60000; case_ms = 90000; cases = 4; cleanup_ms = 60000;
        cooperative_whole_ms = 480000; capture_bytes_per_stream = 4096 } }

# Every deadline is from this invocation's original monotonic clock. Native
# compiler/store/OS observations are cooperative checks, not hard cancellation.
function Check-Time([long]$Deadline) {
    if ($clock.ElapsedMilliseconds -ge $Deadline) { throw 'ke-controller:deadline' }
}
function Digest([IO.Stream]$Stream) {
    $sha = [Security.Cryptography.SHA256]::Create()
    try {
        $Stream.Position = 0
        return ([BitConverter]::ToString($sha.ComputeHash($Stream))).Replace('-', '').ToLowerInvariant()
    } finally { $sha.Dispose(); $Stream.Position = 0 }
}
function Open-Pinned([string]$Path, [string]$Expected, [long]$Limit) {
    $attr = [IO.File]::GetAttributes($Path)
    if (($attr -band ([IO.FileAttributes]::Directory -bor [IO.FileAttributes]::ReparsePoint)) -ne 0) {
        throw 'ke-controller:input-kind'
    }
    $held = [IO.FileStream]::new($Path, [IO.FileMode]::Open,
        [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        if ($held.Length -lt 1 -or $held.Length -gt $Limit -or (Digest $held) -cne $Expected) {
            throw 'ke-controller:input-identity'
        }
        $item = [pscustomobject]@{ Path = $Path; Stream = $held; Bytes = $held.Length; Sha256 = $Expected }
        $state.holds.Add($item)
        return $item
    } catch { $held.Dispose(); throw }
}
function Read-Pinned($Item) {
    $Item.Stream.Position = 0
    $bytes = New-Object byte[] ([int]$Item.Bytes)
    $offset = 0
    while ($offset -lt $bytes.Length) {
        $count = $Item.Stream.Read($bytes, $offset, $bytes.Length - $offset)
        if ($count -eq 0) { throw 'ke-controller:input-short' }
        $offset += $count
    }
    if ($Item.Stream.ReadByte() -ne -1) { throw 'ke-controller:input-growth' }
    $Item.Stream.Position = 0
    return ,$bytes
}
function Check-Pins {
    foreach ($item in $state.holds) {
        if ($item.Stream.Length -ne $item.Bytes -or (Digest $item.Stream) -cne $item.Sha256 -or
            ([IO.File]::GetAttributes($item.Path) -band
                ([IO.FileAttributes]::Directory -bor [IO.FileAttributes]::ReparsePoint)) -ne 0) {
            throw 'ke-controller:input-drift'
        }
    }
}
function New-PrivateDirectory([string]$Path) {
    if ([IO.File]::Exists($Path) -or [IO.Directory]::Exists($Path)) { throw 'ke-controller:scratch-exists' }
    $security = [Security.AccessControl.DirectorySecurity]::new()
    $security.SetAccessRuleProtection($true, $false)
    $inherit = [Security.AccessControl.InheritanceFlags]'ContainerInherit,ObjectInherit'
    foreach ($sid in @([Security.Principal.WindowsIdentity]::GetCurrent().User,
        [Security.Principal.SecurityIdentifier]::new('S-1-5-18'))) {
        $rule = [Security.AccessControl.FileSystemAccessRule]::new($sid,
            [Security.AccessControl.FileSystemRights]::FullControl, $inherit,
            [Security.AccessControl.PropagationFlags]::None, [Security.AccessControl.AccessControlType]::Allow)
        $null = $security.AddAccessRule($rule)
    }
    $null = [IO.Directory]::CreateDirectory($Path, $security)
    $attributes = [IO.File]::GetAttributes($Path)
    if (($attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'ke-controller:scratch-kind' }
}
function Write-NewJson([string]$Name, $Value, [int]$Limit) {
    if ($Name -cnotmatch '\A[a-z0-9-]+\.json\z') { throw 'ke-controller:output-name' }
    $encoded = [Text.UTF8Encoding]::new($false).GetBytes(($Value | ConvertTo-Json -Depth 12 -Compress))
    if ($encoded.Length -gt $Limit) { throw 'ke-controller:receipt-bound' }
    $path = $outputRoot + '\' + $Name
    $temporary = $path + '.pending'
    $writer = [IO.FileStream]::new($temporary, [IO.FileMode]::CreateNew,
        [IO.FileAccess]::Write, [IO.FileShare]::None)
    try { $writer.Write($encoded, 0, $encoded.Length); $writer.Flush($true) }
    finally { $writer.Dispose() }
    [IO.File]::Move($temporary, $path)
    $sha = [Security.Cryptography.SHA256]::Create()
    try { return ([BitConverter]::ToString($sha.ComputeHash($encoded))).Replace('-', '').ToLowerInvariant() }
    finally { $sha.Dispose() }
}
# Project only fixed operation labels/numeric native causes. No exception text,
# compiler source excerpts, stack, environment, DER or key bytes are exported.
function Closed-Failures([Exception]$Failure) {
    $current = $Failure
    for ($depth = 0; $depth -lt 8; $depth++) {
        if ($current -is [AggregateException] -or $null -eq $current.InnerException) { break }
        $current = $current.InnerException
    }
    $items = @(if ($current -is [AggregateException]) { $current.InnerExceptions } else { $current })
    if ($items.Count -lt 1 -or $items.Count -gt 3) {
        return [pscustomobject]@{ type = 'Other'; owner = 'controller'; operation = 'failure-bound'; native_error = $null }
    }
    foreach ($item in $items) {
        $kind = $item.GetType().Name
        if ($kind -cnotin @('Win32Exception', 'InvalidOperationException', 'ArgumentException',
            'OutOfMemoryException', 'MethodInvocationException', 'RuntimeException')) { $kind = 'Other' }
        $owner = 'unknown'; $operation = 'unclassified'
        if ($item.Message -cmatch '\Ake-store:(open|enumerate|free|close|add|delete|context-layout|guest-profile|input-bound|input-identity|entry-bound|encoded-bound|fixture-preexists|already-owned|owned-entry-missing)\z') {
            $owner = 'store'; $operation = $Matches[1]
        } elseif ($item.Message -cmatch '\Ake-controller:([a-z-]{1,48})\z') {
            $owner = 'controller'; $operation = $Matches[1]
        } elseif ($item.Message -cmatch '\A(capture-limit|capture-link|capture-short|capture-growth|capture-non-ascii)\z') {
            $owner = 'capture'; $operation = $Matches[1]
        }
        $code = if ($item -is [ComponentModel.Win32Exception]) { $item.NativeErrorCode } else { $null }
        [pscustomobject]@{ type = $kind; owner = $owner; operation = $operation; native_error = $code }
    }
}
function Child-Facts {
    foreach ($child in $state.children) {
        [ordered]@{ role = $child.Role; case = $child.Case; started = $child.Started;
            launch_state = $child.LaunchState;
            pid = $child.PidValue; start_utc_ticks = $child.StartTicks; image = $child.Image;
            image_sha256 = $child.Sha256; reaped = $child.Reaped; exit_code = $child.ExitCode;
            force_killed = $child.Killed }
    }
}
function Publish-Progress([string]$Phase) {
    if ($state.progress -ge 32) { throw 'ke-controller:progress-bound' }
    $state.progress++
    $value = [ordered]@{ schema = 1; status = 'incomplete'; phase = $Phase;
        controller = $facts.controller; baseline_file = 'healthy-ke-baseline.json';
        baseline_sha256 = $state.baseline_hash; public_pki = $facts.public_pki;
        root_owned = ($null -ne $store -and $store.RootOwned);
        crl_owned = ($null -ne $store -and $store.CrlOwned); last_insertion = $state.insertion;
        children = @(Child-Facts); elapsed_ms = $clock.ElapsedMilliseconds;
        recovery_owner = 'root-coordinator-after-actual-child-completion' }
    $null = Write-NewJson ('healthy-ke-progress-' + $state.progress + '.json') $value 32768
}
function Assert-StoreState {
    $expected = [Collections.Generic.List[string]]::new()
    foreach ($row in $state.baseline) { $expected.Add($row) }
    if ($store.RootOwned) { $expected.Add('Root|cert|' + $RootSha256) }
    if ($store.CrlOwned) { $expected.Add('CA|crl|' + $CrlSha256) }
    $expected.Sort([StringComparer]::Ordinal)
    $actual = @($store.Snapshot())
    if ($actual.Count -ne $expected.Count -or
        [string]::Join("`n", $actual) -cne [string]::Join("`n", $expected.ToArray())) {
        throw 'ke-controller:store-drift'
    }
}
function Start-Child([string]$Role, [string]$Case, [string]$Image, [string]$Sha256,
    [string[]]$Arguments, [string]$Directory, [long]$Deadline) {
    Check-Time $Deadline
    foreach ($argument in $Arguments) {
        if ($argument.Length -gt 1024 -or $argument -cnotmatch '\A[\x20-\x21\x23-\x7e]+\z') {
            throw 'ke-controller:child-arguments'
        }
    }
    $info = [Diagnostics.ProcessStartInfo]::new()
    $info.FileName = $Image; $info.Arguments = '"' + [string]::Join('" "', $Arguments) + '"'
    $info.WorkingDirectory = $Directory; $info.UseShellExecute = $false
    $info.RedirectStandardOutput = $true; $info.RedirectStandardError = $true; $info.CreateNoWindow = $true
    $info.EnvironmentVariables.Clear()
    $info.EnvironmentVariables['SystemRoot'] = 'C:\Windows'
    $info.EnvironmentVariables['PATH'] = 'C:\Windows\System32;C:\Windows'
    $info.EnvironmentVariables['TEMP'] = $Directory; $info.EnvironmentVariables['TMP'] = $Directory
    $process = [Diagnostics.Process]::new(); $process.StartInfo = $info
    $child = [pscustomobject]@{ Process = $process; Role = $Role; Case = $Case; Image = $Image;
        Sha256 = $Sha256; Started = $false; PidValue = $null; StartTicks = $null;
        Reaped = $false; ExitCode = $null; Killed = $false; LaunchState = 'intent' }
    $state.children.Add($child)
    Publish-Progress ('before-' + $Case + '-' + $Role + '-launch')
    Check-Time $Deadline
    $launchFailure = $null; $ackFailure = $null
    # A throwing/partial Start is not evidence that no process exists. Preserve
    # that uncertainty even if some identity fields can subsequently be read.
    $child.LaunchState = 'unknown'
    try {
        if (-not $process.Start()) {
            $child.LaunchState = 'not-started-confirmed'
            throw 'ke-controller:child-start'
        }
        $child.Started = $true; $child.PidValue = $process.Id
        $child.StartTicks = $process.StartTime.ToUniversalTime().Ticks
        $child.LaunchState = 'started'
    } catch {
        $launchFailure = $_
        if ($child.LaunchState -ceq 'unknown') {
            try { $child.PidValue = $process.Id; $child.Started = $true } catch { }
            try { $child.StartTicks = $process.StartTime.ToUniversalTime().Ticks } catch { }
        }
    } finally {
        # This acknowledgment precedes capture creation, which can itself fail.
        try { Publish-Progress ('after-' + $Case + '-' + $Role + '-launch') }
        catch { $ackFailure = $_; $state.cleanup.Add(@{ phase = 'launch-acknowledgment'; failures = @(Closed-Failures $_.Exception) }) }
    }
    if ($null -ne $launchFailure) { throw $launchFailure }
    if ($null -ne $ackFailure) { throw $ackFailure }
    Check-Time $Deadline
    foreach ($channel in @('stdout', 'stderr')) {
        $stream = if ($channel -ceq 'stdout') { $process.StandardOutput.BaseStream } else { $process.StandardError.BaseStream }
        $capture = New-Capture $stream ($Directory + '\' + $Role + '-' + $channel + '.raw')
        $capture | Add-Member -NotePropertyName Broken -NotePropertyValue $false
        $capture | Add-Member -NotePropertyName Closed -NotePropertyValue $false
        $capture | Add-Member -NotePropertyName Role -NotePropertyValue $Role
        $capture | Add-Member -NotePropertyName Channel -NotePropertyValue $channel
        $state.captures.Add($capture)
    }
    Check-Time $Deadline
    return $child
}
function Poll-Children {
    foreach ($capture in $state.captures) {
        if (-not $capture.Closed -and -not $capture.Broken) {
            try { Pump-Capture $capture } catch { $capture.Broken = $true; throw }
        }
    }
    foreach ($child in $state.children) {
        if ($child.Started -and -not $child.Reaped -and $child.Process.HasExited) {
            if (-not $child.Process.WaitForExit(0)) { throw 'ke-controller:reap-state' }
            $child.ExitCode = $child.Process.ExitCode; $child.Reaped = $true
        }
    }
}
function Read-Ready([string]$Path) {
    $attr = [IO.File]::GetAttributes($Path)
    if (($attr -band ([IO.FileAttributes]::ReparsePoint -bor [IO.FileAttributes]::Directory)) -ne 0) {
        throw 'ke-controller:ready-kind'
    }
    $reader = [IO.FileStream]::new($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        if ($reader.Length -lt 7 -or $reader.Length -gt 11) { throw 'ke-controller:ready-bound' }
        $bytes = New-Object byte[] ([int]$reader.Length)
        $offset = 0
        while ($offset -lt $bytes.Length) {
            $count = $reader.Read($bytes, $offset, $bytes.Length - $offset)
            if ($count -eq 0) { throw 'ke-controller:ready-short' }; $offset += $count
        }
        if ($reader.ReadByte() -ne -1) { throw 'ke-controller:ready-growth' }
        foreach ($value in $bytes) { if ($value -gt 127) { throw 'ke-controller:ready-ascii' } }
        $text = [Text.Encoding]::ASCII.GetString($bytes)
        if ($text -cnotmatch '\Aport=([1-9][0-9]{0,4})\n\z') { throw 'ke-controller:ready-syntax' }
        $portValue = [int]$Matches[1]
        if ($portValue -lt 1 -or $portValue -gt 65535) { throw 'ke-controller:ready-port' }
        return $portValue
    } finally { $reader.Dispose() }
}
function Admit-Labels([string]$Text, [bool]$Peer) {
    $pattern = if ($Peer) {
        '\Astage=([a-z-]+) cause=([a-z-]+) cleanup_stage=([a-z-]+) cleanup_cause=([a-z-]+) readiness_cleanup_cause=([a-z-]+)\n\z'
    } else {
        '\Astage=([a-z-]+) cause=([a-z-]+) cleanup_stage=([a-z-]+) cleanup_cause=([a-z-]+) admission_stage=([a-z-]+) admission_cause=([a-z-]+)\n\z'
    }
    if ($Text -cnotmatch $pattern) { throw 'ke-controller:diagnostic-shape' }
    $groups = @($Matches[1], $Matches[2], $Matches[3], $Matches[4], $Matches[5])
    if (-not $Peer) { $groups += $Matches[6] }
    $stages = @('none', 'admission', 'resources', 'currentness', 'request', 'response', 'request-sealing',
        'connect', 'tls', 'handshake', 'exporter', 'shutdown', 'io', 'input', 'closure', 'readiness')
    $causes = @('none', 'arguments', 'allocation', 'deadline', 'cancelled', 'control', 'parameters',
        'timeout', 'interrupted', 'refused', 'unexpected-eof', 'invalid-data', 'io', 'framing',
        'negotiation', 'cookie-budget', 'entropy', 'encoding', 'endpoint', 'peer-mismatch', 'setup',
        'profile', 'export', 'byte-budget', 'offer', 'file', 'missing-notify', 'already-exists',
        'permission', 'reset', 'broken-pipe')
    if ($groups[0] -cnotin $stages -or $groups[1] -cnotin $causes -or
        $groups[2] -cnotin $stages -or $groups[3] -cnotin $causes -or
        ($Peer -and $groups[4] -cnotin $causes) -or
        (-not $Peer -and ($groups[4] -cnotin $stages -or $groups[5] -cnotin $causes))) {
        throw 'ke-controller:diagnostic-label'
    }
    $result = [ordered]@{ stage = $groups[0]; cause = $groups[1]; cleanup_stage = $groups[2]; cleanup_cause = $groups[3] }
    if ($Peer) { $result.readiness_cleanup_cause = $groups[4] }
    else { $result.admission_stage = $groups[4]; $result.admission_cause = $groups[5] }
    return $result
}
function Run-Pair([string]$Case, [string]$ServerIdentity, [bool]$Healthy, [long]$Deadline) {
    $Deadline = [Math]::Min($Deadline, $clock.ElapsedMilliseconds + 90000)
    Check-Time $Deadline; Check-Pins; Assert-StoreState
    $directory = $scratch + '\cases\' + $Case
    New-PrivateDirectory $directory
    $ready = $directory + '\peer.ready'
    $captureStart = $state.captures.Count
    $row = [ordered]@{ name = $Case; status = 'incomplete'; port = $null; outputs = @(); client_labels = $null; peer_labels = $null }
    $facts.cases += $row
    $peer = Start-Child 'peer' $Case $peerInput.Path $PeerSha256 @($privateLeaf.Path, $privateKey.Path, $ready) $directory $Deadline
    while (-not [IO.File]::Exists($ready)) {
        Check-Time $Deadline; Poll-Children
        if ($peer.Reaped) { throw 'ke-controller:peer-before-ready' }
        Start-Sleep -Milliseconds 20
    }
    $portValue = Read-Ready $ready
    Check-Time $Deadline
    if ($peer.Process.HasExited -or $peer.Process.StartTime.ToUniversalTime().Ticks -ne $peer.StartTicks) {
        throw 'ke-controller:ready-peer-state'
    }
    # Fixed read-only NetTCPIP query, no probing connection or broad fallback.
    $listeners = @(NetTCPIP\Get-NetTCPConnection -LocalAddress '127.0.0.1' -LocalPort $portValue -State Listen -ErrorAction Stop)
    Check-Time $Deadline
    if ($listeners.Count -ne 1 -or $listeners[0].OwningProcess -ne $peer.PidValue -or
        $peer.Process.HasExited -or $peer.Process.StartTime.ToUniversalTime().Ticks -ne $peer.StartTicks) {
        throw 'ke-controller:listener-owner'
    }
    $row.port = $portValue
    $endpoint = '127.0.0.1:' + $portValue
    $client = Start-Child 'client' $Case $clientInput.Path $ClientSha256 @($endpoint, $ServerIdentity) $directory $Deadline
    while ($true) {
        Check-Time $Deadline; Poll-Children
        $pairCaptures = @($state.captures | Select-Object -Skip $captureStart)
        if ($client.Reaped -and $peer.Reaped -and @($pairCaptures | Where-Object { -not $_.Eof }).Count -eq 0) { break }
        Start-Sleep -Milliseconds 20
    }
    $raw = @{}
    foreach ($capture in $pairCaptures) {
        $capture.Writer.Dispose(); $capture.Stream.Dispose(); $capture.Closed = $true
        $value = Read-Capture $capture.Path
        $raw[$capture.Role + '-' + $capture.Channel] = $value
        $row.outputs += [ordered]@{ role = $capture.Role; channel = $capture.Channel;
            bytes = $value.Bytes; sha256 = $value.Sha256; admitted = $false }
    }
    if ($Healthy) {
        if ($client.ExitCode -ne 0 -or $peer.ExitCode -ne 0 -or
            $raw['client-stderr'].Text -cne '' -or $raw['peer-stderr'].Text -cne '' -or
            $raw['client-stdout'].Text -cne "scope=laboratory-ke completed=1 cookies=1 cookie_bytes=4 sealed_packet_bytes=132 transmitted=0 native_cleanup_qualified=0`n" -or
            $raw['peer-stdout'].Text -cne "scope=laboratory-ke-peer completed=1 offer_bytes=16 response_bytes=24 exporters=2 peer_close=1`n") {
            throw 'ke-controller:healthy-output'
        }
    } else {
        if ($client.ExitCode -ne 1 -or $peer.ExitCode -ne 1 -or
            $raw['client-stdout'].Text -cne '' -or $raw['peer-stdout'].Text -cne '') { throw 'ke-controller:negative-exit' }
        $row.client_labels = Admit-Labels $raw['client-stderr'].Text $false
        $row.peer_labels = Admit-Labels $raw['peer-stderr'].Text $true
        if ($row.client_labels.stage -cne 'handshake' -or $row.client_labels.cause -cne 'invalid-data') {
            throw 'ke-controller:negative-client-cause'
        }
        # Alert/EOF/reset races are admitted only in this closed IO family.
        # A deadline, budget, input, setup, readiness or cleanup failure is not
        # evidence of ordinary trust/name rejection.
        $negativeIo = @('invalid-data', 'unexpected-eof', 'reset', 'broken-pipe', 'io')
        if ($row.peer_labels.stage -cne 'io' -or $row.peer_labels.cause -cnotin $negativeIo -or
            $row.peer_labels.readiness_cleanup_cause -cne 'none' -or
            -not (($row.peer_labels.cleanup_stage -ceq 'none' -and $row.peer_labels.cleanup_cause -ceq 'none') -or
                ($row.peer_labels.cleanup_stage -ceq 'io' -and $row.peer_labels.cleanup_cause -cin $negativeIo))) {
            throw 'ke-controller:negative-peer-cause'
        }
        # The unchanged client maps reset/broken-pipe to its closed "io" label.
        # Preserve an ordinary later response/shutdown closure fault beside the
        # original handshake failure; currentness/admission failures stay open.
        $clientClosureIo = @('invalid-data', 'unexpected-eof', 'io')
        if (-not (($row.client_labels.cleanup_stage -ceq 'none' -and $row.client_labels.cleanup_cause -ceq 'none') -or
                ($row.client_labels.cleanup_stage -cin @('shutdown', 'response') -and
                    $row.client_labels.cleanup_cause -cin $clientClosureIo)) -or
            $row.client_labels.admission_stage -cne 'none' -or $row.client_labels.admission_cause -cne 'none') {
            throw 'ke-controller:negative-client-secondary'
        }
    }
    Check-Time $Deadline; Check-Pins; Assert-StoreState
    # Export raw child bytes only after exact literal/closed-label admission.
    foreach ($capture in $pairCaptures) {
        $destination = $outputRoot + '\healthy-ke-' + $Case + '-' + $capture.Role + '-' + $capture.Channel + '.txt'
        [IO.File]::Copy($capture.Path, $destination, $false)
    }
    Check-Time $Deadline
    foreach ($output in $row.outputs) { $output.admitted = $true }
    Publish-Progress ('completed-' + $Case + '-pair')
    Check-Time $Deadline
    $row.status = if ($Healthy) { 'passed_healthy_exchange' } else { 'passed_ordinary_verifier_refusal' }
}

try {
    if ($RunId -cnotmatch '\A[A-Za-z0-9_-]{1,48}\z' -or $Identity -cnotmatch '\Ake-[0-9a-f]{32}\.invalid\z') {
        throw 'ke-controller:arguments'
    }
    foreach ($pin in @($ControllerSha256, $ClientSha256, $PeerSha256, $RootSha256, $LeafSha256, $KeySha256, $CrlSha256)) {
        if ($pin -cnotmatch '\A[0-9a-f]{64}\z') { throw 'ke-controller:hash-arguments' }
    }
    if ($ClientSha256 -cne '39d2c7f7c8a094115a1d221c87f89f457dcc2e50c89b346e1c9d0975ab15f625' -or
        [Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or
        [Environment]::UserName -cne 'WDAGUtilityAccount' -or
        [Environment]::OSVersion.Version.Build -ne 26100 -or -not [Environment]::Is64BitProcess) {
        throw 'ke-controller:guest-profile'
    }
    $inputRoot = 'C:\NevroTickKeInput_' + $RunId
    $outputRoot = 'C:\NevroTickKeOutput_' + $RunId
    $scratch = 'C:\NevroTickKeHealthyScratch_' + $RunId
    if ($PSCommandPath -cne ($inputRoot + '\nts-ke-healthy-peer.ps1') -or
        -not [IO.Directory]::Exists($inputRoot) -or -not [IO.Directory]::Exists($outputRoot) -or
        ([IO.DriveInfo]::new('C:\')).DriveFormat -cne 'NTFS' -or
        ([IO.File]::GetAttributes('C:\') -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'ke-controller:paths'
    }
    $state.output_admitted = $true
    Check-Time 60000
    $null = Open-Pinned $PSCommandPath $ControllerSha256 65536
    $captureSource = Open-Pinned ($inputRoot + '\nts-ke-malformed-peer.ps1') '981de68fdf978f85e230d63e7146b9316dad0ecd0270aadf34a7196777fc891e' 65536
    $storeSource = Open-Pinned ($inputRoot + '\nts-ke-store.cs') 'f070740a3d111e7fc7e7ac98f12be9b323abc3fa16267fc6a2689ff0da089d40' 65536
    $clientInput = Open-Pinned ($inputRoot + '\nevrotick-nts-ke-probe.exe') $ClientSha256 16777216
    $peerInput = Open-Pinned ($inputRoot + '\nevrotick-nts-ke-peer.exe') $PeerSha256 16777216
    $rootInput = Open-Pinned ($inputRoot + '\root.der') $RootSha256 16384
    $leafInput = Open-Pinned ($inputRoot + '\leaf.der') $LeafSha256 16384
    $keyInput = Open-Pinned ($inputRoot + '\leaf.key.der') $KeySha256 16384
    $crlInput = Open-Pinned ($inputRoot + '\healthy.crl.der') $CrlSha256 65536
    $self = [Diagnostics.Process]::GetCurrentProcess()
    try { $facts.controller = @{ pid = $self.Id; start_utc_ticks = $self.StartTime.ToUniversalTime().Ticks;
        image = $self.MainModule.FileName; source_sha256 = $ControllerSha256 } } finally { $self.Dispose() }
    $facts.public_pki = @{ root_sha256 = $RootSha256; leaf_sha256 = $LeafSha256; crl_sha256 = $CrlSha256; identity = $Identity }
    $facts.private_key_identity_admitted = $true
    New-PrivateDirectory $scratch; $state.scratch_created = $true
    foreach ($name in @('compile', 'input', 'cases')) { New-PrivateDirectory ($scratch + '\' + $name) }
    foreach ($item in @($leafInput, $keyInput)) {
        $bytes = Read-Pinned $item
        try {
            $destination = $scratch + '\input\' + [IO.Path]::GetFileName($item.Path)
            $writer = [IO.FileStream]::new($destination, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
            try { $writer.Write($bytes, 0, $bytes.Length); $writer.Flush($true) } finally { $writer.Dispose() }
        } finally { [Array]::Clear($bytes, 0, $bytes.Length) }
    }
    $privateLeaf = Open-Pinned ($scratch + '\input\leaf.der') $LeafSha256 16384
    $privateKey = Open-Pinned ($scratch + '\input\leaf.key.der') $KeySha256 16384
    $text = [Text.UTF8Encoding]::new($false, $true).GetString((Read-Pinned $captureSource))
    $tokens = $null; $parseErrors = $null
    $ast = [Management.Automation.Language.Parser]::ParseInput($text, [ref]$tokens, [ref]$parseErrors)
    if ($parseErrors.Count -ne 0) { throw 'ke-controller:capture-source-parse' }
    foreach ($name in @('New-Capture', 'Pump-Capture', 'Read-Capture')) {
        $definitions = @($ast.EndBlock.Statements | Where-Object {
            $_ -is [Management.Automation.Language.FunctionDefinitionAst] -and $_.Name -ceq $name })
        if ($definitions.Count -ne 1) { throw 'ke-controller:capture-source-definition' }
        . ([scriptblock]::Create($definitions[0].Extent.Text))
    }
    if ($null -ne ('NevroTickGuestKe.StoreFixture' -as [type])) { throw 'ke-controller:preexisting-helper' }
    $runtime = [Runtime.InteropServices.RuntimeEnvironment]::GetRuntimeDirectory()
    if ($runtime -cne 'C:\Windows\Microsoft.NET\Framework64\v4.0.30319\') { throw 'ke-controller:compiler-profile' }
    $compiler = $runtime + 'csc.exe'
    $compilerPin = (Get-FileHash -LiteralPath $compiler -Algorithm SHA256).Hash.ToLowerInvariant()
    $facts.compiler = @{ path = $compiler; sha256 = $compilerPin;
        version = [Diagnostics.FileVersionInfo]::GetVersionInfo($compiler).FileVersion }
    $env:TEMP = $scratch + '\compile'; $env:TMP = $env:TEMP
    $state.phase = 'compile'; Check-Time 60000
    Add-Type -Path $storeSource.Path -ErrorAction Stop
    Check-Time 60000
    if ((Get-FileHash -LiteralPath $compiler -Algorithm SHA256).Hash.ToLowerInvariant() -cne $compilerPin) {
        throw 'ke-controller:compiler-drift'
    }
    $store = [NevroTickGuestKe.StoreFixture]::new((Read-Pinned $rootInput), $RootSha256, (Read-Pinned $crlInput), $CrlSha256)
    $state.phase = 'baseline'; Check-Time 60000
    $state.baseline = @($store.Snapshot()); $store.AssertAbsent()
    foreach ($row in $state.baseline) { if ($row.EndsWith('|' + $LeafSha256, [StringComparison]::Ordinal)) { throw 'ke-controller:leaf-preexists' } }
    $state.baseline_hash = Write-NewJson 'healthy-ke-baseline.json' @{ schema = 1; rows = $state.baseline } 524288
    Check-Time 60000; Check-Pins
    $state.phase = 'untrusted'; Run-Pair 'untrusted' $Identity $false 150000
    $state.phase = 'install-root'; Check-Time 150000; Assert-StoreState
    $state.insertion = 'root-intent'; Publish-Progress 'before-root-insertion'
    $insertionFailure = $null
    try { Check-Time 150000; $store.InstallRoot(); $state.insertion = 'root-returned' }
    catch { $insertionFailure = $_ }
    finally {
        try { Publish-Progress 'after-root-insertion' }
        catch { $state.cleanup.Add(@{ phase = 'root-insertion-progress'; failures = @(Closed-Failures $_.Exception) }) }
    }
    if ($null -ne $insertionFailure) { throw $insertionFailure }
    if ($state.cleanup.Count -ne 0) { throw 'ke-controller:insertion-progress' }
    Check-Time 150000; Assert-StoreState
    $state.phase = 'missing-status'; Run-Pair 'missing-status' $Identity $false 240000
    $state.phase = 'install-crl'; Check-Time 240000; Assert-StoreState
    $state.insertion = 'crl-intent'; Publish-Progress 'before-crl-insertion'
    $insertionFailure = $null
    try { Check-Time 240000; $store.InstallCrl(); $state.insertion = 'crl-returned' }
    catch { $insertionFailure = $_ }
    finally {
        try { Publish-Progress 'after-crl-insertion' }
        catch { $state.cleanup.Add(@{ phase = 'crl-insertion-progress'; failures = @(Closed-Failures $_.Exception) }) }
    }
    if ($null -ne $insertionFailure) { throw $insertionFailure }
    if ($state.cleanup.Count -ne 0) { throw 'ke-controller:insertion-progress' }
    Check-Time 240000; Assert-StoreState
    $state.phase = 'healthy'; Run-Pair 'healthy' $Identity $true 330000
    $state.phase = 'wrong-identity'; Run-Pair 'wrong-identity' ('wrong-' + $Identity) $false 420000
    $facts.status = 'passed_four_cold_process_cases'
} catch {
    $primary = [ordered]@{ phase = $state.phase; failures = @(Closed-Failures $_.Exception) }
} finally {
    # First stop only still-live owned Process objects. Actual completion, not
    # Dispose/Kill/requested cancellation, gates trust restoration.
    $cleanupDeadline = [Math]::Min(480000, $clock.ElapsedMilliseconds + 60000)
    foreach ($child in $state.children) {
        if ($child.Started -and -not $child.Reaped) {
            try {
                if (-not $child.Process.HasExited) {
                    $child.Process.Kill(); $child.Killed = $true; $facts.force_killed = $true
                }
            } catch { $state.cleanup.Add(@{ phase = 'owned-child-stop'; failures = @(Closed-Failures $_.Exception) }) }
        }
    }
    while ($clock.ElapsedMilliseconds -lt $cleanupDeadline) {
        try { Poll-Children } catch {
            # At most one row per fixed owner; repeated poll failure cannot
            # grow the final receipt or suppress the recovery-custody record.
            foreach ($closed in @(Closed-Failures $_.Exception)) {
                $owner = $closed.owner
                if (-not $state.poll_failures.ContainsKey($owner)) {
                    $entry = @{ phase = 'capture-or-reap'; owner = $owner;
                        occurrences = 0; first = $closed; last = $closed }
                    $state.poll_failures[$owner] = $entry; $state.cleanup.Add($entry)
                }
                $state.poll_failures[$owner].occurrences++
                $state.poll_failures[$owner].last = $closed
            }
        }
        $live = @($state.children | Where-Object { $_.LaunchState -ceq 'unknown' -or ($_.Started -and -not $_.Reaped) })
        $pending = @($state.captures | Where-Object { -not $_.Closed -and -not $_.Eof -and -not $_.Broken })
        if ($live.Count -eq 0 -and $pending.Count -eq 0) { break }
        Start-Sleep -Milliseconds 20
    }
    foreach ($capture in $state.captures) {
        if (-not $capture.Closed) {
            try { $capture.Stream.Dispose() } catch { $state.cleanup.Add(@{ phase = 'capture-stream-close' }) }
            if ($null -ne $capture.Pending) {
                $wait = $null
                try {
                    $wait = $capture.Pending.AsyncWaitHandle
                    $remaining = [Math]::Max(0, [Math]::Min(200, $cleanupDeadline - $clock.ElapsedMilliseconds))
                    if ($wait.WaitOne([int]$remaining)) { $null = $capture.Stream.EndRead($capture.Pending); $capture.Pending = $null }
                    else { $state.cleanup.Add(@{ phase = 'capture-read-unreaped' }) }
                } catch { $state.cleanup.Add(@{ phase = 'capture-read-close'; failures = @(Closed-Failures $_.Exception) }) }
                finally { if ($null -ne $wait) { $wait.Close() } }
            }
            try { $capture.Writer.Dispose() } catch { $state.cleanup.Add(@{ phase = 'capture-writer-close' }) }
            if (-not $capture.Eof) { $state.cleanup.Add(@{ phase = 'capture-no-eof' }) }
            $capture.Closed = $true
        }
    }
    $facts.capture_facts = @()
    foreach ($capture in $state.captures) {
        $captureHash = $null
        try {
            # Do not decode/export unexpected bytes. Hash only the bounded,
            # closed private capture leaf before owned scratch is removed.
            if (([IO.File]::GetAttributes($capture.Path) -band
                ([IO.FileAttributes]::Directory -bor [IO.FileAttributes]::ReparsePoint)) -ne 0) {
                throw 'ke-controller:capture-kind'
            }
            $reader = [IO.FileStream]::new($capture.Path, [IO.FileMode]::Open,
                [IO.FileAccess]::Read, [IO.FileShare]::Read)
            try {
                if ($reader.Length -gt 4096) { throw 'ke-controller:capture-bound' }
                $captureHash = Digest $reader
            } finally { $reader.Dispose() }
        } catch { $state.cleanup.Add(@{ phase = 'capture-identity'; failures = @(Closed-Failures $_.Exception) }) }
        $facts.capture_facts += @{ role = $capture.Role; file = [IO.Path]::GetFileName($capture.Path);
            case = [IO.Path]::GetFileName([IO.Path]::GetDirectoryName($capture.Path));
            bytes = $capture.Bytes; sha256 = $captureHash; eof = $capture.Eof; broken = $capture.Broken }
    }
    $unreaped = @($state.children | Where-Object { $_.LaunchState -ceq 'unknown' -or ($_.Started -and -not $_.Reaped) })
    if ($null -ne $store) {
        if ($unreaped.Count -ne 0) {
            $facts.restoration = 'restoration_deferred_live_child'
        } else {
            # Attempt both independently, including insertion-before-Close errors.
            foreach ($operation in @('crl', 'root')) {
                try {
                    Check-Time $cleanupDeadline
                    if ($operation -ceq 'crl') { if ($store.CrlOwned) { $store.RemoveCrl() } }
                    else { if ($store.RootOwned) { $store.RemoveRoot() } }
                } catch { $state.cleanup.Add(@{ phase = 'restore-' + $operation; failures = @(Closed-Failures $_.Exception) }) }
            }
            try {
                Check-Time $cleanupDeadline
                $after = @($store.Snapshot()); $store.AssertAbsent()
                $facts.physical_store_after = $after
                if ($store.RootOwned -or $store.CrlOwned -or $after.Count -ne $state.baseline.Count -or
                    [string]::Join("`n", $after) -cne [string]::Join("`n", $state.baseline)) { throw 'ke-controller:restoration-drift' }
                $facts.restoration = 'baseline-restored'
            } catch {
                $facts.restoration = 'incomplete'
                $state.cleanup.Add(@{ phase = 'restore-readback'; failures = @(Closed-Failures $_.Exception) })
            }
        }
        $facts.root_owned = $store.RootOwned; $facts.crl_owned = $store.CrlOwned
        if ($null -ne $state.baseline_hash) {
            try { Publish-Progress $facts.restoration }
            catch { $state.cleanup.Add(@{ phase = 'restoration-progress'; failures = @(Closed-Failures $_.Exception) }) }
        }
    }
    try { Check-Pins } catch { $state.cleanup.Add(@{ phase = 'input-bookend'; failures = @(Closed-Failures $_.Exception) }) }
    foreach ($item in $state.holds) { $item.Stream.Dispose() }
    foreach ($child in $state.children) {
        if ($child.LaunchState -cne 'unknown' -and ($child.Reaped -or -not $child.Started)) { $child.Process.Dispose() }
    }
    $env:TEMP = $oldTemp; $env:TMP = $oldTmp
    # Only a fresh owned scratch tree, finite lazy inventory before any removal.
    # Unknown/live child means retain the entire tree for exact recovery custody.
    if ($state.scratch_created -and $unreaped.Count -eq 0) {
        try {
            Check-Time $cleanupDeadline
            $entries = [Collections.Generic.List[string]]::new()
            $queue = [Collections.Generic.Queue[object]]::new(); $queue.Enqueue(@($scratch, 0))
            [long]$total = 0
            while ($queue.Count -ne 0) {
                $entry = $queue.Dequeue(); $directory = [string]$entry[0]; $depth = [int]$entry[1]
                if ($depth -gt 4 -or ([IO.File]::GetAttributes($directory) -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'ke-controller:scratch-kind' }
                foreach ($path in [IO.Directory]::EnumerateFileSystemEntries($directory)) {
                    Check-Time $cleanupDeadline
                    if ($entries.Count -ge 128) { throw 'ke-controller:scratch-count' }
                    $attr = [IO.File]::GetAttributes($path)
                    if (($attr -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'ke-controller:scratch-kind' }
                    $entries.Add($path)
                    if (($attr -band [IO.FileAttributes]::Directory) -ne 0) { $queue.Enqueue(@($path, $depth + 1)) }
                    else { $total += [IO.FileInfo]::new($path).Length; if ($total -gt 8388608) { throw 'ke-controller:scratch-bytes' } }
                }
            }
            for ($index = $entries.Count - 1; $index -ge 0; $index--) {
                Check-Time $cleanupDeadline
                $path = $entries[$index]
                if (([IO.File]::GetAttributes($path) -band [IO.FileAttributes]::Directory) -ne 0) { [IO.Directory]::Delete($path, $false) }
                else { [IO.File]::Delete($path) }
            }
            [IO.Directory]::Delete($scratch, $false); $facts.scratch = 'removed-owned-bounded-tree'
        } catch { $facts.scratch = 'retained-cleanup-gap'; $state.cleanup.Add(@{ phase = 'owned-scratch'; failures = @(Closed-Failures $_.Exception) }) }
    } elseif ($state.scratch_created) { $facts.scratch = 'retained-live-child-recovery' }
}
$facts.primary = $primary
$facts.children = @(Child-Facts)
$facts.cleanup = @($state.cleanup.ToArray())
$facts.baseline_file = if ($null -ne $state.baseline_hash) { 'healthy-ke-baseline.json' } else { $null }
$facts.baseline_sha256 = $state.baseline_hash
$facts.last_insertion = $state.insertion
$facts.elapsed_ms = $clock.ElapsedMilliseconds
if ($null -ne $primary -or $state.cleanup.Count -ne 0 -or $facts.force_killed -or
    $facts.restoration -cne 'baseline-restored' -or $facts.elapsed_ms -ge 480000 -or $facts.cases.Count -ne 4) {
    $facts.status = 'incomplete'
}
if ($state.output_admitted) { $null = Write-NewJson 'healthy-ke.json' $facts 1048576 }
# Receipt publication is a blocking operation too. A late record cannot turn
# the actual invocation into exit-zero completion; consumers must join both.
if ($facts.status -cne 'passed_four_cold_process_cases' -or $clock.ElapsedMilliseconds -ge 480000) { exit 1 }
exit 0
