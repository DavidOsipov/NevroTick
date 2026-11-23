# SPDX-License-Identifier: GPL-2.0-only
# Local path and read-only file controls. No AppX, SCM or installation operations.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$module = Import-Module (Join-Path $PSScriptRoot 'installed_identity.psm1') -Force -PassThru
try {
    & $module {
        function Assert-Refused([scriptblock]$Action, [string]$Expected) {
            $output = [Collections.Generic.List[object]]::new()
            try {
                & $Action | ForEach-Object { $output.Add($_) }
            } catch {
                if ($output.Count -eq 0 -and $_.Exception.Message -ceq $Expected) { return }
                throw
            }
            throw "Expected refusal: $Expected"
        }

        function Assert-ControlImage([string]$Path, [long]$Bytes, [string]$Digest) {
            $image = [IO.File]::Open($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
            try {
                Confirm-S06Image $image $Path $Bytes $Digest
            } finally {
                $image.Dispose()
            }
        }

        $name = 'NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc'
        $root = "C:\Package Volume\$name"
        $member = 'nevrotick-lab-s02-engine.exe'
        $key = 'NevroTick-Lab-S02-Engine'
        $expectedPath = "$root\$member"
        $registered = [pscustomobject]@{
            PackageFullName = $name; Name = 'NevroTick.S06.ScmFixture'
            Publisher = 'CN=NevroTick S06 Laboratory'; Version = [version]'1.0.0.0'
            Architecture = 'X64'; InstallLocation = $root
            IsDevelopmentMode = $false; IsPartiallyStaged = $false
        }
        if ((Confirm-S06Registration @($registered) $name) -cne $root) {
            throw 'Registered root did not survive readback'
        }
        Assert-Refused { Confirm-S06Registration @() $name } 'Expected exactly one S0.6 registration in the current user profile'
        Assert-Refused { Confirm-S06Registration @($registered, $registered) $name } 'Expected exactly one S0.6 registration in the current user profile'
        $registered.PackageFullName = 'NevroTick.S06.ScmFixture_1.0.1.0_x64__0123456789abc'
        Assert-Refused { Confirm-S06Registration @($registered) $name } 'S0.6 registration differs from the approved package identity'
        $registered.PackageFullName = $name
        $registered.IsDevelopmentMode = $true
        Assert-Refused { Confirm-S06Registration @($registered) $name } 'S0.6 registration differs from the approved package identity'
        $registered.IsDevelopmentMode = $false
        $registered.IsPartiallyStaged = $true
        Assert-Refused { Confirm-S06Registration @($registered) $name } 'S0.6 registration differs from the approved package identity'
        $registered.IsPartiallyStaged = $false
        $registered.InstallLocation = "C:\Package Volume\..\Package Volume\$name"
        Assert-Refused { Confirm-S06Registration @($registered) $name } 'S0.6 registration does not supply an admitted installed root'
        if ((Confirm-S06Registration @([pscustomobject]@{
            PackageFullName = $name; Name = 'NevroTick.S06.ScmFixture'
            Publisher = 'CN=NevroTick S06 Laboratory'; Version = [version]'1.0.0.0'
            Architecture = 'X64'; InstallLocation = $root
            IsDevelopmentMode = $false; IsPartiallyStaged = $false
        }) $name) -cne $root) { throw 'Healthy registration did not recover' }

        $service = [pscustomobject]@{
            Name = $key; State = 'Stopped'; StartMode = 'Disabled'; ProcessId = 0
            PathName = '"' + $expectedPath + '"'
        }
        if ((Confirm-S06StoppedService @($service) $key $root $member) -cne $expectedPath) {
            throw 'Quoted SCM path lost the exact registered role'
        }
        $service.PathName = $expectedPath
        if ((Confirm-S06StoppedService @($service) $key $root $member) -cne $expectedPath) {
            throw 'Unquoted SCM path lost the exact registered role'
        }
        foreach ($path in @(
            "C:\Copied Out\$name\$member",
            "$root\nevrotick-lab-s02-broker.exe",
            ('"' + $expectedPath + '" --extra'),
            ("$root\" + [char]0x00ad + $member),
            "%ProgramFiles%\$name\$member"
        )) {
            $service.PathName = $path
            Assert-Refused { Confirm-S06StoppedService @($service) $key $root $member } 'SCM path differs from the fixed installed member'
        }
        $service.PathName = $expectedPath
        $service.State = 'Running'
        Assert-Refused { Confirm-S06StoppedService @($service) $key $root $member } 'S0.6 service must be disabled and stopped at this observation'

        $private = Join-Path ([IO.Path]::GetTempPath()) ('NevroTick-s06-image-control-' + [guid]::NewGuid().ToString('N'))
        $null = [IO.Directory]::CreateDirectory($private)
        try {
            $path = Join-Path $private 'exact.bin'
            $digest = 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'
            [IO.File]::WriteAllBytes($path, [byte[]](97,98,99))
            Assert-ControlImage $path 3 $digest
            Assert-Refused { Assert-ControlImage ($private + '\.\exact.bin') 3 $digest } 'Opened image does not resolve to the registered package member'
            Assert-Refused { Assert-ControlImage $path 2 $digest } 'Installed image extent differs from the approved member'
            [IO.File]::WriteAllBytes($path, [byte[]](97,98,100))
            Assert-Refused { Assert-ControlImage $path 3 $digest } 'Installed image bytes differ from the approved member'
            [IO.File]::WriteAllBytes($path, [byte[]](97,98,99))
            Assert-ControlImage $path 3 $digest

            # Replace only the module's external observations. Readback, hashing,
            # identity checks and Windows sharing still act on owned native files.
            $imageRoot = Join-Path $private $name
            $null = [IO.Directory]::CreateDirectory($imageRoot)
            $engine = Join-Path $imageRoot $member
            $broker = Join-Path $imageRoot 'nevrotick-lab-s02-broker.exe'
            $engineKey = 'NevroTick-Lab-S02-Engine'
            $brokerKey = 'NevroTick-Lab-S02-Broker'
            $brokerDigest = 'cb8379ac2098aa165029e3938a51da0bcecfc008fd6795f401178647f96c5b34'
            [IO.File]::WriteAllBytes($engine, [byte[]](97,98,99))
            [IO.File]::WriteAllBytes($broker, [byte[]](100,101,102))
            $registered.InstallLocation = $imageRoot
            $reads = [Collections.Generic.List[string]]::new()
            $script:s06ImageControl = [pscustomobject]@{
                Registration = $registered; Reads = $reads; ChangeRoot = $false
                AlternateRoot = Join-Path (Join-Path $private 'successor') $name
                Paths = @{ $engineKey = $engine; $brokerKey = $broker }
            }
            function script:Assert-S06WritersDenied {
                foreach ($roleKey in @('NevroTick-Lab-S02-Engine', 'NevroTick-Lab-S02-Broker')) {
                    try {
                        $writer = [IO.File]::Open($script:s06ImageControl.Paths[$roleKey],
                            [IO.FileMode]::Open, [IO.FileAccess]::Write, [IO.FileShare]::ReadWrite)
                    } catch {
                        $cause = $_.Exception
                        while ($null -ne $cause.InnerException -and $cause -isnot [IO.IOException]) {
                            $cause = $cause.InnerException
                        }
                        if ($cause -is [IO.IOException] -and ($cause.HResult -band 0xffff) -eq 32) {
                            continue
                        }
                        throw
                    }
                    try {
                        $writer.WriteByte(120)
                    } finally {
                        $writer.Dispose()
                    }
                    throw "S0.6 sharing control unexpectedly opened a writer for $roleKey"
                }
            }
            function script:Read-S06Registration([string]$FullName) {
                $control = $script:s06ImageControl
                $control.Reads.Add('registration')
                if ($control.Reads.Count -eq 6) {
                    Assert-S06WritersDenied
                    if ($control.ChangeRoot) { $control.Registration.InstallLocation = $control.AlternateRoot }
                }
                Confirm-S06Registration @($control.Registration) $FullName
            }
            function script:Read-S06StoppedService([string]$Key, [string]$Root, [string]$Member) {
                $control = $script:s06ImageControl
                $control.Reads.Add($Key)
                if ($control.Reads.Count -eq 4) { Assert-S06WritersDenied }
                $row = [pscustomobject]@{
                    Name = $Key; State = 'Stopped'; StartMode = 'Disabled'; ProcessId = 0
                    PathName = $control.Paths[$Key]
                }
                Confirm-S06StoppedService @($row) $Key $Root $Member
            }

            $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
            try { $deploymentSid = $identity.User.Value } finally { $identity.Dispose() }
            $inputs = @{
                RunId = [guid]'8381e9c5-0c17-4a99-8b47-d50bdf18d205'; ExpectedUserSid = $deploymentSid
                PackageFullName = $name; SignedPackageSha256 = ('1' * 64)
                EngineBytes = 3; EngineSha256 = $digest; BrokerBytes = 3; BrokerSha256 = $brokerDigest
            }
            $receipt = @(Read-S06InstalledIdentity @inputs)
            if (($reads -join ',') -cne ('registration,' + "$engineKey,$brokerKey,$engineKey,$brokerKey,registration") -or
                $receipt.Count -ne 1 -or $receipt[0].schema_version -ne 2 -or
                $receipt[0].outcome -cne 'observed-installed-images' -or $receipt[0].roles.Count -ne 2 -or
                $receipt[0].roles[0].sha256 -cne $digest -or $receipt[0].roles[1].sha256 -cne $brokerDigest) {
                throw 'Healthy companion observation did not emit its exact identity record'
            }
            [IO.File]::WriteAllBytes($engine, [byte[]](97,98,99))
            [IO.File]::WriteAllBytes($broker, [byte[]](100,101,102))

            $reads.Clear()
            $inputs.BrokerSha256 = ('0' * 64)
            Assert-Refused { Read-S06InstalledIdentity @inputs } 'Installed image bytes differ from the approved member'
            if (($reads -join ',') -cne "registration,$engineKey,$brokerKey") {
                throw 'Bad broker image did not stop before either recheck'
            }
            [IO.File]::WriteAllBytes($engine, [byte[]](97,98,99))
            [IO.File]::WriteAllBytes($broker, [byte[]](100,101,102))

            $reads.Clear()
            $inputs.BrokerSha256 = $brokerDigest
            $script:s06ImageControl.ChangeRoot = $true
            Assert-Refused { Read-S06InstalledIdentity @inputs } 'Installed registration root changed during readback'
            [IO.File]::WriteAllBytes($engine, [byte[]](97,98,99))
            [IO.File]::WriteAllBytes($broker, [byte[]](100,101,102))
        } finally {
            [IO.Directory]::Delete($private, $true)
        }
        's06-installed-identity controls=passed'
    }
} finally {
    Remove-Module $module
}
