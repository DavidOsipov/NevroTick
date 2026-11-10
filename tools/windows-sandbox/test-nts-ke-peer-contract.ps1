# GUEST ONLY / ExistingLogin: source-contract controls, not a TLS/native run.
# Load only five SHA256-pinned function definitions. Never run fixture preamble,
# start a listener/client, change trust or execute host/control commands.
param(
    [Parameter(Mandatory = $true)][string]$RunId,
    [Parameter(Mandatory = $true)][string]$FixturePath,
    [Parameter(Mandatory = $true)][string]$FixtureSha256,
    [Parameter(Mandatory = $true)][string]$OutputDirectory,
    [ValidateSet('Red', 'Green')][string]$Checkpoint = 'Red'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$clock = [System.Diagnostics.Stopwatch]::StartNew()
$captures = @()
$accepted = $null
$source = $null
$outputAdmitted = $false
$phase = 'admission'
$failure = $null
$cases = @()

try {
    if ($RunId.Length -gt 48 -or $RunId -cnotmatch '\A[A-Za-z0-9_-]{1,48}\z' -or
        $FixtureSha256.Length -ne 64 -or $FixtureSha256 -cnotmatch '\A[0-9A-Fa-f]{64}\z') {
        throw 'arguments'
    }
    $expectedFixture = 'C:\NevroTickKeInput_' + $RunId + '\nts-ke-malformed-peer.ps1'
    $expectedOutput = 'C:\NevroTickKeOutput_' + $RunId
    if (-not [string]::Equals($FixturePath, $expectedFixture, [StringComparison]::OrdinalIgnoreCase) -or
        -not [string]::Equals($OutputDirectory, $expectedOutput, [StringComparison]::OrdinalIgnoreCase) -or
        [System.IO.Path]::GetFullPath($FixturePath) -cne $FixturePath -or
        [System.IO.Path]::GetFullPath($OutputDirectory) -cne $OutputDirectory -or
        -not [System.IO.Directory]::Exists($OutputDirectory)) { throw 'paths' }
    if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or
        [Environment]::UserName -cne 'WDAGUtilityAccount' -or
        [Environment]::OSVersion.Version.Build -ne 26100) { throw 'guest-context' }
    $stem = 'peer-contract-' + $Checkpoint.ToLowerInvariant()
    $temporary = $OutputDirectory + '\' + $stem + '.done.tmp'
    $completion = $OutputDirectory + '\' + $stem + '.done.json'
    foreach ($path in @($temporary, $completion)) {
        if ([System.IO.File]::Exists($path) -or [System.IO.Directory]::Exists($path)) {
            throw 'output-exists'
        }
    }
    $outputAdmitted = $true
    # The exact input/output WSB mapping roots may be reparse boundaries;
    # reject the fixture leaf and hold it deny-write/delete through all controls.
    $attributes = [System.IO.File]::GetAttributes($FixturePath)
    if (($attributes -band ([System.IO.FileAttributes]::ReparsePoint -bor
        [System.IO.FileAttributes]::Directory)) -ne 0) { throw 'source-link' }
    $source = [System.IO.FileStream]::new($FixturePath, [System.IO.FileMode]::Open,
        [System.IO.FileAccess]::Read, [System.IO.FileShare]::Read)
    if ($source.Length -lt 1 -or $source.Length -gt 65536) { throw 'source-size' }
    $bytes = New-Object byte[] ([int]$source.Length)
    $offset = 0
    while ($offset -lt $bytes.Length) {
        $count = $source.Read($bytes, $offset, $bytes.Length - $offset)
        if ($count -eq 0) { throw 'source-short' }
        $offset += $count
    }
    if ($source.ReadByte() -ne -1) { throw 'source-growth' }
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try { $actualHash = ([BitConverter]::ToString($sha.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant() }
    finally { $sha.Dispose() }
    if ($actualHash -cne $FixtureSha256.ToLowerInvariant()) { throw 'source-identity' }
    $phase = 'definitions'
    $text = [System.Text.UTF8Encoding]::new($false, $true).GetString($bytes)
    $tokens = $null
    $parseErrors = $null
    $tree = [System.Management.Automation.Language.Parser]::ParseInput(
        $text, $FixturePath, [ref]$tokens, [ref]$parseErrors)
    if ($parseErrors.Count -ne 0) { throw 'source-syntax' }
    foreach ($name in @('Pump-Capture', 'Check-Controller', 'Receive-Exact', 'Start-OwnedChild', 'Stop-OwnedChild')) {
        $definitions = @($tree.EndBlock.Statements | Where-Object {
            $_ -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $_.Name -ceq $name
        })
        if ($definitions.Count -ne 1) { throw 'source-function' }
        # AST-selected full function extent only: no param block, preamble or main.
        . ([scriptblock]::Create($definitions[0].Extent.Text))
    }

    $phase = 'controls'
    # Healthy immediate completion: one actual Receive call within1000ms.
    $accepted = [pscustomobject]@{ ReceiveTimeout = 0; Calls = 0 }
    $accepted | Add-Member -MemberType ScriptMethod -Name Receive -Value {
        param([byte[]]$Buffer, [int]$Offset, [int]$Count, $Flags)
        $this.Calls++
        $Buffer[$Offset] = 22
        return 1
    }
    $buffer = New-Object byte[] 1
    $start = $clock.ElapsedMilliseconds
    $reason = 'returned'
    $errorType = $null
    try { Receive-Exact $buffer 0 1 ($clock.ElapsedMilliseconds + 1000) }
    catch { $reason = 'other-exception'; $errorType = $_.Exception.GetType().Name }
    $cases += [ordered]@{ name = 'immediate-completion'; expected = 'returned'; actual = $reason;
        passed = ($reason -ceq 'returned' -and $accepted.Calls -eq 1 -and $buffer[0] -eq 22);
        receive_calls = $accepted.Calls; elapsed_ms = $clock.ElapsedMilliseconds - $start;
        exception_type = $errorType }

    # Deadline fault: Receive really runs, sleeps30ms, then returns with only5ms
    # originally available. Success must be refused by a fresh completion check.
    $accepted = [pscustomobject]@{ ReceiveTimeout = 0; Calls = 0 }
    $accepted | Add-Member -MemberType ScriptMethod -Name Receive -Value {
        param([byte[]]$Buffer, [int]$Offset, [int]$Count, $Flags)
        $this.Calls++
        [System.Threading.Thread]::Sleep(30)
        $Buffer[$Offset] = 22
        return 1
    }
    $buffer = New-Object byte[] 1
    $start = $clock.ElapsedMilliseconds
    $reason = 'returned'
    $errorType = $null
    try { Receive-Exact $buffer 0 1 ($clock.ElapsedMilliseconds + 5) }
    catch {
        $reason = if ($_.Exception.Message -ceq 'record-limit') { 'record-limit' } else { 'other-exception' }
        $errorType = $_.Exception.GetType().Name
    }
    $cases += [ordered]@{ name = 'late-completion'; expected = 'record-limit'; actual = $reason;
        passed = ($reason -ceq 'record-limit' -and $accepted.Calls -eq 1 -and $buffer[0] -eq 22);
        receive_calls = $accepted.Calls; elapsed_ms = $clock.ElapsedMilliseconds - $start;
        exception_type = $errorType }

    # Healthy timeout neighbor: a genuine SocketException10060 after one200ms
    # poll must retry, preserve the original1000ms deadline, and admit next data.
    $accepted = [pscustomobject]@{ ReceiveTimeout = 0; Calls = 0; FirstTimeout = 0 }
    $accepted | Add-Member -MemberType ScriptMethod -Name Receive -Value {
        param([byte[]]$Buffer, [int]$Offset, [int]$Count, $Flags)
        $this.Calls++
        if ($this.Calls -eq 1) {
            $this.FirstTimeout = $this.ReceiveTimeout
            [System.Threading.Thread]::Sleep(200)
            throw [System.Net.Sockets.SocketException]::new(10060)
        }
        $Buffer[$Offset] = 22
        return 1
    }
    $buffer = New-Object byte[] 1
    $start = $clock.ElapsedMilliseconds
    $reason = 'returned'
    $errorType = $null
    try { Receive-Exact $buffer 0 1 ($clock.ElapsedMilliseconds + 1000) }
    catch { $reason = 'other-exception'; $errorType = $_.Exception.GetType().Name }
    $cases += [ordered]@{ name = 'timeout-then-completion'; expected = 'returned'; actual = $reason;
        passed = ($reason -ceq 'returned' -and $accepted.Calls -eq 2 -and
            $accepted.FirstTimeout -eq 200 -and $buffer[0] -eq 22);
        receive_calls = $accepted.Calls; first_receive_timeout_ms = $accepted.FirstTimeout;
        elapsed_ms = $clock.ElapsedMilliseconds - $start; exception_type = $errorType }

    # Pure Process-shaped doubles exercise the actual source helpers. No native
    # Process is constructed/launched and no listener or trust operation runs.
    function New-ProcessDouble([string]$Mode) {
        $value = [pscustomobject]@{ Mode = $Mode; StartCalls = 0; KillCalls = 0;
            WaitCalls = 0; LastWait = $null; IsExited = ($Mode -ceq 'healthy'); ExitCode = 0 }
        $value | Add-Member -MemberType ScriptProperty -Name Id -Value {
            if ($this.Mode -ceq 'throw-unavailable') { throw [InvalidOperationException]::new('source-control-no-identity') }
            return 11223
        }
        $value | Add-Member -MemberType ScriptProperty -Name StartTime -Value {
            if ($this.Mode -ceq 'throw-unavailable') { throw [InvalidOperationException]::new('source-control-no-identity') }
            return [DateTime]::new(2026, 11, 10, 0, 0, 0, [DateTimeKind]::Utc)
        }
        $value | Add-Member -MemberType ScriptProperty -Name HasExited -Value { return $this.IsExited }
        $value | Add-Member -MemberType ScriptMethod -Name Start -Value {
            $this.StartCalls++
            if ($this.Mode -ceq 'refused') { return $false }
            if ($this.Mode -cin @('throw-associated', 'throw-unavailable')) {
                throw [InvalidOperationException]::new('source-control-partial-start')
            }
            return $true
        }
        $value | Add-Member -MemberType ScriptMethod -Name Kill -Value {
            $this.KillCalls++; $this.IsExited = $true; $this.ExitCode = 1
        }
        $value | Add-Member -MemberType ScriptMethod -Name WaitForExit -Value {
            param([int]$Milliseconds)
            $this.WaitCalls++; $this.LastWait = $Milliseconds
            return $this.IsExited
        }
        return $value
    }
    foreach ($modeValue in @('healthy', 'refused', 'throw-associated', 'throw-unavailable')) {
        $double = New-ProcessDouble $modeValue
        $child = [pscustomobject]@{ Process = $double; LaunchState = 'not-attempted';
            Started = $false; Exited = $false; PidValue = $null; StartTicks = $null;
            ExitCode = $null; Killed = $false; Disposed = $false }
        $childCleanup = [Collections.Generic.List[string]]::new()
        $reason = 'returned'
        try { Start-OwnedChild $child }
        catch {
            $message = $_.Exception.GetBaseException().Message
            $reason = if ($message -ceq 'source-control-partial-start') { 'source-control-partial-start' }
                elseif ($message -ceq 'launch-refused') { 'launch-refused' } else { 'other-exception' }
        }
        Stop-OwnedChild $child $childCleanup
        $passed = $false
        switch ($modeValue) {
            'healthy' {
                $passed = $reason -ceq 'returned' -and $child.LaunchState -ceq 'started' -and
                    $child.Started -and $child.Exited -and $child.ExitCode -eq 0 -and
                    $double.StartCalls -eq 1 -and $double.KillCalls -eq 0 -and $double.WaitCalls -eq 1 -and
                    $double.LastWait -eq 2000 -and $childCleanup.Count -eq 0
            }
            'refused' {
                $passed = $reason -ceq 'launch-refused' -and $child.LaunchState -ceq 'not-started-confirmed' -and
                    -not $child.Started -and -not $child.Exited -and $null -eq $child.PidValue -and
                    $double.StartCalls -eq 1 -and $double.KillCalls -eq 0 -and $double.WaitCalls -eq 0 -and
                    $childCleanup.Count -eq 0
            }
            'throw-associated' {
                $passed = $reason -ceq 'source-control-partial-start' -and $child.LaunchState -ceq 'unknown' -and
                    $child.Started -and $child.Exited -and $child.ExitCode -eq 1 -and $child.PidValue -eq 11223 -and
                    $child.Killed -and $double.StartCalls -eq 1 -and $double.KillCalls -eq 1 -and $double.WaitCalls -eq 1 -and
                    $double.LastWait -eq 2000 -and $childCleanup.Count -eq 1 -and
                    $childCleanup[0] -ceq 'child-launch-indeterminate'
            }
            'throw-unavailable' {
                $passed = $reason -ceq 'source-control-partial-start' -and $child.LaunchState -ceq 'unknown' -and
                    -not $child.Started -and -not $child.Exited -and $null -eq $child.PidValue -and
                    $null -eq $child.StartTicks -and -not $child.Killed -and
                    $double.StartCalls -eq 1 -and $double.KillCalls -eq 0 -and $double.WaitCalls -eq 0 -and
                    $childCleanup.Count -eq 2 -and $childCleanup[0] -ceq 'child-launch-indeterminate' -and
                    $childCleanup[1] -ceq 'child-cleanup'
            }
        }
        $cases += [ordered]@{ name = 'launch-' + $modeValue; passed = $passed; actual = $reason;
            launch_state = $child.LaunchState; started_observed = $child.Started;
            exit_observed = $child.Exited; start_calls = $double.StartCalls;
            kill_calls = $double.KillCalls; wait_calls = $double.WaitCalls; cleanup = @($childCleanup.ToArray()) }
    }
    if ($clock.ElapsedMilliseconds -ge 5000) { throw 'helper-limit' }
} catch { $failure = $phase }
finally { if ($null -ne $source) { $source.Dispose() } }

if (-not $outputAdmitted) { exit 1 }
try {
    # Seven bounded outcomes only; no exception messages/source text/environment.
    # New <=16KiB temporary leaf closes before same-directory atomic completion.
    $passed = $null -eq $failure -and $cases.Count -eq 7 -and
        @($cases | Where-Object { -not $_.passed }).Count -eq 0
    $evidence = [ordered]@{ schema = 'nevrotick-sandbox-ke-contract-v2'; run_id = $RunId;
        checkpoint = $Checkpoint.ToLowerInvariant(); fixture_sha256 = $FixtureSha256.ToLowerInvariant();
        status = $(if ($passed) { 'passed' } elseif ($null -ne $failure) { 'incomplete' } else { 'failed' });
        harness_failure_stage = $failure; cases = $cases; elapsed_ms = $clock.ElapsedMilliseconds;
        execution_policy = (Get-ExecutionPolicy).ToString();
        powershell_version = $PSVersionTable.PSVersion.ToString();
        loaded_definitions = @('Pump-Capture', 'Check-Controller', 'Receive-Exact', 'Start-OwnedChild', 'Stop-OwnedChild');
        fixture_preamble_executed = $false; native_client_launched = $false; listener_started = $false }
    $encoded = [System.Text.UTF8Encoding]::new($false).GetBytes(($evidence | ConvertTo-Json -Depth 5 -Compress))
    if ($encoded.Length -gt 16384) { throw 'evidence-limit' }
    $writer = [System.IO.FileStream]::new($temporary, [System.IO.FileMode]::CreateNew,
        [System.IO.FileAccess]::Write, [System.IO.FileShare]::None)
    try { $writer.Write($encoded, 0, $encoded.Length); $writer.Flush($true) }
    finally { $writer.Dispose() }
    [System.IO.File]::Move($temporary, $completion)
    if ($passed) { exit 0 }
} catch { }
exit 1
