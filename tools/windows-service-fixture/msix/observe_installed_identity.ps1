# SPDX-License-Identifier: GPL-2.0-only
#Requires -Version 5.1
#Requires -PSEdition Desktop

[CmdletBinding(PositionalBinding = $false)]
param(
    [Parameter(Mandatory)][guid]$RunId,
    [Parameter(Mandatory)][string]$PackageFullName,
    [Parameter(Mandatory)][string]$ExpectedUserSid,
    [Parameter(Mandatory)][string]$SignedPackageSha256,
    [Parameter(Mandatory)][long]$EngineBytes,
    [Parameter(Mandatory)][string]$EngineSha256,
    [Parameter(Mandatory)][long]$BrokerBytes,
    [Parameter(Mandatory)][string]$BrokerSha256
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (![Environment]::Is64BitProcess) {
    throw 'Installed identity requires 64-bit Windows PowerShell'
}

# Windows PowerShell can reassemble PSModulePath on startup. Resolve these
# providers only from the original interpreter's Windows module directory.
[string]$osModules = [IO.Path]::Combine($PSHOME, 'Modules')
$env:PSModulePath = $osModules
$PSModuleAutoLoadingPreference = 'None'
foreach ($name in @('Microsoft.PowerShell.Utility', 'Appx', 'CimCmdlets')) {
    $null = Import-Module -Global -Name ([IO.Path]::Combine($osModules, $name, $name + '.psd1'))
}
$null = Import-Module -Name ([IO.Path]::Combine($PSScriptRoot, 'installed_identity.psm1'))
[hashtable]$inputs = @{
    RunId = $RunId
    PackageFullName = $PackageFullName
    ExpectedUserSid = $ExpectedUserSid
    SignedPackageSha256 = $SignedPackageSha256
    EngineBytes = $EngineBytes
    EngineSha256 = $EngineSha256
    BrokerBytes = $BrokerBytes
    BrokerSha256 = $BrokerSha256
}
installed_identity\Write-S06InstalledIdentity -Inputs $inputs -OutputStream ([Console]::OpenStandardOutput())
