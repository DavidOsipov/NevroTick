# SPDX-License-Identifier: GPL-2.0-only
Set-StrictMode -Version Latest

Import-Module (Join-Path $PSScriptRoot 'prestart_child.psm1') -Scope Local -DisableNameChecking -ErrorAction Stop

function Assert-PrestartDeploymentTime {
    param([Diagnostics.Stopwatch]$Clock, [int]$TimeoutMilliseconds)

    if (!$Clock.IsRunning -or $Clock.ElapsedMilliseconds -ge $TimeoutMilliseconds) {
        throw [TimeoutException]::new('prestart-deployment-session:deadline')
    }
}

function Read-PrestartDeploymentCommand {
    param([IO.Stream]$Stream, [byte[]]$Buffer, [Diagnostics.Stopwatch]$Clock, [int]$TimeoutMilliseconds)

    Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
    # One outstanding read. The containing process owner settles a blocked pipe
    # after timeout; this run never starts another read in that condition.
    [Threading.Tasks.Task[int]]$pending = $Stream.ReadAsync($Buffer, 0, 1)
    while (!$pending.IsCompleted) {
        Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
        Start-Sleep -Milliseconds 5
    }
    Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
    if ($pending.GetAwaiter().GetResult() -eq 0) { return -1 }
    return [int]$Buffer[0]
}

function Write-PrestartDeploymentReply {
    param([IO.Stream]$Stream, [hashtable]$Fields)

    [byte[]]$data = [Text.Encoding]::UTF8.GetBytes((ConvertTo-Json -InputObject $Fields -Compress -ErrorAction Stop))
    if ($data.Length -lt 1 -or $data.Length -gt 24000) {
        throw [IO.InvalidDataException]::new('prestart-deployment-session:reply-extent')
    }
    # The native launcher runs on Windows x64, where the integer is little-endian.
    [byte[]]$length = [BitConverter]::GetBytes([uint32]$data.Length)
    try {
        $Stream.Write($length, 0, $length.Length)
        $Stream.Write($data, 0, $data.Length)
        $Stream.Flush()
    } catch {
        throw [IO.IOException]::new('prestart-deployment-session:reply-transport')
    }
}

function Set-PrestartDeploymentCapture {
    param([hashtable]$Reply, [object]$Capture)

    if ($Capture.ProcessId -isnot [int] -or $Capture.ProcessId -le 0 -or $Capture.ExitCode -isnot [int] -or
        $Capture.InputSha256Before -isnot [string] -or $Capture.InputSha256Before -cnotmatch '\A[0-9a-f]{64}\z' -or
        $Capture.InputSha256After -isnot [string] -or $Capture.InputSha256After -cnotmatch '\A[0-9a-f]{64}\z' -or
        $Capture.Stdout -isnot [byte[]] -or $Capture.Stdout.Length -gt 8192 -or
        $Capture.Stderr -isnot [byte[]] -or $Capture.Stderr.Length -gt 8192) {
        throw [IO.InvalidDataException]::new('prestart-deployment-session:capture')
    }

    $Reply['pid'] = $Capture.ProcessId
    $Reply['before'] = $Capture.InputSha256Before
    $Reply['after'] = $Capture.InputSha256After
    $Reply['exit'] = $Capture.ExitCode
    $Reply['stdout'] = [Convert]::ToBase64String($Capture.Stdout)
    $Reply['stderr'] = [Convert]::ToBase64String($Capture.Stderr)
}

function Get-PrestartDeploymentExceptionChain {
    param([Exception]$Failure)

    while ($null -ne $Failure) {
        $Failure
        if ($Failure -isnot [Management.Automation.MethodInvocationException] -and
            $Failure -isnot [Reflection.TargetInvocationException] -and
            $Failure.GetType() -ne [Management.Automation.RuntimeException]) { break }
        $Failure = $Failure.InnerException
    }
}

function Get-PrestartDeploymentRefusalCode {
    param([string]$Operation, [Exception]$Failure)

    $chain = @(Get-PrestartDeploymentExceptionChain $Failure)
    [Exception]$cause = $chain[-1]
    if ($cause -is [TimeoutException]) { return $Operation + '-deadline' }
    if ($cause -is [ObjectDisposedException]) { return $Operation + '-disposed' }

    # These are the fixed SCM refusal kinds, independent of exception messages.
    [string]$kind = $cause.Data['Kind']
    if ($kind -cin @('attempt-spent', 'identity-incomplete', 'captures-incomplete', 'retired',
            'invalid-clock', 'deadline', 'null-handle', 'deployment-actor', 'not-disabled', 'type-changed',
            'not-stopped', 'service-missing', 'native-failure')) {
        if ($kind -ceq 'native-failure' -or $kind -ceq 'service-missing') {
            $number = $cause.Data['NativeErrorCode']
            if (($number -is [int] -and $number -ge 0) -or $number -is [uint32]) {
                [string]$dword = $number.ToString([Globalization.CultureInfo]::InvariantCulture)
                return $Operation + '-' + $kind + '-' + $dword
            }
        }
        return $Operation + '-' + $kind
    }
    return $Operation + '-failed'
}

function Add-PrestartDeploymentCaptureCleanup {
    param([Exception]$Failure, [Collections.Generic.List[string]]$Cleanup)

    # PowerShell's collector can put the retirement metadata on the invocation
    # wrapper. The native launch or publication owners put it on their exception.
    foreach ($exception in (Get-PrestartDeploymentExceptionChain $Failure)) {
        [Collections.IDictionary]$data = $exception.Data
        # WinTrust uses this fixed diagnostic when its close is the only
        # failure. When verification already failed, the owner attaches close.
        if ($data.Contains('PrestartArchiveTrustCleanupFailure') -or
            ($exception -is [IO.InvalidDataException] -and
                $exception.Message -cmatch '\Aprestart-observer:archive-signature-close-0x[0-9a-f]{8}\z')) {
            if (!$Cleanup.Contains('archive-trust-close-failed')) { $Cleanup.Add('archive-trust-close-failed') }
        }
        if ($data.Contains('PrestartObserverTerminationFailure') -and !$Cleanup.Contains('child-termination-failed')) {
            $Cleanup.Add('child-termination-failed')
        }
        if (($data.Contains('PrestartObserverRetired') -and $true -ne $data['PrestartObserverRetired']) -or
            $data.Contains('PrestartObserverRetirementFailure') -or
            ($data.Contains('PrestartObserverChild') -and $true -ne $data['PrestartObserverRetired'])) {
            if (!$Cleanup.Contains('child-retirement-unconfirmed')) { $Cleanup.Add('child-retirement-unconfirmed') }
        }
        if (($data.Contains('PrestartObserverReleased') -and $true -ne $data['PrestartObserverReleased']) -or
            $data.Contains('PrestartObserverChild') -or $data.Contains('PrestartObserverDisposalFailure') -or
            $data.Contains('PrestartObserverCleanupFailure')) {
            if (!$Cleanup.Contains('child-release-unconfirmed')) { $Cleanup.Add('child-release-unconfirmed') }
        }
        if ($data.Contains('PrestartAccessActorDisposalFailure') -and !$Cleanup.Contains('actor-dispose-failed')) {
            $Cleanup.Add('actor-dispose-failed')
        }
        foreach ($pending in $data['PrestartChildPendingReads']) {
            if ($pending -is [Threading.Tasks.Task] -and !$pending.IsCompleted -and
                !$Cleanup.Contains('child-read-unconfirmed')) {
                $Cleanup.Add('child-read-unconfirmed')
            }
        }
    }
}

function Get-PrestartDeploymentWindowCleanupCode {
    param([Exception]$Failure, [string]$Fallback)

    [string]$operation = $Failure.Data['Operation']
    switch ($operation) {
        'close-broker' { return 'window-close-broker-failed' }
        'close-engine' { return 'window-close-engine-failed' }
        'close-manager' { return 'window-close-manager-failed' }
        'retire' { return 'window-retirement-deadline' }
        default { return $Fallback }
    }
}

function Invoke-PrestartDeploymentSessionCore {
    param(
        [IO.Stream]$InputStream,
        [IO.Stream]$OutputStream,
        [guid]$RunId,
        [Diagnostics.Stopwatch]$Clock,
        [int]$TimeoutMilliseconds,
        [Parameter(Mandatory = $true)][int]$RetirementTimeoutMilliseconds,
        [object]$Window,
        [object]$Archive,
        [object]$Actor,
        [scriptblock]$CaptureIdentity,
        [scriptblock]$CaptureObservation,
        [switch]$FullPrestart
    )

    $ErrorActionPreference = 'Stop'
    [string]$run = $RunId.ToString('D')
    [byte[]]$buffer = [byte[]]::new(1)
    # R can retire either route at any stage. A and U check the original
    # window before the coordinator starts each separately owned access probe.
    [int[]]$sequence = @(83, 67, 73, 67, 79, 70) # SCICOF
    [string[]]$operation = @('sample', 'check', 'identity', 'check', 'observation', 'confirm')
    if ($FullPrestart) {
        $sequence = @(83, 67, 73, 67, 79, 65, 85, 70) # SCICOAUF
        $operation = @('sample', 'check', 'identity', 'check', 'observation',
            'provisioning-check', 'standard-user-check', 'confirm')
    }
    [int]$next = 0
    [bool]$retireRequested = $false
    [string]$protocolRefusal = ''
    [string]$operationRefusal = ''
    [Exception]$firstFailure = $null
    [Exception]$replyFailure = $null
    [Exception]$sessionFailure = $null
    [Collections.Generic.List[string]]$cleanup = [Collections.Generic.List[string]]::new()
    try {
        if ($TimeoutMilliseconds -lt 1 -or $RetirementTimeoutMilliseconds -lt $TimeoutMilliseconds -or
            $RetirementTimeoutMilliseconds -gt 120000) {
            throw [ArgumentOutOfRangeException]::new('RetirementTimeoutMilliseconds')
        }
        if ($null -eq $InputStream) { $InputStream = [Console]::OpenStandardInput() }
        if ($null -eq $OutputStream) { $OutputStream = [Console]::OpenStandardOutput() }

        while ($true) {
            try {
                [int]$command = Read-PrestartDeploymentCommand $InputStream $buffer $Clock $RetirementTimeoutMilliseconds
                if ($command -eq 82) {
                    if ((Read-PrestartDeploymentCommand $InputStream $buffer $Clock $RetirementTimeoutMilliseconds) -ne -1) {
                        $protocolRefusal = 'trailing-command'
                    } else {
                        $retireRequested = $true
                    }
                    break
                }
            } catch {
                $readFailureChain = @(Get-PrestartDeploymentExceptionChain $_.Exception)
                $protocolRefusal = 'request-read-failed'
                if ($readFailureChain[-1] -is [TimeoutException]) { $protocolRefusal = 'request-deadline' }
                break
            }

            if ($command -eq -1) {
                $protocolRefusal = 'unexpected-eof'
                break
            }
            if ($null -ne $firstFailure -or $next -eq $sequence.Length -or $command -ne $sequence[$next]) {
                $protocolRefusal = 'unexpected-command'
                break
            }

            [hashtable]$reply = @{ run = $run; kind = 'current' }
            try {
                Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
                switch ($command) {
                    83 {
                        $sample = $Window.Initial
                        $reply['kind'] = 'sample'
                        $reply['engine'] = @([uint32]$sample.Engine.ConfigurationType, [uint32]$sample.Engine.StatusType)
                        $reply['broker'] = @([uint32]$sample.Broker.ConfigurationType, [uint32]$sample.Broker.StatusType)
                    }
                    67 { [void]$Window.Check() }
                    65 {
                        [void]$Window.Check()
                        $reply['kind'] = 'access-current'
                    }
                    85 {
                        [void]$Window.Check()
                        $reply['kind'] = 'standard-current'
                    }
                    73 {
                        $reply['kind'] = 'identity'
                        Set-PrestartDeploymentCapture $reply (& $CaptureIdentity)
                    }
                    79 {
                        $reply['kind'] = 'observation'
                        Set-PrestartDeploymentCapture $reply (& $CaptureObservation)
                    }
                    70 {
                        [void]$Window.ConfirmDeployment()
                        $reply['kind'] = 'confirmed'
                    }
                }
                Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
            } catch {
                $firstFailure = $_.Exception
                $operationRefusal = Get-PrestartDeploymentRefusalCode $operation[$next] $firstFailure
                $reply = @{ run = $run; kind = 'refused'; code = $operationRefusal }
            }
            try { Write-PrestartDeploymentReply $OutputStream $reply }
            catch {
                $replyFailure = $_.Exception
                break
            }
            $next++
        }
    } catch {
        $sessionFailure = $_.Exception
        $protocolRefusal = 'session-failed'
    } finally {
        # Retire the original window exactly once. It can return the same native
        # failure as the operation above, with additional close failures attached.
        [Exception]$retirement = $null
        [bool]$retirementThrew = $false
        try { $retirement = $Window.Retire() } catch {
            $retirement = $_.Exception
            $retirementThrew = $true
        }
        try { [void]$Archive.Dispose() } catch { $cleanup.Add('archive-dispose-failed') }
        try { [void]$Actor.Dispose() } catch { $cleanup.Add('actor-dispose-failed') }

        Add-PrestartDeploymentCaptureCleanup $firstFailure $cleanup
        if ($null -ne $retirement) {
            Add-PrestartDeploymentCaptureCleanup $retirement $cleanup
            $retirementChain = @(Get-PrestartDeploymentExceptionChain $retirement)
            $firstFailureChain = @(Get-PrestartDeploymentExceptionChain $firstFailure)
            [Exception]$nativeRetirement = $retirementChain[-1]
            [Exception]$nativeFirst = $null
            if ($firstFailureChain.Count -ne 0) { $nativeFirst = $firstFailureChain[-1] }
            if ($retirementThrew -or ![object]::ReferenceEquals($nativeRetirement, $nativeFirst)) {
                $cleanup.Add((Get-PrestartDeploymentWindowCleanupCode $nativeRetirement 'window-retire-failed'))
            }
            foreach ($exception in $retirementChain) {
                foreach ($attached in $exception.Data['RetirementFailures']) {
                    if ($null -ne $attached) {
                        [string]$code = Get-PrestartDeploymentWindowCleanupCode $attached 'window-retirement-failed'
                        if (!$cleanup.Contains($code)) { $cleanup.Add($code) }
                    }
                }
            }
        }
        if ($retireRequested -and (!$Clock.IsRunning -or $Clock.ElapsedMilliseconds -ge $RetirementTimeoutMilliseconds)) {
            $cleanup.Add('retirement-deadline')
        }
        # The retired frame represents bounded wire detail; the containing
        # process owner still confirms EOF, exit and job retirement.
        if ($cleanup.Count -gt 8) {
            $cleanup.RemoveRange(7, $cleanup.Count - 7)
            $cleanup.Add('additional-cleanup-failures')
        }
    }

    if ($null -eq $replyFailure -and $null -eq $sessionFailure) {
        try {
            if ($retireRequested) {
                Write-PrestartDeploymentReply $OutputStream @{ run = $run; kind = 'retired'; cleanup = $cleanup.ToArray() }
                return
            }
            if ($null -eq $firstFailure) {
                Write-PrestartDeploymentReply $OutputStream @{ run = $run; kind = 'refused'; code = $protocolRefusal }
            }
        } catch {
            $replyFailure = $_.Exception
        }
    }

    # A broken or incomplete exchange cannot acknowledge retirement. The fixed
    # owner reports only these bounded codes on its separate diagnostic stream.
    [Collections.Generic.List[string]]$transport = [Collections.Generic.List[string]]::new(2)
    if ($protocolRefusal.Length -ne 0) { $transport.Add($protocolRefusal) }
    if ($null -ne $replyFailure) {
        $replyChain = @(Get-PrestartDeploymentExceptionChain $replyFailure)
        [Exception]$replyCause = $replyChain[-1]
        if ($replyCause -is [IO.IOException] -and $replyCause.Message -ceq 'prestart-deployment-session:reply-transport') {
            $transport.Add('reply-transport')
        } else {
            $transport.Add('reply-failed')
        }
    }
    [Exception]$initiatingFailure = $firstFailure
    if ($null -eq $initiatingFailure) { $initiatingFailure = $sessionFailure }
    if ($null -eq $initiatingFailure) { $initiatingFailure = $replyFailure }
    [InvalidOperationException]$unconfirmed = [InvalidOperationException]::new(
        'prestart-deployment-session:unconfirmed-retirement', $initiatingFailure)
    $unconfirmed.Data['PrestartDeploymentSessionFailure'] = @{
        Operation = $operationRefusal; Transport = $transport.ToArray(); Cleanup = $cleanup.ToArray()
    }
    throw $unconfirmed
}

# The dedicated process hands over its original live window, archive and actor
# as its final action. Its parent owns the other pipe ends and process/job exit.
function Invoke-PrestartDeploymentSession {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory = $true)][NevroTick.PrestartScmWindow]$Window,
        [Parameter(Mandatory = $true)][NevroTick.PrestartActor]$Actor,
        [Parameter(Mandatory = $true)][NevroTick.PrestartObserverChild+ArchiveLease]$Archive,
        [Parameter(Mandatory = $true)][guid]$RunId,
        [Parameter(Mandatory = $true)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory = $true)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds,
        [Parameter(Mandatory = $true)][int]$RetirementTimeoutMilliseconds,
        [Parameter(Mandatory = $true)][string]$PowerShellPath,
        [Parameter(Mandatory = $true)][long]$PowerShellBytes,
        [Parameter(Mandatory = $true)][string]$PowerShellSha256,
        [Parameter(Mandatory = $true)][string]$InputDirectory,
        [Parameter(Mandatory = $true)][long]$EntryBytes,
        [Parameter(Mandatory = $true)][string]$EntrySha256,
        [Parameter(Mandatory = $true)][long]$ModuleBytes,
        [Parameter(Mandatory = $true)][string]$ModuleSha256,
        [Parameter(Mandatory = $true)][long]$HelperBytes,
        [Parameter(Mandatory = $true)][string]$HelperSha256,
        [Parameter(Mandatory = $true)][string]$ExpectedArchiveSha256,
        [Parameter(Mandatory = $true)][string]$ExpectedSignerSha256,
        [Parameter(Mandatory = $true)][string]$ObserverPath,
        [Parameter(Mandatory = $true)][long]$ObserverBytes,
        [Parameter(Mandatory = $true)][string]$ObserverSha256,
        [switch]$FullPrestart
    )

    [hashtable]$identityInputs = @{
        Window = $Window; Archive = $Archive; RunId = $RunId
        PowerShellPath = $PowerShellPath; PowerShellBytes = $PowerShellBytes; PowerShellSha256 = $PowerShellSha256
        InputDirectory = $InputDirectory; EntryBytes = $EntryBytes; EntrySha256 = $EntrySha256
        ModuleBytes = $ModuleBytes; ModuleSha256 = $ModuleSha256
        HelperBytes = $HelperBytes; HelperSha256 = $HelperSha256
        ExpectedArchiveSha256 = $ExpectedArchiveSha256; ExpectedSignerSha256 = $ExpectedSignerSha256
    }
    Invoke-PrestartDeploymentSessionCore -Window $Window -Archive $Archive -Actor $Actor -RunId $RunId `
        -Clock $Clock -TimeoutMilliseconds $TimeoutMilliseconds `
        -RetirementTimeoutMilliseconds $RetirementTimeoutMilliseconds -FullPrestart:$FullPrestart -CaptureIdentity {
            prestart_child\Invoke-PrestartIdentity @identityInputs
        } -CaptureObservation {
            prestart_child\Invoke-PrestartVerifiedScmObservation -Window $Window -ImagePath $ObserverPath `
                -ExpectedBytes $ObserverBytes -ExpectedSha256 $ObserverSha256
        }
}

# The pinned owner entry holds this module and the local prestart_child module.
# Keep helper admission and both capture functions in that same module scope.
function Invoke-PrestartDeploymentOwner {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory)][guid]$RunId,
        [Parameter(Mandatory)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory)][int]$WorkMilliseconds,
        [Parameter(Mandatory)][int]$RetirementMilliseconds,
        [Parameter(Mandatory)][string]$DeploymentSid,
        [Parameter(Mandatory)][string]$SignerSha256,
        [Parameter(Mandatory)][hashtable]$Pins,
        [switch]$FullPrestart
    )

    $ErrorActionPreference = 'Stop'
    [object]$actor = $null
    [object]$archive = $null
    [object]$window = $null
    [string]$stage = 'helper'
    try {
        Assert-PrestartDeploymentTime $Clock $WorkMilliseconds
        prestart_child\Import-PrestartDeploymentHelpers `
            -Path ([IO.Path]::Combine($PSScriptRoot, 'prestart_deployment_helpers.dll')) `
            -ExpectedBytes $Pins.deployment_helper.Bytes -ExpectedSha256 $Pins.deployment_helper.Sha256
        $stage = 'actor'
        $actor = prestart_child\Open-PrestartObserverActor -Clock $Clock -TimeoutMilliseconds $WorkMilliseconds
        $stage = 'archive'
        $archive = prestart_child\Open-PrestartArchive -Clock $Clock -TimeoutMilliseconds $WorkMilliseconds `
            -ArchivePath ([IO.Path]::Combine($PSScriptRoot, 'prestart_fixture.msix')) `
            -ExpectedArchiveBytes $Pins.archive.Bytes -ExpectedArchiveSha256 $Pins.archive.Sha256
        $stage = 'window'
        $window = prestart_child\Open-PrestartScmWindow -Actor $actor -ExpectedDeploymentSid $DeploymentSid `
            -Clock $Clock -TimeoutMilliseconds $WorkMilliseconds

        $stage = 'wire'
        [hashtable]$session = @{
            Window = $window; Actor = $actor; Archive = $archive; RunId = $RunId; Clock = $Clock
            TimeoutMilliseconds = $WorkMilliseconds; RetirementTimeoutMilliseconds = $RetirementMilliseconds
            FullPrestart = $FullPrestart
            PowerShellPath = [IO.Path]::Combine([Environment]::SystemDirectory, 'WindowsPowerShell\v1.0\powershell.exe')
            PowerShellBytes = $Pins.interpreter.Bytes; PowerShellSha256 = $Pins.interpreter.Sha256
            InputDirectory = [IO.Path]::Combine($PSScriptRoot, 'identity')
            EntryBytes = $Pins.identity_entry.Bytes; EntrySha256 = $Pins.identity_entry.Sha256
            ModuleBytes = $Pins.identity_module.Bytes; ModuleSha256 = $Pins.identity_module.Sha256
            HelperBytes = $Pins.identity_helper.Bytes; HelperSha256 = $Pins.identity_helper.Sha256
            ExpectedArchiveSha256 = $Pins.archive.Sha256; ExpectedSignerSha256 = $SignerSha256
            ObserverPath = [IO.Path]::Combine($PSScriptRoot, 'nevrotick-prestart-observer.exe')
            ObserverBytes = $Pins.observer.Bytes; ObserverSha256 = $Pins.observer.Sha256
        }
        Invoke-PrestartDeploymentSession @session
    } catch {
        [Exception]$first = $_.Exception
        [Exception]$refusal = [InvalidOperationException]::new('prestart-owner:' + $stage, $first)
        [Exception[]]$chain = @(Get-PrestartDeploymentExceptionChain $first)
        if ($chain[-1].Data.Contains('PrestartDeploymentSessionFailure')) {
            # The session has already attempted retirement of all originals.
            $refusal.Data['PrestartDeploymentSessionFailure'] = $chain[-1].Data['PrestartDeploymentSessionFailure']
            throw $refusal
        }
        [Collections.Generic.List[Exception]]$windowFailures = [Collections.Generic.List[Exception]]::new()
        # The session normally retires its inputs. These operations are
        # idempotent if it also threw, and cover failure before its finally.
        if ($null -ne $window) {
            try {
                [Exception]$retirement = $window.Retire()
                if ($null -ne $retirement -and ![object]::ReferenceEquals($retirement, $chain[-1])) {
                    $windowFailures.Add($retirement)
                }
            } catch { $windowFailures.Add($_.Exception) }
        }
        foreach ($exception in $chain) {
            foreach ($cleanup in $exception.Data['RetirementFailures']) {
                if ($cleanup -is [Exception]) { $windowFailures.Add($cleanup) }
            }
        }
        if ($windowFailures.Count -ne 0) { $refusal.Data['prestart-owner:window-retire'] = $windowFailures.ToArray() }
        try { if ($null -ne $archive) { $archive.Dispose() } }
        catch { $refusal.Data['prestart-owner:archive-close'] = $_.Exception }
        try { if ($null -ne $actor) { $actor.Dispose() } }
        catch { $refusal.Data['prestart-owner:actor-close'] = $_.Exception }
        throw $refusal
    }
}

function Invoke-PrestartAccessSessionCore {
    param(
        [string]$Role,
        [guid]$RunId,
        [Diagnostics.Stopwatch]$Clock,
        [int]$TimeoutMilliseconds,
        [string]$ExpectedActorSid,
        [IO.Stream]$InputStream,
        [IO.Stream]$OutputStream,
        [scriptblock]$Capture
    )

    $ErrorActionPreference = 'Stop'
    if ($RunId -eq [guid]::Empty -or $Role -cnotin @('access', 'standard') -or
        $ExpectedActorSid.Length -gt 183 -or
        $ExpectedActorSid -cnotmatch '\AS-1-(?:0|[1-9][0-9]{0,9}|0x[0-9A-Fa-f]{12})(?:-(?:0|[1-9][0-9]{0,9})){1,15}\z') {
        throw [ArgumentException]::new('prestart-access-session:profile')
    }
    if ($null -eq $InputStream) { $InputStream = [Console]::OpenStandardInput() }
    if ($null -eq $OutputStream) { $OutputStream = [Console]::OpenStandardOutput() }

    [byte[]]$buffer = [byte[]]::new(1)
    [int]$command = Read-PrestartDeploymentCommand $InputStream $buffer $Clock $TimeoutMilliseconds
    if ($command -eq -1) { return }
    [bool]$provisioning = $Role -ceq 'access'
    [int]$expected = if ($provisioning) { 65 } else { 85 }
    if ($command -ne $expected) {
        throw [IO.InvalidDataException]::new('prestart-access-session:unexpected-command')
    }
    if ((Read-PrestartDeploymentCommand $InputStream $buffer $Clock $TimeoutMilliseconds) -ne -1) {
        throw [IO.InvalidDataException]::new('prestart-access-session:trailing-command')
    }

    [hashtable]$observerInputs = if ($provisioning) {
        @{
            Selector = 'scm_access_probe::windows::native_installed_roles_prestart_access'
            ExpectedProvisioningUserSid = $ExpectedActorSid
        }
    } else {
        @{
            Selector = 'scm_access_probe::windows::native_standard_user_prestart_change_config_refused'
            ExpectedStandardUserSid = $ExpectedActorSid
        }
    }
    [hashtable]$reply = @{ run = $RunId.ToString('D'); kind = $Role }
    try {
        Set-PrestartDeploymentCapture $reply (& $Capture $observerInputs)
        Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
    } catch {
        [Exception]$first = $_.Exception
        [string]$code = Get-PrestartDeploymentRefusalCode $Role $first
        [Collections.Generic.List[string]]$cleanup = [Collections.Generic.List[string]]::new()
        Add-PrestartDeploymentCaptureCleanup $first $cleanup
        try {
            Write-PrestartDeploymentReply $OutputStream @{
                run = $RunId.ToString('D'); kind = 'refused'; code = $code; cleanup = $cleanup.ToArray()
            }
        } catch { $first.Data['PrestartAccessRefusalWriteFailure'] = $_.Exception }
        throw $first
    }
    Write-PrestartDeploymentReply $OutputStream $reply
}

# The enclosing dedicated process owns its independently admitted primary token,
# pinned inputs and native job. It exits after this one request or unused EOF.
function Invoke-PrestartAccessSession {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory)][string]$Role,
        [Parameter(Mandatory)][guid]$RunId,
        [Parameter(Mandatory)][Diagnostics.Stopwatch]$Clock,
        [Parameter(Mandatory)][ValidateRange(1, 120000)][int]$TimeoutMilliseconds,
        [Parameter(Mandatory)][string]$ExpectedActorSid,
        [Parameter(Mandatory)][string]$ObserverPath,
        [Parameter(Mandatory)][ValidateRange(1, 16777216)][long]$ObserverBytes,
        [Parameter(Mandatory)][string]$ObserverSha256,
        [Parameter(Mandatory)][string]$HelperPath,
        [Parameter(Mandatory)][ValidateRange(1, 2097152)][int]$HelperBytes,
        [Parameter(Mandatory)][string]$HelperSha256,
        [IO.Stream]$InputStream,
        [IO.Stream]$OutputStream,
        [ref]$CollectedCapture
    )

    [hashtable]$session = @{
        Role = $Role; RunId = $RunId; Clock = $Clock; TimeoutMilliseconds = $TimeoutMilliseconds
        ExpectedActorSid = $ExpectedActorSid; InputStream = $InputStream; OutputStream = $OutputStream
    }
    Invoke-PrestartAccessSessionCore @session -Capture {
        param([hashtable]$ObserverInputs)

        [object]$actor = $null
        [Exception]$failure = $null
        [object]$capture = $null
        try {
            Assert-PrestartDeploymentTime $Clock $TimeoutMilliseconds
            [hashtable]$helper = @{
                Path = $HelperPath; ExpectedBytes = $HelperBytes; ExpectedSha256 = $HelperSha256
            }
            prestart_child\Import-PrestartDeploymentHelpers @helper
            $actor = prestart_child\Open-PrestartObserverActor -Clock $Clock -TimeoutMilliseconds $TimeoutMilliseconds
            [hashtable]$pinnedImage = @{
                ImagePath = $ObserverPath; ExpectedBytes = $ObserverBytes; ExpectedSha256 = $ObserverSha256
                Actor = $actor; Clock = $Clock; TimeoutMilliseconds = $TimeoutMilliseconds
            }
            if ($null -ne $CollectedCapture) { $pinnedImage.CollectedCapture = $CollectedCapture }
            $capture = prestart_child\Invoke-PrestartObserver @ObserverInputs @pinnedImage
        } catch {
            $failure = $_.Exception
            throw
        } finally {
            if ($null -ne $actor) {
                try { $actor.Dispose() } catch {
                    if ($null -eq $failure) {
                        $_.Exception.Data['PrestartAccessActorDisposalFailure'] = $true
                        throw
                    }
                    $failure.Data['PrestartAccessActorDisposalFailure'] = $_.Exception
                }
            }
        }
        return $capture
    }
}

Export-ModuleMember -Function Invoke-PrestartDeploymentSession, Invoke-PrestartDeploymentOwner, Invoke-PrestartAccessSession
