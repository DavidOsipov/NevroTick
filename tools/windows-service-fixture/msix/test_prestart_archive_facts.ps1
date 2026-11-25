# SPDX-License-Identifier: GPL-2.0-only
# CHECK-18/26/27. Read two exact unsigned laboratory archives without installation.
[CmdletBinding(PositionalBinding = $false)]
param(
    [Parameter(Mandatory = $true)][string]$CurrentArchive,
    [Parameter(Mandatory = $true)][string]$CurrentSha256,
    [Parameter(Mandatory = $true)][string]$PreviousArchive,
    [Parameter(Mandatory = $true)][string]$PreviousSha256,
    [string]$CurrentPayloadSha256,
    [string]$CrossedPayloadSha256
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Add-Type -Path (Join-Path $PSScriptRoot 'prestart_archive_facts.cs')

function Read-FixtureBytes([string]$Path) {
    [IO.FileStream]$stream = [IO.File]::Open($Path, 'Open', 'Read', 'Read')
    try {
        [long]$length = $stream.Length
        if ($length -lt 1 -or $length -gt 8MB) { throw 'archive-control:input-size' }
        [byte[]]$bytes = [byte[]]::new([int]$length)
        [int]$offset = 0
        while ($offset -lt $bytes.Length) {
            [int]$count = $stream.Read($bytes, $offset, $bytes.Length - $offset)
            if ($count -le 0) { throw 'archive-control:short-read' }
            $offset += $count
        }
        if ($stream.ReadByte() -ne -1) { throw 'archive-control:extra-byte' }
        ,$bytes
    } finally {
        $stream.Dispose()
    }
}

function Get-Sha256([byte[]]$Bytes) {
    [Security.Cryptography.SHA256]$sha = [Security.Cryptography.SHA256]::Create()
    try {
        [BitConverter]::ToString($sha.ComputeHash($Bytes)).Replace('-', '').ToLowerInvariant()
    } finally {
        $sha.Dispose()
    }
}

function Assert-Refused(
    [byte[]]$Archive, [string]$Digest, [type]$ExceptionType, [string]$ExpectedMessage
) {
    try {
        [void][NevroTick.PrestartArchiveFacts]::Inspect($Archive, $Digest)
    } catch {
        [Exception]$cause = $_.Exception
        while ($null -ne $cause.InnerException) { $cause = $cause.InnerException }
        if ($cause.GetType() -ne $ExceptionType -or $cause.Message -cne $ExpectedMessage) { throw }
        if ($cause -is [Runtime.InteropServices.COMException] -and $cause.ErrorCode -ge 0) { throw }
        return
    }
    throw ('archive-control:missing-refusal:' + $ExpectedMessage)
}

[byte[]]$current = Read-FixtureBytes $CurrentArchive
[byte[]]$previous = Read-FixtureBytes $PreviousArchive
[NevroTick.PrestartArchiveFacts]$first = [NevroTick.PrestartArchiveFacts]::Inspect($current, $CurrentSha256)
Assert-Refused $previous $CurrentSha256 ([IO.InvalidDataException]) 'prestart-archive:archive-digest'
[NevroTick.PrestartArchiveFacts]$older = [NevroTick.PrestartArchiveFacts]::Inspect($previous, $PreviousSha256)

Assert-Refused $current ($CurrentSha256.ToUpperInvariant()) ([IO.InvalidDataException]) 'prestart-archive:expected-sha256'
Assert-Refused ([byte[]]::new(0)) $CurrentSha256 ([IO.InvalidDataException]) 'prestart-archive:archive-size'
Assert-Refused ([byte[]]::new(8MB + 1)) $CurrentSha256 ([IO.InvalidDataException]) 'prestart-archive:archive-size'
[byte[]]$nonPackage = [Text.Encoding]::ASCII.GetBytes('not a package')
Assert-Refused $nonPackage (Get-Sha256 $nonPackage) ([Runtime.InteropServices.COMException]) 'prestart-archive:package-reader'

[NevroTick.PrestartArchiveFacts]$restored = [NevroTick.PrestartArchiveFacts]::Inspect($current, $CurrentSha256)
[hashtable]$observation = @{
    current = $first
    previous = $older
    restored = $restored
    refused = @('crossed-pin', 'malformed-pin', 'empty', 'oversize', 'non-package-with-matching-pin')
}

# The independent crossed catalog retains both role files and changes the expected launcher.
if ($PSBoundParameters.ContainsKey('CurrentPayloadSha256') -or $PSBoundParameters.ContainsKey('CrossedPayloadSha256')) {
    if ($CurrentPayloadSha256 -cnotmatch '\A[0-9a-f]{64}\z' -or
        $CrossedPayloadSha256 -cnotmatch '\A[0-9a-f]{64}\z' -or
        $CurrentPayloadSha256 -ceq $CrossedPayloadSha256) {
        throw 'archive-control:payload-pins'
    }
    [NevroTick.PrestartArchiveFacts]$bound = [NevroTick.PrestartArchiveFacts]::InspectWithPayload(
        $current, $CurrentSha256, $CurrentPayloadSha256)
    [bool]$crossedRefused = $false
    try {
        [void][NevroTick.PrestartArchiveFacts]::InspectWithPayload($current, $CurrentSha256, $CrossedPayloadSha256)
    } catch {
        [Exception]$cause = $_.Exception
        while ($null -ne $cause.InnerException) { $cause = $cause.InnerException }
        if ($cause -isnot [IO.InvalidDataException] -or $cause.Message -cne 'prestart-archive:payload-digest') { throw }
        $crossedRefused = $true
    }
    [NevroTick.PrestartArchiveFacts]$boundRestored = [NevroTick.PrestartArchiveFacts]::InspectWithPayload(
        $current, $CurrentSha256, $CurrentPayloadSha256)
    if (!$crossedRefused -or $null -ne $first.PayloadSha256 -or
        $bound.PayloadSha256 -cne $CurrentPayloadSha256 -or $boundRestored.PayloadSha256 -cne $CurrentPayloadSha256 -or
        $bound.EngineSha256 -cne $first.EngineSha256 -or $bound.BrokerSha256 -cne $first.BrokerSha256) {
        throw 'archive-control:payload-binding'
    }
    $observation.payload = @{ current = $bound.PayloadSha256; crossedRefused = $crossedRefused; restored = $boundRestored.PayloadSha256 }
}
ConvertTo-Json -InputObject $observation -Compress -Depth 3
