# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
#Requires -PSEdition Desktop
[CmdletBinding(PositionalBinding = $false)]
param(
    [Parameter(Mandatory)][guid]$RunId,
    [Parameter(Mandatory)][string]$ExpectedSha256,
    [Parameter(Mandatory)][int]$ExpectedBytes
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSScriptRoot 'prestart_deployment_activation.psm1') -ErrorAction Stop

[byte[]]$original = [IO.File]::ReadAllBytes((Join-Path $PSScriptRoot 'deployment-activation.bin'))
[string]$root = Join-Path $PSScriptRoot ([guid]::NewGuid().ToString('N'))
$null = [IO.Directory]::CreateDirectory($root)

function New-Control([string]$Name, [byte[]]$Frame, [byte[]]$Marker) {
    [string]$case = Join-Path $root $Name
    [string]$records = Join-Path $case 'input'
    [string]$stage = Join-Path $case 'local'
    $null = [IO.Directory]::CreateDirectory($records)
    $null = [IO.Directory]::CreateDirectory($stage)
    [string]$inputPath = Join-Path $records 'deployment-activation.bin'
    [IO.File]::WriteAllBytes($inputPath, $Frame)
    if ($null -ne $Marker) { [IO.File]::WriteAllBytes($inputPath + '.commit', $Marker) }
    return @{ RecordsIn = $records; Stage = $stage; RunId = $RunId
        Clock = [Diagnostics.Stopwatch]::StartNew(); WorkMilliseconds = 8000 }
}

function Assert-Refused([hashtable]$Arguments, [string]$Expected = '') {
    try {
        $unexpected = Receive-PrestartDeploymentActivation @Arguments
    } catch {
        if ($Expected -and $_.Exception.Message -notlike ('*' + $Expected + '*')) { throw }
        return
    }
    $unexpected.Lease.Dispose()
    throw 'expected activation refusal'
}

try {
    [hashtable]$healthy = New-Control 'healthy' $original ([byte[]]::new(0))
    $admitted = Receive-PrestartDeploymentActivation @healthy
    try {
        if ($admitted.Path -cne (Join-Path $healthy.Stage 'prestart_deployment_owner.ps1') -or
            $admitted.Bytes -ne $ExpectedBytes -or $admitted.Sha256 -cne $ExpectedSha256 -or
            $admitted.Lease.Position -ne 0 -or !$admitted.Lease.CanRead) {
            throw 'wrong live staged entry'
        }
        if ((Get-FileHash -Algorithm SHA256 -LiteralPath $admitted.Path).Hash.ToLowerInvariant() -cne $ExpectedSha256) {
            throw 'independent staged readback differs'
        }
        [bool]$excludedDelete = $false
        try { [IO.File]::Delete($admitted.Path) } catch [IO.IOException] { $excludedDelete = $true }
        if (!$excludedDelete) { throw 'release during active lease' }
    } finally { $admitted.Lease.Dispose() }
    Assert-Refused $healthy
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $admitted.Path).Hash.ToLowerInvariant() -cne $ExpectedSha256) {
        throw 'duplicate changed the original local leaf'
    }
    [IO.File]::Delete($admitted.Path)

    [byte[]]$tampered = $original.Clone()
    $tampered[-1] = $tampered[-1] -bxor 1
    [hashtable]$altered = New-Control 'tampered' $tampered ([byte[]]::new(0))
    Assert-Refused $altered 'prestart-activation:digest'
    [hashtable]$uncommitted = New-Control 'uncommitted' $original $null
    Assert-Refused $uncommitted
    [hashtable]$badCommit = New-Control 'bad-commit' $original ([byte[]]@(1))
    Assert-Refused $badCommit 'prestart-activation:commit'
    [hashtable]$foreign = New-Control 'foreign-run' $original ([byte[]]::new(0))
    $foreign.RunId = [guid]'b3409394-c69e-47cf-bae4-14389983ed5c'
    Assert-Refused $foreign 'prestart-activation:header'
    [hashtable]$expired = New-Control 'expired' $original ([byte[]]::new(0))
    $expired.Clock.Stop()
    Assert-Refused $expired 'prestart-activation:admission'
    [hashtable]$trailing = New-Control 'trailing' ([byte[]]($original + 0)) ([byte[]]::new(0))
    Assert-Refused $trailing 'prestart-activation:extent'
    foreach ($case in @($altered, $uncommitted, $badCommit, $foreign, $expired, $trailing)) {
        if (@([IO.Directory]::GetFileSystemEntries($case.Stage)).Count -ne 0) { throw 'local output on refusal' }
    }
    [Console]::WriteLine('activation-controls=8 failed=0')
} finally {
    [IO.Directory]::Delete($root, $true)
}
