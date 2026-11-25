# SPDX-License-Identifier: GPL-2.0-only
# Read-only companion for the held S0.6 inspection of disabled services.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# The supervisor owns and holds the verified sibling through child retirement.
[type]$script:s06ImagePathType = & {
    [string]$path = [IO.Path]::Combine($PSScriptRoot, 'prestart_image_path.dll')
    [IO.FileInfo]$file = [IO.FileInfo]::new($path)
    if (!$file.Exists -or $file.Length -lt 1 -or $file.Length -gt 65536) {
        throw 'Installed identity requires the fixed precompiled path helper'
    }

    # Load this file without reusing a same-name assembly from another path.
    [Reflection.Assembly]$assembly = [Reflection.Assembly]::LoadFile($path)
    if ($assembly.GetName().Name -cne 'prestart_image_path' -or
        ![string]::Equals($assembly.Location, $path, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'Installed identity loaded an unexpected path helper assembly'
    }
    [type]$reader = $assembly.GetType('NevroTick.S06ImagePath', $false, $false)
    if ($null -eq $reader -or ![object]::ReferenceEquals($reader.Assembly, $assembly) -or
        !$reader.IsPublic -or !$reader.IsAbstract -or !$reader.IsSealed) {
        throw 'Installed identity path helper has an unexpected type'
    }

    [Reflection.BindingFlags]$flags = 'Public, Static, DeclaredOnly'
    [Reflection.MethodInfo]$read = $reader.GetMethod('Read', $flags)
    if ($null -eq $read -or $read.IsGenericMethod -or $read.ReturnType -ne [string]) {
        throw 'Installed identity path helper has an unexpected Read method'
    }
    [Reflection.ParameterInfo[]]$parameters = $read.GetParameters()
    if ($parameters.Length -ne 1 -or $parameters[0].ParameterType -ne [Microsoft.Win32.SafeHandles.SafeFileHandle]) {
        throw 'Installed identity path helper has an unexpected Read method'
    }
    return $reader
}

function Resolve-S06ImagePath {
    param([string]$Root, [string]$Member, [string]$StoredPath, [string]$PackageFullName)

    # Compare the admitted literal shapes. The SCM string never becomes a file input.
    [string]$expected = [IO.Path]::Combine($Root, $Member)
    if ([string]::Equals($StoredPath, $expected, [StringComparison]::Ordinal) -or
        [string]::Equals($StoredPath, ('"' + $expected + '"'), [StringComparison]::Ordinal)) {
        return $expected
    }
    if ([string]::IsNullOrEmpty($PackageFullName) -or
        ![string]::Equals([IO.Path]::GetFileName($Root), $PackageFullName, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'SCM path differs from the fixed installed member'
    }

    # Appx can report the directory in lowercase while SCM quotes the full
    # package identity spelling. Prove that this derived alias resolves to
    # the registered member even on a case-sensitive volume.
    [string]$identityRoot = [IO.Path]::Combine([IO.Path]::GetDirectoryName($Root), $PackageFullName)
    [string]$identityMember = [IO.Path]::Combine($identityRoot, $Member)
    if (![string]::Equals($StoredPath, ('"' + $identityMember + '"'), [StringComparison]::Ordinal)) {
        throw 'SCM path differs from the fixed installed member'
    }
    [IO.FileStream]$aliasFile = [IO.File]::Open($identityMember, [IO.FileMode]::Open,
        [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try { Confirm-S06ImagePath $aliasFile $expected }
    finally { $aliasFile.Dispose() }
    return $expected
}

function Confirm-S06Registration {
    param([object[]]$Packages, [string]$FullName)

    if ($Packages.Count -ne 1) {
        throw 'Expected exactly one S0.6 registration in the current user profile'
    }
    $package = $Packages[0]
    if ($package.IsDevelopmentMode -isnot [bool] -or $package.IsDevelopmentMode -or
        $package.IsPartiallyStaged -isnot [bool] -or $package.IsPartiallyStaged -or
        ![string]::Equals($package.PackageFullName, $FullName, [StringComparison]::OrdinalIgnoreCase) -or
        ![string]::Equals($package.Name, 'NevroTick.S06.ScmFixture', [StringComparison]::Ordinal) -or
        ![string]::Equals($package.Publisher, 'CN=NevroTick S06 Laboratory', [StringComparison]::Ordinal) -or
        ![string]::Equals($package.Version.ToString(), '1.0.0.0', [StringComparison]::Ordinal) -or
        ![string]::Equals($package.Architecture.ToString(), 'X64', [StringComparison]::OrdinalIgnoreCase)) {
        throw 'S0.6 registration differs from the approved package identity'
    }
    $root = [string]$package.InstallLocation
    if ($root.Length -gt 512 -or $root -cnotmatch '\A[A-Za-z]:\\[^\x00-\x1f"*?<>|:]+\z' -or
        ![string]::Equals($root, [IO.Path]::GetFullPath($root), [StringComparison]::Ordinal) -or
        ![string]::Equals([IO.Path]::GetFileName($root), $FullName, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'S0.6 registration does not supply an admitted installed root'
    }
    return $root
}

function Read-S06Registration {
    param([string]$FullName)

    $packages = @(Appx\Get-AppxPackage -Name 'NevroTick.S06.ScmFixture' -PackageTypeFilter Main |
        Select-Object -First 2)
    return (Confirm-S06Registration $packages $FullName)
}

function Confirm-S06StoppedService {
    param([object[]]$Services, [string]$Key, [string]$Root, [string]$Member, [string]$PackageFullName)

    if ($Services.Count -ne 1 -or ![string]::Equals($Services[0].Name, $Key, [StringComparison]::Ordinal)) {
        throw 'Expected one service at the fixed S0.6 SCM key'
    }
    $service = $Services[0]
    if (![string]::Equals($service.State, 'Stopped', [StringComparison]::Ordinal) -or
        ![string]::Equals($service.StartMode, 'Disabled', [StringComparison]::Ordinal) -or
        $service.ProcessId -ne 0) {
        throw 'S0.6 service must be disabled and stopped at this observation'
    }
    return (Resolve-S06ImagePath $Root $Member $service.PathName $PackageFullName)
}

function Read-S06StoppedService {
    param([string]$Key, [string]$Root, [string]$Member, [string]$PackageFullName)

    # Key originates in the two fixed source literals below.
    $services = @(CimCmdlets\Get-CimInstance -ClassName Win32_Service -Filter "Name='$Key'" |
        Select-Object -First 2)
    return (Confirm-S06StoppedService $services $Key $Root $Member $PackageFullName)
}

function Read-S06ImageFinalPath {
    param([Microsoft.Win32.SafeHandles.SafeFileHandle]$Handle)

    [type]$reader = $script:s06ImagePathType
    return $reader::Read($Handle)
}

function Confirm-S06ImagePath {
    param([IO.FileStream]$File, [string]$Path)

    if (![string]::Equals((Read-S06ImageFinalPath $File.SafeFileHandle), ('\\?\' + $Path), [StringComparison]::Ordinal)) {
        throw 'Opened image does not resolve to the registered package member'
    }
}

function Confirm-S06Image {
    param([IO.FileStream]$File, [string]$Path, [long]$Bytes, [string]$Digest)

    if ($Bytes -le 0 -or $Bytes -gt 2097152) {
        throw 'Expected image length exceeds its read bound'
    }
    Confirm-S06ImagePath $File $Path
    if ($File.Length -ne $Bytes) {
        throw 'Installed image extent differs from the approved member'
    }
    $actual = (Microsoft.PowerShell.Utility\Get-FileHash -InputStream $File -Algorithm SHA256).Hash
    if ($File.Position -ne $Bytes -or $File.Length -ne $Bytes -or
        ![string]::Equals($actual, $Digest, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'Installed image bytes differ from the approved member'
    }
}

function Read-S06InstalledIdentity {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][guid]$RunId,
        [Parameter(Mandatory)][ValidatePattern('\ANevroTick\.S06\.ScmFixture_1\.0\.0\.0_x64__[0-9a-z]{13}\z')]
        [string]$PackageFullName,
        [Parameter(Mandatory)][ValidatePattern('\AS-1-[0-9]{1,10}(?:-[0-9]{1,10}){1,14}\z')]
        [string]$ExpectedUserSid,
        [Parameter(Mandatory)][ValidatePattern('\A[a-f0-9]{64}\z')][string]$SignedPackageSha256,
        [Parameter(Mandatory)][ValidateRange(1,2097152)][long]$EngineBytes,
        [Parameter(Mandatory)][ValidatePattern('\A[a-f0-9]{64}\z')][string]$EngineSha256,
        [Parameter(Mandatory)][ValidateRange(1,2097152)][long]$BrokerBytes,
        [Parameter(Mandatory)][ValidatePattern('\A[a-f0-9]{64}\z')][string]$BrokerSha256
    )

    if ($RunId -eq [guid]::Empty) {
        throw 'Installed observation requires a nonempty supervisor run ID'
    }
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    try {
        if (![string]::Equals($identity.User.Value, $ExpectedUserSid, [StringComparison]::Ordinal)) {
            throw 'Current user differs from the admitted deployment subject'
        }
    } finally {
        $identity.Dispose()
    }

    $root = Read-S06Registration $PackageFullName
    $roles = @(
        @{ key = 'NevroTick-Lab-S02-Engine'; member = 'nevrotick-lab-s02-engine.exe'; bytes = $EngineBytes; sha256 = $EngineSha256 },
        @{ key = 'NevroTick-Lab-S02-Broker'; member = 'nevrotick-lab-s02-broker.exe'; bytes = $BrokerBytes; sha256 = $BrokerSha256 }
    )
    $paths = [string[]]::new($roles.Count)
    $files = [IO.FileStream[]]::new($roles.Count)
    $failure = $null
    $releaseFailures = [Collections.Generic.List[Management.Automation.ErrorRecord]]::new()
    try {
        for ($index = 0; $index -lt $roles.Count; $index++) {
            $role = $roles[$index]
            $paths[$index] = Read-S06StoppedService $role.key $root $role.member $PackageFullName
            # Exclude writes and deletion through both service and registration rechecks.
            $files[$index] = [IO.File]::Open($paths[$index], [IO.FileMode]::Open,
                [IO.FileAccess]::Read, [IO.FileShare]::Read)
            Confirm-S06Image $files[$index] $paths[$index] $role.bytes $role.sha256
        }
        foreach ($role in $roles) {
            $null = Read-S06StoppedService $role.key $root $role.member $PackageFullName
        }
        if (![string]::Equals((Read-S06Registration $PackageFullName), $root, [StringComparison]::Ordinal)) {
            throw 'Installed registration root changed during readback'
        }
        for ($index = 0; $index -lt $roles.Count; $index++) {
            Confirm-S06ImagePath $files[$index] $paths[$index]
        }
    } catch {
        $failure = $_
        throw
    } finally {
        foreach ($file in $files) {
            if ($null -eq $file) { continue }
            try {
                $file.Dispose()
            } catch {
                $releaseFailures.Add($_)
            }
        }
        if ($releaseFailures.Count -gt 0) {
            if ($null -ne $failure) {
                $failure.Exception.Data['S06ImageReleaseFailures'] = $releaseFailures.ToArray()
            } else {
                if ($releaseFailures.Count -gt 1) {
                    $releaseFailures[0].Exception.Data['S06ImageReleaseFailures'] = @($releaseFailures[1])
                }
                throw $releaseFailures[0]
            }
        }
    }

    # The trusted supervisor binds these expected hashes to the signed input.
    [pscustomobject]@{
        schema_version = 2
        outcome = 'observed-installed-images'
        run_id = $RunId.ToString('D')
        signed_package_sha256 = $SignedPackageSha256
        package_full_name = $PackageFullName
        roles = $roles
    }
}

# Bind one identity read to a raw stream owned by the caller. Finish the
# readback and encode the entire record before writing any success bytes.
function Write-S06InstalledIdentity {
    [CmdletBinding(PositionalBinding = $false)]
    param(
        [Parameter(Mandatory)][hashtable]$Inputs,
        [Parameter(Mandatory)][IO.Stream]$OutputStream
    )

    [string[]]$fields = @('RunId', 'PackageFullName', 'ExpectedUserSid', 'SignedPackageSha256',
        'EngineBytes', 'EngineSha256', 'BrokerBytes', 'BrokerSha256')
    if ($Inputs.Count -ne $fields.Count -or !$OutputStream.CanWrite) {
        throw 'Installed identity requires an exact request and a writable output stream'
    }
    foreach ($field in $fields) {
        if (!$Inputs.ContainsKey($field)) {
            throw 'Installed identity requires an exact request'
        }
    }

    [object[]]$records = @(Read-S06InstalledIdentity @Inputs)
    if ($records.Count -ne 1 -or $records[0] -isnot [pscustomobject]) {
        throw 'Installed identity did not return one record'
    }
    $record = $records[0]
    if (@($record.PSObject.Properties).Count -ne 6 -or $record.schema_version -isnot [int] -or
        $record.schema_version -ne 2 -or $record.outcome -cne 'observed-installed-images' -or
        $record.run_id -cne ([guid]$Inputs.RunId).ToString('D') -or
        $record.signed_package_sha256 -cne $Inputs.SignedPackageSha256 -or
        $record.package_full_name -cne $Inputs.PackageFullName -or
        $record.roles -isnot [array] -or $record.roles.Count -ne 2) {
        throw 'Installed identity did not return a complete schema-2 record'
    }
    foreach ($role in $record.roles) {
        if ($role -isnot [hashtable] -or $role.Count -ne 4 -or
            [string]::IsNullOrEmpty($role.key) -or [string]::IsNullOrEmpty($role.member) -or
            $role.bytes -isnot [long] -or $role.bytes -lt 1 -or $role.bytes -gt 2097152 -or
            $role.sha256 -cnotmatch '\A[a-f0-9]{64}\z') {
            throw 'Installed identity did not return a complete schema-2 role'
        }
    }

    [Text.UTF8Encoding]$encoding = [Text.UTF8Encoding]::new($false, $true)
    [string]$json = Microsoft.PowerShell.Utility\ConvertTo-Json -InputObject $record -Depth 4 -Compress -ErrorAction Stop -WarningAction Stop
    [string]$line = $json + "`n"
    if ($encoding.GetByteCount($line) -gt 8192) {
        throw 'Installed identity record exceeds the output bound'
    }
    [byte[]]$bytes = $encoding.GetBytes($line)
    $OutputStream.Write($bytes, 0, $bytes.Length)
    $OutputStream.Flush()
}

Export-ModuleMember -Function Read-S06InstalledIdentity, Write-S06InstalledIdentity
