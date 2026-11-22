# SPDX-License-Identifier: GPL-2.0-only
# Local path and read-only file controls. No AppX, SCM or installation operations.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$module = Import-Module (Join-Path $PSScriptRoot 'installed_identity.psm1') -Force -PassThru
try {
    & $module {
        function Assert-Refused([scriptblock]$Action, [string]$Expected) {
            try {
                $null = & $Action
            } catch {
                if ($_.Exception.Message -ceq $Expected) { return }
                throw
            }
            throw "Expected refusal: $Expected"
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
            Confirm-S06Image $path 3 $digest
            Assert-Refused { Confirm-S06Image ($private + '\.\exact.bin') 3 $digest } 'Opened image does not resolve to the registered package member'
            Assert-Refused { Confirm-S06Image $path 2 $digest } 'Installed image extent differs from the approved member'
            [IO.File]::WriteAllBytes($path, [byte[]](97,98,100))
            Assert-Refused { Confirm-S06Image $path 3 $digest } 'Installed image bytes differ from the approved member'
            [IO.File]::WriteAllBytes($path, [byte[]](97,98,99))
            Confirm-S06Image $path 3 $digest
        } finally {
            [IO.Directory]::Delete($private, $true)
        }
        's06-installed-identity controls=passed'
    }
} finally {
    Remove-Module $module
}
