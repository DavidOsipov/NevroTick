#requires -Version 5.1
<#
Install the selected stable Rust tools for the current Windows user.
No persistent PATH, service, clock, trust-store or execution-policy changes.
Linking is configured separately through the selected LLVM+xwin route.
This script installs no Visual Studio or system Windows SDK packages.

Sources:
https://static.rust-lang.org/rustup/release-stable.toml
https://static.rust-lang.org/dist/channel-rust-stable.toml
https://rust-lang.github.io/rustup/installation/index.html
https://learn.microsoft.com/en-us/visualstudio/install/workload-component-id-vs-build-tools?view=visualstudio
#>
[CmdletBinding()]
param(
    [ValidatePattern('^\d+\.\d+\.\d+$')]
    [string]$RustVersion = '1.99.1',
    [ValidatePattern('^\d+\.\d+\.\d+$')]
    [string]$RustupVersion = '1.29.2',
    [ValidatePattern('^[a-fA-F0-9]{64}$')]
    [string]$RustupSha256 = '00f139da614dced87d3c3d4121c43eebdac3c1bf8a5ee86b9d80dfd71c8d70fd'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
if ($env:OS -ne 'Windows_NT' -or $env:PROCESSOR_ARCHITECTURE -ne 'AMD64') {
    throw 'Run this script in native x64 Windows PowerShell.'
}
if ($env:CARGO_HOME -or $env:RUSTUP_HOME) {
    throw 'This per-user setup expects default Cargo/Rustup homes. Review existing home overrides first.'
}
foreach ($name in @('RUSTUP_DIST_SERVER', 'RUSTUP_UPDATE_ROOT', 'RUSTUP_TOOLCHAIN')) {
    if ([Environment]::GetEnvironmentVariable($name, 'Process')) {
        throw "Review the existing $name override before installing pinned Rust tools."
    }
}

function Invoke-Checked {
    param([string]$Executable, [string[]]$Arguments)
    & $Executable @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Executable exited with code $LASTEXITCODE."
    }
}

function Get-WebText {
    param([string]$Uri)
    $content = (Invoke-WebRequest -UseBasicParsing -Uri $Uri).Content
    # Windows PowerShell returns byte[] for Rust's octet-stream manifests.
    if ($content -is [byte[]]) {
        return [Text.Encoding]::UTF8.GetString($content)
    }
    return [string]$content
}

$userPathBefore = [Environment]::GetEnvironmentVariable('Path', 'User')
$machinePathBefore = [Environment]::GetEnvironmentVariable('Path', 'Machine')
$hostTarget = 'x86_64-pc-windows-msvc'
$selectedToolchain = "$RustVersion-$hostTarget"
$cargoBin = Join-Path $env:USERPROFILE '.cargo\bin'
$rustup = Join-Path $cargoBin 'rustup.exe'
$newRustup = -not (Test-Path -LiteralPath $rustup)

$installRustup = $newRustup
if (-not $newRustup) {
    $versionOutput = & $rustup --version
    if ($LASTEXITCODE -ne 0) { throw 'Existing Rustup could not report its version.' }
    $installRustup = -not (($versionOutput | Select-Object -First 1) -match "^rustup $([regex]::Escape($RustupVersion))\s")
}
if ($installRustup) {
    $cache = Join-Path $env:LOCALAPPDATA "NevroTick\setup-cache\rustup-$RustupVersion"
    New-Item -ItemType Directory -Path $cache -Force | Out-Null
    $installer = Join-Path $cache 'rustup-init.exe'
    $url = "https://static.rust-lang.org/rustup/archive/$RustupVersion/$hostTarget/rustup-init.exe"
    $published = ((Get-WebText ($url + '.sha256')).Trim() -split '\s+')[0]
    if ($published -ne $RustupSha256) {
        throw 'Published Rustup SHA256 differs from the selected digest. Review version and checksum together.'
    }
    if (-not (Test-Path -LiteralPath $installer) -or (Get-FileHash -Algorithm SHA256 $installer).Hash -ne $RustupSha256) {
        Invoke-WebRequest -UseBasicParsing -Uri $url -OutFile $installer
    }
    if ((Get-FileHash -Algorithm SHA256 $installer).Hash -ne $RustupSha256) {
        throw 'Rustup installer SHA256 mismatch; installer was not executed.'
    }
    Write-Host "Verified Rustup installer SHA256: $RustupSha256"
    # -y is Rustup's supported noninteractive mode. Without MSVC it warns;
    # it neither prompts for nor installs Visual Studio. No test-only skip
    # environment variable is used. Rust installation alone does not qualify linking.
    # Preserve any existing default toolchain; a fresh install gets its default below.
    Invoke-Checked $installer @('-y', '--no-modify-path', '--default-host', $hostTarget, '--default-toolchain', 'none', '--profile', 'minimal')
}

Invoke-Checked $rustup @('toolchain', 'install', $selectedToolchain, '--no-self-update', '--profile', 'minimal', '--component', 'rustfmt', '--component', 'clippy', '--component', 'rust-analyzer', '--component', 'rust-src', '--component', 'llvm-tools-preview')
if ($newRustup) {
    Invoke-Checked $rustup @('default', $selectedToolchain)
}
Invoke-Checked $rustup @('run', $selectedToolchain, 'rustc', '-vV')
Invoke-Checked $rustup @('run', $selectedToolchain, 'cargo', '--version')
Invoke-Checked $rustup @('run', $selectedToolchain, 'rustfmt', '--version')
Invoke-Checked $rustup @('run', $selectedToolchain, 'cargo', 'clippy', '--version')
Invoke-Checked $rustup @('run', $selectedToolchain, 'rust-analyzer', '--version')
Invoke-Checked $rustup @('component', 'list', '--toolchain', $selectedToolchain, '--installed')
Invoke-Checked $rustup @('target', 'list', '--toolchain', $selectedToolchain, '--installed')

if ($userPathBefore -cne [Environment]::GetEnvironmentVariable('Path', 'User') -or
    $machinePathBefore -cne [Environment]::GetEnvironmentVariable('Path', 'Machine')) {
    throw 'Persistent PATH changed during setup; inspect the change before continuing.'
}
Write-Host "Rust tools: $cargoBin (persistent PATH unchanged)."
Write-Host "Use explicit paths or add '$cargoBin' to PATH in the current shell only."
Write-Host 'LLVM+xwin linking and an actual Windows build/run must be verified separately; tool installation alone is not native qualification.'
