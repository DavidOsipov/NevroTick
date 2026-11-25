# SPDX-License-Identifier: GPL-2.0-only
# W11-W05/W11-W10, S0.6 and CHECK-16/18/26/27.
[CmdletBinding(PositionalBinding = $false, DefaultParameterSetName = 'Scripted')]
param(
    [Parameter(Mandatory, ParameterSetName = 'Direct')][switch]$ExpectAbsent,
    [Parameter(Mandatory, ParameterSetName = 'Wrapper')][switch]$ExpectWrapperAbsent,
    [Parameter(Mandatory, ParameterSetName = 'UnsignedObservation')][string]$UnsignedArchive,
    [Parameter(Mandatory, ParameterSetName = 'UnsignedObservation')][long]$ExpectedArchiveBytes,
    [Parameter(Mandatory, ParameterSetName = 'UnsignedObservation')]
    [ValidatePattern('\A[a-f0-9]{64}\z')][string]$ExpectedArchiveSha256
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Assert-LabRegistrationAbsent {
    $sc = Join-Path $env:SystemRoot 'System32\sc.exe'
    foreach ($role in @('NevroTick-Lab-S02-Engine', 'NevroTick-Lab-S02-Broker')) {
        $null = & $sc queryex $role
        $code = $LASTEXITCODE
        if ($code -ne 1060) { throw "Expected error 1060 from status read for $role, received $code" }
    }
}

function Get-SampleRefusal {
    param([object]$Actor, [string]$SubjectSid, [Diagnostics.Stopwatch]$Clock)

    [object]$window = $null
    try {
        $window = Open-PrestartScmWindow -Actor $Actor -ExpectedDeploymentSid $SubjectSid `
            -Clock $Clock -TimeoutMilliseconds 15000
    } catch {
        [Exception]$failure = $_.Exception
        while ($null -ne $failure.InnerException -and $null -eq $failure.Data['Operation']) {
            $failure = $failure.InnerException
        }
        return $failure
    } finally {
        if ($null -ne $window) {
            [Exception]$retirement = $window.Retire()
            if ($null -ne $retirement) { throw $retirement }
        }
    }
    throw 'Unexpected acceptance of the absent SCM pair'
}

if ($ExpectAbsent -or $ExpectWrapperAbsent) {
    Assert-LabRegistrationAbsent
}
if ($ExpectAbsent -or $ExpectWrapperAbsent -or $PSCmdlet.ParameterSetName -ceq 'UnsignedObservation') {
    [Security.Principal.WindowsIdentity]$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    try { [string]$expectedSid = $identity.User.Value } finally { $identity.Dispose() }
}

if ($ExpectWrapperAbsent) {
    Import-Module (Join-Path $PSScriptRoot 'prestart_child.psm1') -DisableNameChecking
    [Diagnostics.Stopwatch]$clock = [Diagnostics.Stopwatch]::StartNew()
    [object]$actor = Open-PrestartObserverActor -Clock $clock -TimeoutMilliseconds 15000
    try {
        [Exception]$wrong = Get-SampleRefusal $actor 'S-1-0-0' $clock
        if ($wrong.Data['Operation'] -cne 'open-manager' -or $wrong.Data['Kind'] -cne 'deployment-actor' -or
            $null -eq $wrong.InnerException -or
            $wrong.InnerException.Message -cne 'prestart-observer:identity-subject-actor') {
            throw 'The wrapper did not reach the live deployment actor check before opening the SCM'
        }
        [Exception]$absent = Get-SampleRefusal $actor $expectedSid $clock
        if ($absent.Data['Operation'] -cne 'open-engine' -or $absent.Data['Kind'] -cne 'service-missing' -or
            $absent.Data['NativeErrorCode'] -ne 1060 -or $null -ne $absent.Data['RetirementFailures']) {
            throw 'The wrapper did not retain the original absent-service error and clean rollback'
        }
    } finally { $actor.Dispose() }
    Assert-LabRegistrationAbsent
    's06-prestart-scm-window wrapper actor and native absence passed'
    return
}

[string[]]$sources = @(
    (Join-Path $PSScriptRoot 'prestart_actor_token.cs'),
    (Join-Path $PSScriptRoot 'prestart_observer_child.cs'),
    (Join-Path $PSScriptRoot 'prestart_archive_facts.cs'),
    (Join-Path $PSScriptRoot 'prestart_scm_window.cs'),
    (Join-Path $PSScriptRoot 'test_prestart_scm_window.cs')
)
Add-Type -LiteralPath $sources -ReferencedAssemblies @('System.dll', 'System.Core.dll', 'System.Security.dll')

if ($ExpectAbsent) {
    [NevroTick.PrestartScmWindowControls]::ExpectAbsent($expectedSid)
    Assert-LabRegistrationAbsent
    's06-prestart-scm-window native actor and absence passed'
} elseif ($PSCmdlet.ParameterSetName -ceq 'UnsignedObservation') {
    [NevroTick.PrestartScmWindowControls]::ExpectUnsignedObservationRefused(
        $UnsignedArchive, $ExpectedArchiveBytes, $ExpectedArchiveSha256, $expectedSid)
    's06-prestart-scm-window unsigned observation archive controls passed'
} else {
    [NevroTick.PrestartScmWindowControls]::Run()
    's06-prestart-scm-window scripted controls passed'
}
