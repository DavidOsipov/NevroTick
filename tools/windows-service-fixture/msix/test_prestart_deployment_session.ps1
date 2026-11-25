# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
#Requires -PSEdition Desktop

[CmdletBinding(PositionalBinding = $false)]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
[Management.Automation.PSModuleInfo]$session = Import-Module (Join-Path $PSScriptRoot 'prestart_deployment_session.psm1') -PassThru -DisableNameChecking

function Read-ReplyRows([byte[]]$Bytes) {
    [int]$offset = 0
    while ($offset -lt $Bytes.Length) {
        if ($Bytes.Length - $offset -lt 4) { throw 'test frame header truncated' }
        [int]$length = [int][BitConverter]::ToUInt32($Bytes, $offset)
        $offset += 4
        if ($length -le 0 -or $length -gt $Bytes.Length - $offset) { throw 'test frame truncated' }
        [string]$json = [Text.Encoding]::UTF8.GetString($Bytes, $offset, $length)
        ConvertFrom-Json -InputObject $json
        $offset += $length
    }
}

function Invoke-SessionControl([hashtable]$Case) {
    [Collections.Generic.List[string]]$trace = [Collections.Generic.List[string]]::new()
    [object]$sample = [pscustomobject]@{
        Engine = [pscustomobject]@{ ConfigurationType = 528; StatusType = 528 }
        Broker = [pscustomobject]@{ ConfigurationType = 528; StatusType = 16 }
    }
    [IO.IOException]$closeError = [IO.IOException]::new('fixture pending close')
    $closeError.Data['Operation'] = 'close-engine'
    [object]$window = [pscustomobject]@{
        Trace = $trace
        FailSample = $Case.FailSample
        FailCheckAt = $Case.FailCheckAt
        CheckCount = 0
        Sample = $sample
        Capture = [pscustomobject]@{
            ProcessId = 42; ExitCode = 0
            InputSha256Before = ('a' * 64); InputSha256After = ('a' * 64)
            Stdout = [Text.Encoding]::ASCII.GetBytes('fixture capture')
            Stderr = [byte[]]::new(0)
        }
        FailureOnRetirement = $(if ($Case.Cleanup) { $closeError } else { $null })
    }
    $window | Add-Member -MemberType ScriptProperty -Name Initial -Value {
        $this.Trace.Add('sample')
        if ($this.FailSample) { throw [IO.InvalidDataException]::new('fixture sample refusal') }
        return $this.Sample
    }
    $window | Add-Member -MemberType ScriptMethod -Name Check -Value {
        $this.Trace.Add('check')
        $this.CheckCount += 1
        if ($this.CheckCount -eq $this.FailCheckAt) { throw [TimeoutException]::new('fixture check refusal') }
    }
    $window | Add-Member -MemberType ScriptMethod -Name CaptureIdentity -Value {
        $this.Trace.Add('identity')
        return $this.Capture
    }
    $window | Add-Member -MemberType ScriptMethod -Name CaptureObservation -Value {
        $this.Trace.Add('observation')
        return $this.Capture
    }
    $window | Add-Member -MemberType ScriptMethod -Name ConfirmDeployment -Value {
        $this.Trace.Add('confirm')
    }
    $window | Add-Member -MemberType ScriptMethod -Name Retire -Value {
        $this.Trace.Add('window')
        return $this.FailureOnRetirement
    }
    [object]$archive = [pscustomobject]@{ Trace = $trace; Fail = $Case.Cleanup }
    $archive | Add-Member -MemberType ScriptMethod -Name Dispose -Value {
        $this.Trace.Add('archive')
        if ($this.Fail) { throw [IO.IOException]::new('fixture archive close refusal') }
    }
    [object]$actor = [pscustomobject]@{ Trace = $trace; Fail = $Case.Cleanup }
    $actor | Add-Member -MemberType ScriptMethod -Name Dispose -Value {
        $this.Trace.Add('actor')
        if ($this.Fail) { throw [IO.IOException]::new('fixture actor close refusal') }
    }
    [IO.MemoryStream]$inputStream = [IO.MemoryStream]::new([Text.Encoding]::ASCII.GetBytes($Case.Input), $false)
    [IO.MemoryStream]$outputStream = if ($Case.Broken) {
        [IO.MemoryStream]::new([byte[]]::new(0), $false)
    } else { [IO.MemoryStream]::new() }
    [Exception]$caught = $null
    [Diagnostics.Stopwatch]$clock = [Diagnostics.Stopwatch]::StartNew()
    [guid]$run = '91788bb2-be2f-4dab-9644-2de73bd41286'
    [bool]$fullPrestart = $Case.ContainsKey('FullPrestart') -and $Case.FullPrestart
    try {
        & $session {
            param($InputStream, $OutputStream, $Run, $Clock, $Window, $Archive, $Actor, [bool]$FullPrestart)
            [hashtable]$parameters = @{
                InputStream = $InputStream; OutputStream = $OutputStream; RunId = $Run; Clock = $Clock
                TimeoutMilliseconds = 1000; RetirementTimeoutMilliseconds = 2000
                Window = $Window; Archive = $Archive; Actor = $Actor
                CaptureIdentity = { $Window.CaptureIdentity() }
                CaptureObservation = { $Window.CaptureObservation() }
            }
            if ($FullPrestart) { $parameters['FullPrestart'] = $true }
            Invoke-PrestartDeploymentSessionCore @parameters
        } $inputStream $outputStream $run $clock $window $archive $actor $fullPrestart
    } catch {
        $caught = $_.Exception
    } finally {
        [byte[]]$bytes = $outputStream.ToArray()
        $outputStream.Dispose()
        $inputStream.Dispose()
    }
    [object[]]$rows = @(Read-ReplyRows $bytes)
    [string]$seen = [string]::Join(',', $trace)
    if ($seen -cne $Case.Trace) { throw ('wrong retirement history: ' + $seen) }
    if ([string]::Join(',', [string[]]@($rows | ForEach-Object { $_.kind })) -cne $Case.Replies) {
        throw 'wrong wire reply roster'
    }
    foreach ($row in $rows) {
        if ($row.run -cne $run.ToString('D')) { throw 'wrong wire run' }
        if ($row.kind -cin @('current', 'access-current', 'standard-current') -and
            @($row.PSObject.Properties).Count -ne 2) {
            throw 'wrong currentness reply shape'
        }
    }
    if ($Case.Replies -like '*refused*') {
        [object[]]$refusal = @($rows | Where-Object { $_.kind -ceq 'refused' })
        if ($refusal.Count -ne 1 -or $refusal[0].code -cne $Case.WireRefusal) {
            throw 'wrong wire refusal'
        }
    }
    [string]$expectedCleanup = if ($Case.Cleanup) {
        'archive-dispose-failed,actor-dispose-failed,window-close-engine-failed'
    } else { '' }
    if ($Case.Transport -ceq '') {
        if ($null -ne $caught) { throw $caught }
        if ($rows[-1].kind -cne 'retired' -or [string]::Join(',', [string[]]$rows[-1].cleanup) -cne $expectedCleanup) {
            throw 'wrong confirmed retirement'
        }
        return
    }
    [hashtable]$failure = $null
    for ([int]$depth = 0; $depth -lt 6 -and $null -ne $caught; $depth++) {
        if ($caught.Data.Contains('PrestartDeploymentSessionFailure')) {
            $failure = $caught.Data['PrestartDeploymentSessionFailure']
            break
        }
        $caught = $caught.InnerException
    }
    if ($null -eq $failure) { throw 'missing unconfirmed-retirement evidence' }
    if ($failure.Operation -cne $Case.Operation -or
        [string]::Join(',', [string[]]$failure.Transport) -cne $Case.Transport -or
        [string]::Join(',', [string[]]$failure.Cleanup) -cne $expectedCleanup) {
        throw ('wrong unconfirmed-retirement evidence: ' + (ConvertTo-Json -InputObject $failure -Compress))
    }
}

# Observe the actual memory-stream boundaries without using a timer or SCM.
Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

namespace NevroTick.SessionControl
{
    public sealed class AccessInput : MemoryStream
    {
        private readonly List<string> trace;
        private readonly bool changeActor;

        public AccessInput(byte[] input, List<string> trace, bool changeActor) : base(input, false)
        {
            this.trace = trace;
            this.changeActor = changeActor;
            Generation = 41;
        }

        public int Generation { get; private set; }

        public override Task<int> ReadAsync(byte[] buffer, int offset, int count, CancellationToken cancellationToken)
        {
            trace.Add("read");
            if (changeActor)
                Generation++;
            return base.ReadAsync(buffer, offset, count, cancellationToken);
        }
    }

    public sealed class AccessOutput : MemoryStream
    {
        private readonly List<string> trace;
        private readonly bool broken;

        public AccessOutput(List<string> trace, bool broken)
        {
            this.trace = trace;
            this.broken = broken;
        }

        public override void Write(byte[] buffer, int offset, int count)
        {
            trace.Add("write");
            if (broken)
                throw new IOException("fixture access reply write");
            base.Write(buffer, offset, count);
        }
    }

    public sealed class AccessActor : IDisposable
    {
        private readonly List<string> trace;
        private readonly IOException failure;

        public AccessActor(List<string> trace, int generation, IOException failure)
        {
            this.trace = trace;
            this.failure = failure;
            Generation = generation;
        }

        public int Generation { get; private set; }

        public void Dispose()
        {
            trace.Add("dispose");
            if (failure != null)
                throw failure;
        }
    }
}
'@

function Get-ControlExceptionChain([Exception]$Failure) {
    for ([int]$depth = 0; $depth -lt 8 -and $null -ne $Failure; $depth++) {
        $Failure
        $Failure = $Failure.InnerException
    }
}

function Invoke-AccessSessionControl([hashtable]$Case) {
    [guid]$run = 'bd312a9e-8dce-4bd8-a73f-666b43b3296c'
    [string]$sid = 'S-1-5-21-11-22-33-1001'
    [Collections.Generic.List[string]]$trace = [Collections.Generic.List[string]]::new()
    [IO.IOException]$disposal = if ($Case['DisposeFailure']) {
        [IO.IOException]::new('fixture actor close')
    } else { $null }
    [IO.IOException]$observerFailure = [IO.IOException]::new('fixture observer refused')
    if ($Case['ChildCleanup']) {
        $observerFailure.Data['PrestartObserverTerminationFailure'] = [IO.IOException]::new('fixture child terminate')
    }
    [hashtable]$probe = @{
        Trace = $trace
        Actor = $null
        Disposal = $disposal
        Calls = [Collections.Generic.List[hashtable]]::new()
        Failure = $(if ($Case.CaptureFailure) { $observerFailure } else { $null })
        GenerationFailure = [IO.IOException]::new('fixture actor changed while awaiting command')
        Result = [pscustomobject]@{
            ProcessId = 43; ExitCode = $Case.ExitCode
            InputSha256Before = ('b' * 64); InputSha256After = ('b' * 64)
            Stdout = [Text.Encoding]::ASCII.GetBytes('observer stdout')
            Stderr = [Text.Encoding]::ASCII.GetBytes('observer stderr')
        }
    }
    [IO.MemoryStream]$inputStream = [NevroTick.SessionControl.AccessInput]::new(
        [Text.Encoding]::ASCII.GetBytes($Case.Input), $trace, [bool]$Case['ChangeActor'])
    [IO.MemoryStream]$outputStream = [NevroTick.SessionControl.AccessOutput]::new($trace, [bool]$Case['Broken'])
    [Diagnostics.Stopwatch]$clock = [Diagnostics.Stopwatch]::StartNew()
    [Exception]$caught = $null
    try {
        & $session {
            param($Role, $InputStream, $OutputStream, $Run, $Clock, $Sid, $Probe)
            [hashtable]$parameters = @{
                Role = $Role; RunId = $Run; Clock = $Clock; TimeoutMilliseconds = 1000
                ExpectedActorSid = $Sid; InputStream = $InputStream; OutputStream = $OutputStream
            }
            Invoke-PrestartAccessSessionCore @parameters -OpenActor {
                $Probe.Trace.Add('open')
                $Probe.Actor = [NevroTick.SessionControl.AccessActor]::new(
                    $Probe.Trace, $InputStream.Generation, $Probe.Disposal)
                return $Probe.Actor
            } -Capture {
                param([hashtable]$ObserverInputs, [object]$Actor)
                $Probe.Trace.Add('capture')
                $Probe.Calls.Add($ObserverInputs)
                if (![object]::ReferenceEquals($Actor, $Probe.Actor)) { throw 'fixture received a different actor' }
                if ($Actor.Generation -ne $InputStream.Generation) { throw $Probe.GenerationFailure }
                if ($null -ne $Probe.Failure) { throw $Probe.Failure }
                return $Probe.Result
            }
        } $Case.Role $inputStream $outputStream $run $clock $sid $probe
    } catch {
        $caught = $_.Exception
    } finally {
        [byte[]]$bytes = $outputStream.ToArray()
        $outputStream.Dispose()
        $inputStream.Dispose()
    }
    [Exception[]]$chain = @(Get-ControlExceptionChain $caught)
    if ($Case.Error -ceq '') {
        if ($null -ne $caught) { throw $caught }
    } elseif (!(@($chain | ForEach-Object { $_.Message }) -ccontains $Case.Error)) {
        throw ('wrong access session failure: ' + [string]::Join(' / ', [string[]]@($chain | ForEach-Object { $_.Message })))
    }
    [string]$history = [string]::Join(',', $trace)
    if ($history -cne $Case.Trace) { throw ('wrong access actor history: ' + $history) }
    if ($probe.Calls.Count -ne $Case.Calls) { throw 'wrong access session launch count' }
    if ($probe.Calls.Count -ne 0) {
        [hashtable]$observed = $probe.Calls[0]
        if ($observed.Count -ne 2 -or $observed.Selector -cne $Case.Selector -or
            $observed[$Case.SidKey] -cne $sid) {
            throw 'access session selected the wrong observer or actor'
        }
    }
    if ($Case.CaptureFailure -and !($chain | Where-Object { [object]::ReferenceEquals($_, $observerFailure) })) {
        throw 'access session replaced the original observer failure'
    }
    [string]$disposalKey = 'PrestartAccessActorDisposalFailure'
    [object[]]$disposition = @($chain | Where-Object { $_.Data.Contains($disposalKey) } |
        ForEach-Object { $_.Data[$disposalKey] })
    if ($Case['DisposeFailure']) {
        if ($disposition.Count -ne 1) { throw 'access session lost or duplicated the actor disposal failure' }
        if ($Case.Error -ceq 'fixture actor close') {
            if ($disposition[0] -isnot [bool] -or !$disposition[0]) { throw 'actor disposal was not marked primary' }
        } elseif ($disposition[0] -isnot [Exception] -or
            !(@(Get-ControlExceptionChain $disposition[0]) | Where-Object { [object]::ReferenceEquals($_, $disposal) })) {
            throw 'actor disposal was not retained with the original primary failure'
        }
    } elseif ($disposition.Count -ne 0) { throw 'access session invented an actor disposal failure' }

    [string]$transportKey = 'PrestartAccessRefusalWriteFailure'
    [Exception[]]$transport = @($chain | Where-Object { $_.Data.Contains($transportKey) } |
        ForEach-Object { $_.Data[$transportKey] })
    if ($Case['RefusalWriteFailure']) {
        if ($transport.Count -ne 1 -or $transport[0].Message -cne 'prestart-deployment-session:reply-transport') {
            throw 'access session lost the refusal write failure'
        }
    } elseif ($transport.Count -ne 0) { throw 'access session invented a refusal write failure' }

    if ($Case['RefusedFrame']) {
        [object[]]$refusals = @(Read-ReplyRows $bytes)
        [string]$expectedCleanup = if ($Case['ChildCleanup']) {
            'child-termination-failed,actor-dispose-failed'
        } elseif ($Case['DisposeFailure']) {
            'actor-dispose-failed'
        } else { '' }
        if ($refusals.Count -ne 1 -or $refusals[0].kind -cne 'refused' -or
            $refusals[0].run -cne $run.ToString('D') -or $refusals[0].code -cne 'access-failed' -or
            @($refusals[0].PSObject.Properties).Count -ne 4 -or
            [string]::Join(',', [string[]]$refusals[0].cleanup) -cne $expectedCleanup) {
            throw 'wrong access session refusal frame'
        }
        return
    }
    if (!$Case.Reply) {
        if ($bytes.Length -ne 0) { throw 'access session emitted an unexpected reply' }
        return
    }
    [object[]]$rows = @(Read-ReplyRows $bytes)
    if ($rows.Count -ne 1) { throw 'wrong access session frame count' }
    [object]$row = $rows[0]
    [string]$fields = [string]::Join(',', [string[]]@($row.PSObject.Properties.Name | Sort-Object))
    if ($fields -cne 'after,before,exit,kind,pid,run,stderr,stdout' -or
        $row.run -cne $run.ToString('D') -or $row.kind -cne $Case.Role -or
        $row.pid -ne 43 -or $row.exit -ne $Case.ExitCode -or
        $row.before -cne ('b' * 64) -or $row.after -cne ('b' * 64) -or
        [Text.Encoding]::ASCII.GetString([Convert]::FromBase64String($row.stdout)) -cne 'observer stdout' -or
        [Text.Encoding]::ASCII.GetString([Convert]::FromBase64String($row.stderr)) -cne 'observer stderr') {
        throw 'wrong access session capture frame'
    }
}

$accessSelector = 'scm_access_probe::windows::native_installed_roles_prestart_access'
$standardSelector = 'scm_access_probe::windows::native_standard_user_prestart_change_config_refused'
$accessTrace = 'open,read,read,capture,dispose,write,write'
[hashtable[]]$accessCases = @(
    @{
        Name = 'access-producer-captures-provisioning'; Role = 'access'; Input = 'A'; Calls = 1; Reply = $true
        CaptureFailure = $false; ExitCode = 0; Error = ''; Trace = $accessTrace
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
    @{
        Name = 'access-producer-preserves-standard-child-refusal'; Role = 'standard'; Input = 'U'; Calls = 1; Reply = $true
        CaptureFailure = $false; ExitCode = 5; Error = ''; Trace = $accessTrace
        Selector = $standardSelector; SidKey = 'ExpectedStandardUserSid'
    }
    @{
        Name = 'access-producer-accepts-unused-eof'; Role = 'access'; Input = ''; Calls = 0; Reply = $false
        CaptureFailure = $false; ExitCode = 0; Error = ''; Trace = 'open,read,dispose'
    }
    @{
        Name = 'access-producer-refuses-crossed-role-before-launch'; Role = 'standard'; Input = 'A'; Calls = 0; Reply = $false
        CaptureFailure = $false; ExitCode = 0; Error = 'prestart-access-session:unexpected-command'; Trace = 'open,read,dispose'
    }
    @{
        Name = 'access-producer-refuses-trailing-before-launch'; Role = 'access'; Input = 'AA'; Calls = 0; Reply = $false
        CaptureFailure = $false; ExitCode = 0; Error = 'prestart-access-session:trailing-command'; Trace = 'open,read,read,dispose'
    }
    @{
        Name = 'access-producer-preserves-native-capture-failure'; Role = 'access'; Input = 'A'; Calls = 1; Reply = $false
        CaptureFailure = $true; ExitCode = 0; Error = 'fixture observer refused'; Trace = $accessTrace; RefusedFrame = $true
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
    @{
        Name = 'access-producer-preserves-capture-and-cleanup-failures'; Role = 'access'; Input = 'A'; Calls = 1
        Reply = $false; CaptureFailure = $true; ChildCleanup = $true; DisposeFailure = $true; ExitCode = 0
        Error = 'fixture observer refused'; Trace = $accessTrace; RefusedFrame = $true
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
    @{
        Name = 'access-producer-preserves-a-sole-actor-disposal-failure'; Role = 'access'; Input = 'A'; Calls = 1
        Reply = $false; CaptureFailure = $false; DisposeFailure = $true; ExitCode = 0
        Error = 'fixture actor close'; Trace = $accessTrace; RefusedFrame = $true
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
    @{
        Name = 'access-producer-keeps-actor-from-before-command-read'; Role = 'access'; Input = 'A'; Calls = 1
        Reply = $false; CaptureFailure = $false; ChangeActor = $true; ExitCode = 0
        Error = 'fixture actor changed while awaiting command'; Trace = $accessTrace; RefusedFrame = $true
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
    @{
        Name = 'access-producer-keeps-protocol-refusal-with-disposal-failure'; Role = 'standard'; Input = 'A'; Calls = 0
        Reply = $false; CaptureFailure = $false; DisposeFailure = $true; ExitCode = 0; Trace = 'open,read,dispose'
        Error = 'prestart-access-session:unexpected-command'
    }
    @{
        Name = 'access-producer-reports-unused-actor-disposal-failure'; Role = 'access'; Input = ''; Calls = 0
        Reply = $false; CaptureFailure = $false; DisposeFailure = $true; ExitCode = 0; Trace = 'open,read,dispose'
        Error = 'fixture actor close'
    }
    @{
        Name = 'access-producer-keeps-primary-disposal-and-refusal-write-errors'; Role = 'access'; Input = 'A'; Calls = 1
        Reply = $false; CaptureFailure = $true; DisposeFailure = $true; Broken = $true; ExitCode = 0
        Error = 'fixture observer refused'; Trace = 'open,read,read,capture,dispose,write'; RefusalWriteFailure = $true
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
    @{
        Name = 'access-producer-does-not-rewrite-a-broken-success-frame'; Role = 'access'; Input = 'A'; Calls = 1
        Reply = $false; CaptureFailure = $false; Broken = $true; ExitCode = 0
        Error = 'prestart-deployment-session:reply-transport'; Trace = 'open,read,read,capture,dispose,write'
        Selector = $accessSelector; SidKey = 'ExpectedProvisioningUserSid'
    }
)

[hashtable[]]$cases = @(
    @{
        Name = 'paired-default-confirms'; Input = 'SCICOFR'; FailSample = $false; FailCheckAt = 0
        Cleanup = $false; Broken = $false; Transport = ''
        Trace = 'sample,check,identity,check,observation,confirm,window,archive,actor'
        Replies = 'sample,current,identity,current,observation,confirmed,retired'
    }
    @{
        Name = 'paired-refuses-full-access-check'; Input = 'SCICOAR'; FailSample = $false; FailCheckAt = 0
        Cleanup = $false; Broken = $false; Trace = 'sample,check,identity,check,observation,window,archive,actor'
        Replies = 'sample,current,identity,current,observation,refused'
        Operation = ''; Transport = 'unexpected-command'; WireRefusal = 'unexpected-command'
    }
    @{
        Name = 'full-checks-both-access-edges'; Input = 'SCICOAUFR'; FullPrestart = $true
        FailSample = $false; FailCheckAt = 0; Cleanup = $false; Broken = $false; Transport = ''
        Trace = 'sample,check,identity,check,observation,check,check,confirm,window,archive,actor'
        Replies = 'sample,current,identity,current,observation,access-current,standard-current,confirmed,retired'
    }
    @{
        Name = 'full-refuses-skipped-standard-check'; Input = 'SCICOAFR'; FullPrestart = $true
        FailSample = $false; FailCheckAt = 0; Cleanup = $false; Broken = $false
        Trace = 'sample,check,identity,check,observation,check,window,archive,actor'
        Replies = 'sample,current,identity,current,observation,access-current,refused'
        Operation = ''; Transport = 'unexpected-command'; WireRefusal = 'unexpected-command'
    }
    @{
        Name = 'full-refuses-standard-before-provisioning'; Input = 'SCICOUAFR'; FullPrestart = $true
        FailSample = $false; FailCheckAt = 0; Cleanup = $false; Broken = $false
        Trace = 'sample,check,identity,check,observation,window,archive,actor'
        Replies = 'sample,current,identity,current,observation,refused'
        Operation = ''; Transport = 'unexpected-command'; WireRefusal = 'unexpected-command'
    }
    @{
        Name = 'full-retires-after-provisioning-check-refusal'; Input = 'SCICOAR'; FullPrestart = $true
        FailSample = $false; FailCheckAt = 3; Cleanup = $true; Broken = $false; Transport = ''
        Trace = 'sample,check,identity,check,observation,check,window,archive,actor'
        Replies = 'sample,current,identity,current,observation,refused,retired'
        WireRefusal = 'provisioning-check-deadline'
    }
    @{
        Name = 'healthy-retirement'; Input = 'R'; FailSample = $false; FailCheckAt = 0; Cleanup = $false
        Broken = $false; Trace = 'window,archive,actor'; Replies = 'retired'; Transport = ''
    }
    @{
        Name = 'retire-after-operation-refusal'; Input = 'SCR'; FailSample = $false; FailCheckAt = 1; Cleanup = $true
        Broken = $false; Trace = 'sample,check,window,archive,actor'; Replies = 'sample,refused,retired'
        Transport = ''; WireRefusal = 'check-deadline'
    }
    @{
        Name = 'early-eof-after-operation-refusal'; Input = 'SC'; FailSample = $false; FailCheckAt = 1; Cleanup = $true
        Broken = $false; Trace = 'sample,check,window,archive,actor'; Replies = 'sample,refused'
        Operation = 'check-deadline'; Transport = 'unexpected-eof'; WireRefusal = 'check-deadline'
    }
    @{
        Name = 'trailing-after-operation-refusal'; Input = 'SCRX'; FailSample = $false; FailCheckAt = 1; Cleanup = $true
        Broken = $false; Trace = 'sample,check,window,archive,actor'; Replies = 'sample,refused'
        Operation = 'check-deadline'; Transport = 'trailing-command'; WireRefusal = 'check-deadline'
    }
    @{
        Name = 'operation-refusal-write-breaks'; Input = 'S'; FailSample = $true; FailCheckAt = 0; Cleanup = $true
        Broken = $true; Trace = 'sample,window,archive,actor'; Replies = ''
        Operation = 'sample-failed'; Transport = 'reply-transport'
    }
    @{
        Name = 'retired-frame-write-breaks'; Input = 'R'; FailSample = $false; FailCheckAt = 0; Cleanup = $true
        Broken = $true; Trace = 'window,archive,actor'; Replies = ''
        Operation = ''; Transport = 'reply-transport'
    }
    @{
        Name = 'early-eof-preserves-wire-refusal'; Input = ''; FailSample = $false; FailCheckAt = 0; Cleanup = $true
        Broken = $false; Trace = 'window,archive,actor'; Replies = 'refused'; WireRefusal = 'unexpected-eof'
        Operation = ''; Transport = 'unexpected-eof'
    }
)
[int]$missed = 0
foreach ($case in $cases) {
    try {
        Invoke-SessionControl $case
        [Console]::WriteLine('passed ' + $case.Name)
    } catch {
        $missed++
        [Console]::Error.WriteLine('failed ' + $case.Name + ' ' + $_.Exception.Message)
    }
}
foreach ($case in $accessCases) {
    try {
        Invoke-AccessSessionControl $case
        [Console]::WriteLine('passed ' + $case.Name)
    } catch {
        $missed++
        [Console]::Error.WriteLine('failed ' + $case.Name + ' ' + $_.Exception.Message)
    }
}
Remove-Module $session
[Console]::WriteLine("cases=$($cases.Length + $accessCases.Length) failed=$missed")
if ($missed -ne 0) { exit 1 }
