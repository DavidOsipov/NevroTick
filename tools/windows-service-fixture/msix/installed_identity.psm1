# SPDX-License-Identifier: GPL-2.0-only
# Read-only companion for the held S0.6 inspection of disabled services.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-S06ImagePath {
    param([string]$Root, [string]$Member, [string]$StoredPath)

    # Compare the admitted literal shapes. The SCM string never becomes a file input.
    $expected = [IO.Path]::Combine($Root, $Member)
    if (![string]::Equals($StoredPath, $expected, [StringComparison]::Ordinal) -and
        ![string]::Equals($StoredPath, ('"' + $expected + '"'), [StringComparison]::Ordinal)) {
        throw 'SCM path differs from the fixed installed member'
    }
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
    param([object[]]$Services, [string]$Key, [string]$Root, [string]$Member)

    if ($Services.Count -ne 1 -or ![string]::Equals($Services[0].Name, $Key, [StringComparison]::Ordinal)) {
        throw 'Expected one service at the fixed S0.6 SCM key'
    }
    $service = $Services[0]
    if (![string]::Equals($service.State, 'Stopped', [StringComparison]::Ordinal) -or
        ![string]::Equals($service.StartMode, 'Disabled', [StringComparison]::Ordinal) -or
        $service.ProcessId -ne 0) {
        throw 'S0.6 service must be disabled and stopped at this observation'
    }
    return (Resolve-S06ImagePath $Root $Member $service.PathName)
}

function Read-S06StoppedService {
    param([string]$Key, [string]$Root, [string]$Member)

    # Key originates in the two fixed source literals below.
    $services = @(CimCmdlets\Get-CimInstance -ClassName Win32_Service -Filter "Name='$Key'" |
        Select-Object -First 2)
    return (Confirm-S06StoppedService $services $Key $Root $Member)
}

function Read-S06ImageFinalPath {
    param([Microsoft.Win32.SafeHandles.SafeFileHandle]$Handle)

    if ($null -eq ('NevroTick.S06ImagePath' -as [type])) {
        Add-Type -Namespace NevroTick -Name S06ImagePath -MemberDefinition @'
[System.Runtime.InteropServices.DllImport("kernel32.dll", CharSet = System.Runtime.InteropServices.CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
private static extern uint GetFinalPathNameByHandleW(
    Microsoft.Win32.SafeHandles.SafeFileHandle handle,
    System.Text.StringBuilder path, uint capacity, uint flags);

public static string Read(Microsoft.Win32.SafeHandles.SafeFileHandle handle)
{
    var path = new System.Text.StringBuilder(1024);
    uint count = GetFinalPathNameByHandleW(handle, path, (uint)path.Capacity, 0);
    if (count == 0)
    {
        int error = System.Runtime.InteropServices.Marshal.GetLastWin32Error();
        throw new System.ComponentModel.Win32Exception(error);
    }
    if (count >= path.Capacity || count != path.Length)
        throw new System.IO.InvalidDataException("Final image path exceeds its output bound");
    return path.ToString();
}
'@
    }
    return [NevroTick.S06ImagePath]::Read($Handle)
}

function Confirm-S06Image {
    param([string]$Path, [long]$Bytes, [string]$Digest)

    if ($Bytes -le 0 -or $Bytes -gt 2097152) {
        throw 'Expected image length exceeds its read bound'
    }
    # The open handle excludes writes/deletion and supplies both path and contents.
    $file = [IO.File]::Open($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        if (![string]::Equals((Read-S06ImageFinalPath $file.SafeFileHandle), ('\\?\' + $Path), [StringComparison]::Ordinal)) {
            throw 'Opened image does not resolve to the registered package member'
        }
        if ($file.Length -ne $Bytes) {
            throw 'Installed image extent differs from the approved member'
        }
        $actual = (Microsoft.PowerShell.Utility\Get-FileHash -InputStream $file -Algorithm SHA256).Hash
        if ($file.Position -ne $Bytes -or $file.Length -ne $Bytes -or
            ![string]::Equals($actual, $Digest, [StringComparison]::OrdinalIgnoreCase)) {
            throw 'Installed image bytes differ from the approved member'
        }
    } finally {
        $file.Dispose()
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
    foreach ($role in $roles) {
        $path = Read-S06StoppedService $role.key $root $role.member
        Confirm-S06Image $path $role.bytes $role.sha256
    }
    foreach ($role in $roles) {
        $null = Read-S06StoppedService $role.key $root $role.member
    }
    if (![string]::Equals((Read-S06Registration $PackageFullName), $root, [StringComparison]::Ordinal)) {
        throw 'Installed registration root changed during readback'
    }

    # The trusted supervisor binds these expected hashes to the signed input.
    [pscustomobject]@{
        schema_version = 1
        outcome = 'observed-installed-images'
        run_id = $RunId.ToString('D')
        signed_package_sha256 = $SignedPackageSha256
        roles = $roles
    }
}

Export-ModuleMember -Function Read-S06InstalledIdentity
