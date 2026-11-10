# GUEST ONLY: invoke through the owned Sandbox's ExistingLogin session.
# Fixed Windows build 26100 / owner-configured SAC-Off laboratory scope only.
# This is not provider, trust, erasure, production or release qualification.
# REQ-8/BND-2, AC-04, CHECK-6/8/9/17/21/25/26/27; no PKI or host changes.
param(
    [Parameter(Mandatory = $true)][string]$RunId,
    [Parameter(Mandatory = $true)][string]$ClientPath,
    [Parameter(Mandatory = $true)][string]$ClientSha256,
    [Parameter(Mandatory = $true)][string]$OutputDirectory,
    [ValidateSet('Malformed', 'Eof')][string]$Mode = 'Malformed'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$Mode = if ($Mode -ieq 'Malformed') { 'Malformed' } else { 'Eof' }
$clock = [System.Diagnostics.Stopwatch]::StartNew()
$phase = 'admission'
$listener = $null
$accepted = $null
$network = $null
$process = $null
$image = $null
$captures = @()
$cleanup = New-Object 'System.Collections.Generic.List[string]'
$forcedKill = $false
$childStarted = $false
$childExited = $false
$exitCode = $null
$child = [pscustomobject]@{ Process = $null; LaunchState = 'not-attempted';
    Started = $false; Exited = $false; PidValue = $null; StartTicks = $null;
    ExitCode = $null; Killed = $false; Disposed = $false }
$failure = $null
$recordBytes = 0
$recordVersion = $null
$fullRecord = $false
$record = $null
$injectionComplete = $false
$labels = $null
$outputAdmitted = $false
$fileFacts = @()
$build = $null
$donePath = $null
$temporaryDone = $null

# Create only a new fixed output leaf; at most 4096 captured bytes per stream.
# A tracked BeginRead owns one 512-byte buffer, never a Task or line accumulator.
function New-Capture([System.IO.Stream]$Stream, [string]$Path) {
    $writer = [System.IO.FileStream]::new($Path, [System.IO.FileMode]::CreateNew,
        [System.IO.FileAccess]::Write, [System.IO.FileShare]::None)
    try {
        $buffer = New-Object byte[] 512
        $pending = $Stream.BeginRead($buffer, 0, $buffer.Length, $null, $null)
        return [pscustomobject]@{ Stream = $Stream; Writer = $writer; Buffer = $buffer;
            Pending = $pending; Bytes = 0; Eof = $false; Path = $Path }
    } catch {
        $writer.Dispose()
        throw
    }
}

# Drain at most eight completed reads per call; refuse before exceeding 4096.
# Pending reads remain owned by their pipe and are reaped/closed during finally.
function Pump-Capture($Capture) {
    for ($attempt = 0; $attempt -lt 8 -and -not $Capture.Eof; $attempt++) {
        if (-not $Capture.Pending.IsCompleted) { return }
        $count = $Capture.Stream.EndRead($Capture.Pending)
        $Capture.Pending = $null
        if ($count -eq 0) { $Capture.Eof = $true; return }
        if ($Capture.Bytes + $count -gt 4096) { throw 'capture-limit' }
        $Capture.Writer.Write($Capture.Buffer, 0, $count)
        $Capture.Bytes += $count
        $Capture.Pending = $Capture.Stream.BeginRead(
            $Capture.Buffer, 0, $Capture.Buffer.Length, $null, $null)
    }
}

# Poll owned pipes and the controller's original clock; never renew its budget.
function Check-Controller {
    if ($clock.ElapsedMilliseconds -ge 55000) { throw 'controller-limit' }
    foreach ($capture in $captures) { Pump-Capture $capture }
}

# Receive exactly Count bytes into the private <=4096-byte record buffer.
# All receives share one supplied monotonic five-second deadline, including polls.
function Receive-Exact([byte[]]$Buffer, [int]$Offset, [int]$Count, [long]$Deadline) {
    $received = 0
    while ($received -lt $Count) {
        Check-Controller
        $remaining = $Deadline - $clock.ElapsedMilliseconds
        if ($remaining -le 0) { throw 'record-limit' }
        $accepted.ReceiveTimeout = [int][Math]::Min(200, $remaining)
        try {
            $countRead = $accepted.Receive($Buffer, $Offset + $received,
                $Count - $received, [System.Net.Sockets.SocketFlags]::None)
        } catch {
            # PowerShell method calls can wrap the actual SocketException.
            $cause = $_.Exception.GetBaseException()
            if ($cause -is [System.Net.Sockets.SocketException] -and
                $cause.SocketErrorCode -eq [System.Net.Sockets.SocketError]::TimedOut) {
                continue
            }
            throw
        }
        Check-Controller
        if ($clock.ElapsedMilliseconds -ge $Deadline) { throw 'record-limit' }
        if ($countRead -eq 0) { throw 'record-eof' }
        $received += $countRead
    }
}

# Reopen only an owned closed capture leaf, bound at 4096 bytes before hashing.
# Return byte/hash facts and private decoded text; callers never export that text.
function Read-Capture([string]$Path) {
    $attributes = [System.IO.File]::GetAttributes($Path)
    if (($attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'capture-link'
    }
    $stream = [System.IO.FileStream]::new($Path, [System.IO.FileMode]::Open,
        [System.IO.FileAccess]::Read, [System.IO.FileShare]::Read)
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try {
        if ($stream.Length -gt 4096) { throw 'capture-limit' }
        $bytes = New-Object byte[] ([int]$stream.Length)
        $offset = 0
        while ($offset -lt $bytes.Length) {
            $count = $stream.Read($bytes, $offset, $bytes.Length - $offset)
            if ($count -eq 0) { throw 'capture-short' }
            $offset += $count
        }
        if ($stream.ReadByte() -ne -1) { throw 'capture-growth' }
        $digest = ([BitConverter]::ToString($sha.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant()
        foreach ($value in $bytes) { if ($value -gt 127) { throw 'capture-non-ascii' } }
        return [pscustomobject]@{ Name = [System.IO.Path]::GetFileName($Path);
            Bytes = $bytes.Length; Sha256 = $digest;
            Text = [System.Text.Encoding]::ASCII.GetString($bytes) }
    } finally { $sha.Dispose(); $stream.Dispose() }
}

# .NET Framework creates the native process before allocating redirected readers
# and associating its handle/ID. A throwing Start is not proof of no child.
# Reuse the healthy controller's unknown-before-call and same-object observations;
# never discover/kill a process by a loose PID, image name or broad enumeration.
function Start-OwnedChild($Child) {
    $Child.LaunchState = 'unknown'
    try {
        if (-not $Child.Process.Start()) {
            $Child.LaunchState = 'not-started-confirmed'
            throw 'launch-refused'
        }
        $Child.Started = $true
        $Child.PidValue = $Child.Process.Id
        $Child.StartTicks = $Child.Process.StartTime.ToUniversalTime().Ticks
        $Child.LaunchState = 'started'
    } catch {
        $launchFailure = $_
        if ($Child.LaunchState -ceq 'unknown') {
            try { $Child.PidValue = $Child.Process.Id; $Child.Started = $true } catch { }
            try { $Child.StartTicks = $Child.Process.StartTime.ToUniversalTime().Ticks } catch { }
        }
        throw $launchFailure
    }
}

# Observe/reap only the same attributable Process object. Even if later actual
# completion is observed, a partial Start stays an incomplete launch outcome.
# Missing identity/handle retains recovery custody rather than claiming cleanup.
function Stop-OwnedChild($Child, [Collections.Generic.List[string]]$Cleanup) {
    if ($Child.LaunchState -ceq 'not-attempted' -or $Child.LaunchState -ceq 'not-started-confirmed') { return }
    if ($Child.LaunchState -ceq 'unknown') { $Cleanup.Add('child-launch-indeterminate') }
    if ($Child.Exited) { return }
    try {
        $observedPid = $Child.Process.Id
        $observedStart = $Child.Process.StartTime.ToUniversalTime().Ticks
        if ($observedPid -lt 1 -or $observedStart -lt 1 -or
            ($null -ne $Child.PidValue -and $Child.PidValue -ne $observedPid) -or
            ($null -ne $Child.StartTicks -and $Child.StartTicks -ne $observedStart)) { throw 'child-identity' }
        $Child.PidValue = $observedPid; $Child.StartTicks = $observedStart; $Child.Started = $true
        if (-not $Child.Process.HasExited) {
            $Child.Killed = $true
            $Child.Process.Kill()
        }
        if (-not $Child.Process.WaitForExit(2000)) { $Cleanup.Add('child-not-reaped'); return }
        if (-not $Child.Process.HasExited) { $Cleanup.Add('child-reap-state'); return }
        $Child.ExitCode = $Child.Process.ExitCode; $Child.Exited = $true
    } catch { $Cleanup.Add('child-cleanup') }
}

try {
    if ($RunId.Length -gt 48 -or $RunId -cnotmatch '\A[A-Za-z0-9_-]{1,48}\z' -or
        $ClientSha256.Length -ne 64 -or $ClientSha256 -cnotmatch '\A[0-9A-Fa-f]{64}\z') {
        throw 'arguments'
    }
    $inputRoot = 'C:\NevroTickKeInput_' + $RunId
    $expectedClient = $inputRoot + '\nevrotick-nts-ke-probe.exe'
    $expectedOutput = 'C:\NevroTickKeOutput_' + $RunId
    if (-not [string]::Equals($ClientPath, $expectedClient, [StringComparison]::OrdinalIgnoreCase) -or
        -not [string]::Equals($OutputDirectory, $expectedOutput, [StringComparison]::OrdinalIgnoreCase) -or
        [System.IO.Path]::GetFullPath($ClientPath) -cne $ClientPath -or
        [System.IO.Path]::GetFullPath($OutputDirectory) -cne $OutputDirectory) { throw 'paths' }
    # Account/build checks are fail-closed lab guards, not Sandbox authentication
    # or proof of a standard-user token. The controller owns the explicit WSB ID.
    if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or
        [Environment]::UserName -cne 'WDAGUtilityAccount') { throw 'guest-context' }
    $build = [Environment]::OSVersion.Version.Build
    if ($build -ne 26100) { throw 'guest-build' }
    foreach ($root in @($inputRoot, $OutputDirectory)) {
        if (-not [System.IO.Directory]::Exists($root)) { throw 'mapping-missing' }
    }
    # The two exact WSB transport mapping roots may be reparse boundaries.
    # Do not follow a substituted executable leaf or overwrite any output leaf.
    $attributes = [System.IO.File]::GetAttributes($ClientPath)
    if (($attributes -band ([System.IO.FileAttributes]::ReparsePoint -bor
        [System.IO.FileAttributes]::Directory)) -ne 0) { throw 'image-link' }
    $stem = $Mode.ToLowerInvariant()
    $stdoutPath = $OutputDirectory + '\' + $stem + '.stdout.log'
    $stderrPath = $OutputDirectory + '\' + $stem + '.stderr.log'
    $donePath = $OutputDirectory + '\' + $stem + '.done.json'
    $temporaryDone = $OutputDirectory + '\' + $stem + '.done.tmp'
    foreach ($path in @($stdoutPath, $stderrPath, $donePath, $temporaryDone)) {
        if ([System.IO.File]::Exists($path) -or [System.IO.Directory]::Exists($path)) {
            throw 'output-exists'
        }
    }
    $outputAdmitted = $true
    # Keep this deny-write/delete image handle alive through the actual child exit.
    # Hash at most64MiB; filename/mtime alone never admits execution.
    $image = [System.IO.FileStream]::new($ClientPath, [System.IO.FileMode]::Open,
        [System.IO.FileAccess]::Read, [System.IO.FileShare]::Read)
    if ($image.Length -lt 1 -or $image.Length -gt 67108864) { throw 'image-size' }
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try { $actualHash = ([BitConverter]::ToString($sha.ComputeHash($image))).Replace('-', '').ToLowerInvariant() }
    finally { $sha.Dispose() }
    if ($actualHash -cne $ClientSha256.ToLowerInvariant() -or
        ([System.IO.File]::GetAttributes($ClientPath) -band
            [System.IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'image-identity' }
    Check-Controller

    $phase = 'listen'
    $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
    $listener.Server.ExclusiveAddressUse = $true
    $listener.Start(1)
    $port = ([System.Net.IPEndPoint]$listener.LocalEndpoint).Port
    if ($port -lt 1 -or $port -gt 65535) { throw 'listener-port' }
    $phase = 'launch'
    $process = [System.Diagnostics.Process]::new()
    $process.StartInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $process.StartInfo.FileName = $ClientPath
    $process.StartInfo.Arguments = '127.0.0.1:' + $port + ' localhost'
    $process.StartInfo.WorkingDirectory = $OutputDirectory
    $process.StartInfo.UseShellExecute = $false
    $process.StartInfo.CreateNoWindow = $true
    $process.StartInfo.RedirectStandardOutput = $true
    $process.StartInfo.RedirectStandardError = $true
    $child.Process = $process
    Start-OwnedChild $child
    $childStarted = $child.Started
    $childDeadline = $clock.ElapsedMilliseconds + 25000
    $captures += New-Capture $process.StandardOutput.BaseStream $stdoutPath
    $captures += New-Capture $process.StandardError.BaseStream $stderrPath

    $phase = 'accept'
    $acceptDeadline = $clock.ElapsedMilliseconds + 7500
    while (-not $listener.Pending()) {
        Check-Controller
        if ($process.HasExited -or $clock.ElapsedMilliseconds -ge $acceptDeadline) { throw 'accept-refused' }
        [System.Threading.Thread]::Sleep(10)
    }
    Check-Controller
    if ($clock.ElapsedMilliseconds -ge $acceptDeadline) { throw 'accept-refused' }
    $accepted = $listener.AcceptSocket()
    Check-Controller
    if ($clock.ElapsedMilliseconds -ge $acceptDeadline) { throw 'accept-refused' }
    if (-not $accepted.RemoteEndPoint.Address.Equals([System.Net.IPAddress]::Loopback)) {
        throw 'peer-refused'
    }
    $network = [System.Net.Sockets.NetworkStream]::new($accepted, $false)
    $phase = 'client-hello'
    $recordDeadline = $clock.ElapsedMilliseconds + 5000
    $record = New-Object byte[] 4096
    Receive-Exact $record 0 5 $recordDeadline
    if ($clock.ElapsedMilliseconds -ge $recordDeadline) { throw 'record-limit' }
    $payloadLength = ([int]$record[3] * 256) + [int]$record[4]
    if ($record[0] -ne 22 -or $record[1] -ne 3 -or
        $record[2] -notin @(1, 3) -or $payloadLength -lt 4 -or $payloadLength -gt 4091) {
        throw 'record-shape'
    }
    Receive-Exact $record 5 $payloadLength $recordDeadline
    if ($clock.ElapsedMilliseconds -ge $recordDeadline) { throw 'record-limit' }
    $handshakeLength = ([int]$record[6] * 65536) + ([int]$record[7] * 256) + [int]$record[8]
    if ($record[5] -ne 1 -or $handshakeLength -ne $payloadLength - 4) { throw 'hello-shape' }
    if ($clock.ElapsedMilliseconds -ge $recordDeadline) { throw 'record-limit' }
    $recordBytes = 5 + $payloadLength
    $recordVersion = '3.' + $record[2]
    $fullRecord = $true
    [Array]::Clear($record, 0, $record.Length)

    $phase = 'inject'
    if ($Mode -ceq 'Malformed') {
        # Literal unknown handshake type255/length0; no certificate or TLS server.
        [byte[]]$malformed = @(22, 3, 3, 0, 4, 255, 0, 0, 0)
        $network.WriteTimeout = 1000
        $network.Write($malformed, 0, $malformed.Length)
        $network.Flush()
    } else {
        # Orderly send-half EOF is the fixed neighboring refusal, not TLS success.
        $accepted.Shutdown([System.Net.Sockets.SocketShutdown]::Send)
    }
    $injectionComplete = $true
    $phase = 'child-exit'
    # Retain the receive side/socket until actual exit; do not mask delivery by RST.
    while (-not $process.HasExited) {
        Check-Controller
        if ($clock.ElapsedMilliseconds -ge $childDeadline) { throw 'child-limit' }
        [System.Threading.Thread]::Sleep(10)
    }
    Check-Controller
    if ($clock.ElapsedMilliseconds -ge $childDeadline) { throw 'child-limit' }
    if (-not $process.WaitForExit(0)) { throw 'child-reap-state' }
    $child.ExitCode = $process.ExitCode; $child.Exited = $true
    $childExited = $child.Exited
    $exitCode = $child.ExitCode
    $phase = 'capture'
    $captureDeadline = $clock.ElapsedMilliseconds + 2000
    while (@($captures | Where-Object { -not $_.Eof }).Count -gt 0) {
        Check-Controller
        if ($clock.ElapsedMilliseconds -ge $captureDeadline) { throw 'capture-limit' }
        [System.Threading.Thread]::Sleep(10)
    }
} catch {
    # Preserve only the closed harness stage, never exception/argument payloads.
    $failure = $phase
} finally {
    # Retain uncertain launch and actual exit separately; Dispose is no reaping.
    Stop-OwnedChild $child $cleanup
    $forcedKill = $child.Killed
    $childStarted = if ($child.LaunchState -ceq 'unknown' -and -not $child.Started) { $null } else { $child.Started }
    $childExited = $child.Exited; $exitCode = $child.ExitCode
    if ($null -ne $listener) { try { $listener.Stop() } catch { $cleanup.Add('listener-close') } }
    if ($null -ne $network) { try { $network.Dispose() } catch { $cleanup.Add('stream-close') } }
    if ($null -ne $accepted) { try { $accepted.Dispose() } catch { $cleanup.Add('socket-close') } }
    foreach ($capture in $captures) {
        try {
            if (-not $capture.Eof) { $cleanup.Add('capture-not-eof') }
            $capture.Stream.Dispose()
            if ($null -ne $capture.Pending) {
                if ($capture.Pending.AsyncWaitHandle.WaitOne(1000)) {
                    try { [void]$capture.Stream.EndRead($capture.Pending) } catch { }
                } else { $cleanup.Add('capture-not-reaped') }
                $capture.Pending.AsyncWaitHandle.Close()
            }
        } catch { $cleanup.Add('capture-close') }
        try { $capture.Writer.Dispose() } catch { $cleanup.Add('capture-file-close') }
    }
    if ($null -ne $process -and $child.LaunchState -cne 'unknown' -and
        ($child.Exited -or -not $child.Started)) {
        try { $process.Dispose(); $child.Disposed = $true } catch { $cleanup.Add('process-close') }
    }
    if ($null -ne $image -and ($child.Exited -or $child.LaunchState -cin @('not-attempted', 'not-started-confirmed'))) {
        try { $image.Dispose() } catch { $cleanup.Add('image-close') }
    }
    if ($null -ne $record) { [Array]::Clear($record, 0, $record.Length) }
}

if (-not $outputAdmitted) { exit 1 }
try {
    # Capture files are closed before hashing/parsing and before atomic completion.
    $stdout = Read-Capture $stdoutPath
    $stderr = Read-Capture $stderrPath
    $fileFacts = @($stdout, $stderr | ForEach-Object {
        [ordered]@{ name = $_.Name; bytes = $_.Bytes; sha256 = $_.Sha256 }
    })
    $pattern = '\Astage=([a-z-]{1,32}) cause=([a-z-]{1,32}) cleanup_stage=([a-z-]{1,32}) cleanup_cause=([a-z-]{1,32}) admission_stage=([a-z-]{1,32}) admission_cause=([a-z-]{1,32})\r?\n\z'
    $match = [regex]::Match($stderr.Text, $pattern, [System.Text.RegularExpressions.RegexOptions]::CultureInvariant)
    if (-not $match.Success) { throw 'labels' }
    $stages = @('none', 'admission', 'resources', 'currentness', 'request', 'response',
        'request-sealing', 'connect', 'tls', 'handshake', 'exporter', 'shutdown')
    $causes = @('none', 'arguments', 'allocation', 'deadline', 'cancelled', 'control',
        'parameters', 'timeout', 'interrupted', 'refused', 'unexpected-eof', 'invalid-data',
        'io', 'framing', 'negotiation', 'cookie-budget', 'entropy', 'encoding', 'endpoint',
        'peer-mismatch', 'setup', 'profile', 'export')
    for ($index = 1; $index -le 6; $index++) {
        $allowed = if ($index % 2 -eq 1) { $stages } else { $causes }
        if ($match.Groups[$index].Value -cnotin $allowed) { throw 'labels' }
    }
    $labels = [ordered]@{ stage = $match.Groups[1].Value; cause = $match.Groups[2].Value;
        cleanup_stage = $match.Groups[3].Value; cleanup_cause = $match.Groups[4].Value;
        admission_stage = $match.Groups[5].Value; admission_cause = $match.Groups[6].Value }
    $expectedCause = if ($Mode -ceq 'Malformed') { 'invalid-data' } else { 'unexpected-eof' }
    if ($stdout.Bytes -ne 0 -or $exitCode -ne 1 -or $labels.stage -cne 'handshake' -or
        $labels.cause -cne $expectedCause) { throw 'expectation' }
} catch { if ($null -eq $failure) { $failure = 'observation' } }

try {
    # Fixed schema, no raw TLS/Hello/randoms/certificates/keys/environment/arguments.
    # A <=16KiB new temporary leaf is closed then atomically renamed; never replace.
    $passed = $null -eq $failure -and $cleanup.Count -eq 0 -and -not $forcedKill -and
        $child.LaunchState -ceq 'started' -and $childStarted -and $childExited -and $fullRecord -and $injectionComplete -and
        $clock.ElapsedMilliseconds -lt 60000
    $evidence = [ordered]@{
        schema = 'nevrotick-sandbox-ke-negative-v2'; run_id = $RunId; mode = $Mode;
        status = $(if ($passed) { 'passed' } else { 'incomplete' });
        scope = 'guest-loopback-handshake-refusal'; os_build = $build;
        execution_policy = (Get-ExecutionPolicy).ToString();
        powershell_version = $PSVersionTable.PSVersion.ToString();
        sac = 'owner-configured-off-not-measured'; existing_login = $true;
        standard_user_token_qualified = $false; client_sha256 = $ClientSha256.ToLowerInvariant();
        record_count = $(if ($fullRecord) { 1 } else { 0 }); record_bytes = $recordBytes;
        record_content_type = $(if ($fullRecord) { 22 } else { $null });
        record_legacy_version = $recordVersion;
        handshake_type = $(if ($fullRecord) { 1 } else { $null });
        full_record_drained = $fullRecord; injection_complete = $injectionComplete;
        server_sent_bytes = $(if (-not $injectionComplete) { $null }
            elseif ($Mode -ceq 'Malformed') { 9 } else { 0 });
        child_started = $childStarted; child_exited = $childExited; child_exit_code = $exitCode;
        child_launch_state = $child.LaunchState; child_pid = $child.PidValue;
        child_start_utc_ticks = $child.StartTicks; child_process_disposed = $child.Disposed;
        child_recovery_required = ($child.LaunchState -ceq 'unknown' -or ($child.Started -and -not $child.Exited));
        forced_kill = $forcedKill; harness_failure_stage = $failure;
        cleanup_gaps = @($cleanup); client_labels = $labels; capture_files = $fileFacts;
        elapsed_ms = $clock.ElapsedMilliseconds; controller_budget_seconds = 60;
        provider_qualified = $false; trust_qualified = $false; native_erasure_qualified = $false
    }
    $encoded = [System.Text.UTF8Encoding]::new($false).GetBytes(($evidence | ConvertTo-Json -Depth 5 -Compress))
    if ($encoded.Length -gt 16384) { throw 'evidence-limit' }
    $done = [System.IO.FileStream]::new($temporaryDone, [System.IO.FileMode]::CreateNew,
        [System.IO.FileAccess]::Write, [System.IO.FileShare]::None)
    try { $done.Write($encoded, 0, $encoded.Length); $done.Flush($true) }
    finally { $done.Dispose() }
    [System.IO.File]::Move($temporaryDone, $donePath)
    if ($passed) { exit 0 }
} catch { }
exit 1
