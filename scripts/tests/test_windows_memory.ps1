#requires -Version 5.1
<# Native, non-elevated helper controls. No IFEO writes or diagnostic tools run. #>
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT' -or [IntPtr]::Size -ne 8) { throw 'Use native x64 Windows PowerShell.' }
$source = Join-Path $PSScriptRoot '..\windows-memory.ps1'
$tokens = $null; $errors = $null
$ast = [Management.Automation.Language.Parser]::ParseFile($source, [ref]$tokens, [ref]$errors)
if (@($errors).Count) { throw ($errors | Out-String) }
# Import authored function definitions only. The script's mutation entry point
# is never evaluated; each native mutation is replaced below where needed.
foreach ($function in $ast.FindAll({ param($n) $n -is [Management.Automation.Language.FunctionDefinitionAst] }, $false)) {
    Invoke-Expression $function.Extent.Text
}
$script:ImagePattern = '^NevroTick-[a-f0-9]{12,64}\.exe$'
$script:IfeoPath = 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options'
$script:RegistryProfile = (Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot '..\..\policy\windows-memory.json') | ConvertFrom-Json).profile
$script:DebuggerConfigHash = (Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot '..\..\policy\windows-memory.json') | ConvertFrom-Json).debugger_config_sha256
$script:Passed = 0

function Check {
    param([string]$Name, [scriptblock]$Body)
    & $Body
    $script:Passed++
    Write-Output "PASS $Name"
}
function Reject {
    param([scriptblock]$Body, [string]$Contains)
    try { & $Body; throw 'EXPECTED_REJECTION_MISSING' }
    catch {
        if ($_.Exception.Message -eq 'EXPECTED_REJECTION_MISSING' -or $_.Exception.Message -notlike ('*' + $Contains + '*')) { throw }
    }
}
function Assert-True {
    param([bool]$Condition)
    if (-not $Condition) { throw 'Expected condition was false.' }
}
function Empty-State {
    return [ordered]@{
        Registry64 = [ordered]@{ exists = $false; values = @(); children = @() }
        Registry32 = [ordered]@{ exists = $false; values = @(); children = @() }
    }
}
function Sample-Policy {
    return [pscustomobject]@{
        debugger_config_sha256 = $script:DebuggerConfigHash
        debugger_config_location = 'current-user:LocalApplicationData/DBG/config.xml'
        schema = 1; status = 'admitted'; profile = [pscustomobject]@{
            page_heap = 'full'; application_verifier = @('Basics'); image_name_pattern = $script:ImagePattern; tool_timeout_seconds = 30
            registry_provider_keys = $script:RegistryProfile.registry_provider_keys
            registry_after_step = $script:RegistryProfile.registry_after_step
        }; tools = [pscustomobject]@{
            appverif = [pscustomobject]@{ path = 'C:\Windows\System32\appverif.exe'; sha256 = ('a' * 64); file_version = '1.2.3'; signer_subject = 'O=Microsoft Corporation' }
            gflags = [pscustomobject]@{ path = 'C:\Windows\System32\gflags.exe'; sha256 = ('b' * 64); file_version = '1.2.3'; signer_subject = 'O=Microsoft Corporation' }
            cdb = [pscustomobject]@{ path = 'C:\Windows\System32\cdb.exe'; sha256 = ('c' * 64); file_version = '1.2.3'; signer_subject = 'O=Microsoft Corporation' }
        }
        supporting_libraries = @('dbgcore.dll', 'dbgeng.dll', 'dbghelp.dll', 'vfbasics.dll', 'vfcompat.dll', 'vrfcore.dll') | ForEach-Object { [pscustomobject]@{path = ('C:\Windows\System32\' + $_)} }
    }
}

Check 'canonical local path accepted' { Assert-True ((Assert-LocalPath 'C:\Windows\System32\appverif.exe') -eq 'C:\Windows\System32\appverif.exe') }
Check 'UNC path rejected' { Reject { Assert-LocalPath '\\server\share\file.exe' } 'absolute local' }
Check 'alternate data stream rejected' { Reject { Assert-LocalPath 'C:\file.exe:stream' } 'stream syntax' }
Check 'relative traversal rejected' { Reject { Assert-LocalPath 'C:\Windows\..\file.exe' } 'aliases' }
Check 'trailing-dot alias rejected' { Reject { Assert-LocalPath 'C:\folder.\file.exe' } 'aliases' }
Check 'system image name rejected' { Reject { Get-ImageIdentity 'C:\Windows\System32\cmd.exe' $false } 'unique NevroTick' }
Check 'missing candidate accepted for removal identity' {
    $identity = Get-ImageIdentity 'C:\NevroTick-0123456789ab.exe' $false
    Assert-True ($identity.name -eq 'NevroTick-0123456789ab.exe' -and $null -eq $identity.sha256)
}
Check 'missing candidate rejected for enable' { Reject { Get-ImageIdentity 'C:\NevroTick-0123456789ab.exe' $true } 'missing' }
Check 'admitted policy accepted' { Assert-Policy (Sample-Policy) }
Check 'pending installation rejected' {
    $policy = Sample-Policy; $policy.status = 'pending-installation-admission'
    Reject { Assert-Policy $policy } 'unadmitted'
}
Check 'missing hash rejected' {
    $policy = Sample-Policy; $policy.tools.appverif.sha256 = $null
    Reject { Assert-Policy $policy } 'Missing exact'
}
Check 'missing debugger configuration pin rejected' {
    $policy = Sample-Policy; $policy.PSObject.Properties.Remove('debugger_config_sha256')
    Reject { Assert-Policy $policy } 'debugger configuration admission'
}
Check 'malformed debugger configuration pin rejected' {
    $policy = Sample-Policy; $policy.debugger_config_sha256 = 'optional'
    Reject { Assert-Policy $policy } 'debugger configuration admission'
}
Check 'SDK-directory debugger configuration owner rejected' {
    $policy = Sample-Policy; $policy.debugger_config_location = 'SDK/config.xml'
    Reject { Assert-Policy $policy } 'current user KnownFolder owner'
}
Check 'debugger configuration resolves the current user KnownFolder' {
    $expected = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'DBG\config.xml'
    Assert-True ((Get-UserDebuggerConfigurationPath) -ceq $expected)
}
Check 'weakened heap profile rejected' {
    $policy = Sample-Policy; $policy.profile.page_heap = 'standard'
    Reject { Assert-Policy $policy } 'reviewed profile'
}
Check 'empty existing IFEO key rejected' {
    $state = Empty-State; $state.Registry32.exists = $true
    Reject { Assert-InitiallyAbsent $state } 'Pre-existing'
}
Check 'absent IFEO keys accepted' { Assert-InitiallyAbsent (Empty-State) }
Check 'canonical receipt roundtrip compares equal' {
    $state = Empty-State
    Assert-StateMatch $state ((Convert-State $state) | ConvertFrom-Json)
}
Check 'concurrent configuration mutation rejected' {
    $initial = Empty-State; $changed = Empty-State
    $changed.Registry64.values = @([ordered]@{ name = 'Debugger'; kind = 'String'; data = 'unrelated.exe' })
    Reject { Assert-StateMatch $changed $initial } 'outside the recorded step'
}
Check 'enabled profile requires actual verifier and full-page flags' {
    $state = Get-ExpectedStepState 'enable-pageheap'
    Assert-EnabledProfile $state
    $state.Registry64.values[1].data = '0x2'
    Reject { Assert-EnabledProfile $state } 'not established'
}
Check 'weakened Basics mask rejected' {
    $state = Get-ExpectedStepState 'enable-pageheap'
    $state.Registry64.values[3].data = 1
    Reject { Assert-StepState 'enable-pageheap' $state } 'unadmitted registry state'
}
Check 'extra concurrent registry value rejected before adoption' {
    $state = Get-ExpectedStepState 'enable-appverif'
    $state.Registry64.values += [ordered]@{name = 'Debugger'; kind = 'String'; data = 'unrelated.exe'}
    Reject { Assert-StepState 'enable-appverif' $state } 'not adopted for cleanup'
}
Check 'missing verifier provider rejected' {
    $state = Get-ExpectedStepState 'enable-pageheap'
    $state.Registry64.children = @($state.Registry64.children | Select-Object -Skip 1)
    Reject { Assert-StepState 'enable-pageheap' $state } 'unadmitted registry state'
}
Check 'high-bit DWORD retains its unsigned bit pattern' {
    Assert-True ((Convert-RegistryDword ([ordered]@{kind = 'DWord'; data = [int32]::MinValue})) -eq [uint32]2147483648)
}
Check 'tool arguments reject injection before process launch' {
    Reject { Invoke-BoundedTool 'C:\missing.exe' @('/p', 'name.exe;whoami') } 'Unexpected native tool argument'
}

# File ACL mutation is deliberately excluded from these non-elevated controls.
# Its real owner/access behavior must be observed in the elevated fixture lane.
function Protect-OwnedFile { param([string]$Path) }

$temporary = Join-Path $env:TEMP ('NevroTick-memory-helper-tests-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temporary | Out-Null
try {
    $configuration = Join-Path $temporary 'config.xml'
    Check 'missing installed debugger configuration rejected' {
        Reject { Assert-DebuggerConfiguration $configuration $script:DebuggerConfigHash } 'Missing debugger config.xml'
    }
    $sourceConfiguration = Join-Path $PSScriptRoot '..\..\policy\windows-debugger.xml'
    [IO.File]::Copy($sourceConfiguration, $configuration)
    Check 'admitted debugger configuration accepted' {
        Assert-DebuggerConfiguration $configuration $script:DebuggerConfigHash
    }
    [IO.File]::WriteAllText($configuration, ([IO.File]::ReadAllText($configuration).Replace('Value="false"', 'Value="true"')), (New-Object Text.UTF8Encoding($false)))
    Check 'weakened installed debugger configuration rejected' {
        Reject { Assert-DebuggerConfiguration $configuration $script:DebuggerConfigHash } 'differs from the admitted offline settings'
    }
    $fixture = Join-Path $temporary 'NevroTick-0123456789ab.exe'
    $bytes = New-Object byte[] 128
    $bytes[0] = 0x4d; $bytes[1] = 0x5a; $bytes[0x3c] = 64
    $bytes[64] = 0x50; $bytes[65] = 0x45; $bytes[68] = 0x64; $bytes[69] = 0x86
    [IO.File]::WriteAllBytes($fixture, $bytes)
    Check 'AMD64 fixture identity admitted without executing it' {
        $identity = Get-ImageIdentity $fixture $true
        Assert-True ($identity.sha256 -match '^[a-f0-9]{64}$')
    }
    $bytes[68] = 0x4c; $bytes[69] = 0x01
    [IO.File]::WriteAllBytes($fixture, $bytes)
    Check 'x86 PE rejected' { Reject { Get-ImageIdentity $fixture $true } 'AMD64' }
    $bytes[0x3c] = 255
    [IO.File]::WriteAllBytes($fixture, $bytes)
    Check 'out-of-range PE header rejected' { Reject { Get-ImageIdentity $fixture $true } 'Invalid PE header' }
    Check 'durable receipt replacement retains serialized state' {
        $path = Join-Path $temporary 'test.json'
        Write-Receipt $path ([ordered]@{ status = 'prepared'; expected = (Empty-State) })
        Write-Receipt $path ([ordered]@{ status = 'disabled'; expected = (Empty-State) })
        Assert-True ((Get-Content -Raw -LiteralPath $path | ConvertFrom-Json).status -eq 'disabled')
    }
    Check 'native lock handle can set its ACL and excludes another opener' {
        $path = Join-Path $temporary 'native-acl.lock'
        $sid = [Security.Principal.WindowsIdentity]::GetCurrent().User
        $acl = New-Object Security.AccessControl.FileSecurity
        $acl.SetAccessRuleProtection($true, $false)
        $acl.SetOwner($sid)
        $acl.AddAccessRule((New-Object Security.AccessControl.FileSystemAccessRule($sid, 'FullControl', 'Allow')))
        $handle = Open-CustodyLock $path $acl
        try {
            Assert-True ($handle.GetAccessControl().AreAccessRulesProtected)
            Assert-True ($handle.GetAccessControl().GetOwner([Security.Principal.SecurityIdentifier]).Value -eq $sid.Value)
            Reject { $other = [IO.File]::OpenRead($path); $other.Dispose() } 'being used by another process'
        } finally { $handle.Dispose() }
        $handle = Open-CustodyLock $path $acl
        $handle.Dispose()
    }
} finally { Remove-Item -LiteralPath $temporary -Recurse -Force }

Check 'native-step rejection retains prior ownership and records the unexpected state' {
    # Exercise the actual coordinator with independent fake observations. This
    # verifies ordering: an unexpected post-state must never become owned.
    $script:ObservationCount = 0
    $script:UnexpectedState = Get-ExpectedStepState 'enable-appverif'
    $script:UnexpectedState.Registry64.values += [ordered]@{name = 'Debugger'; kind = 'String'; data = 'unrelated.exe'}
    Set-Item Function:script:Get-ImageState {
        param([string]$Name)
        $script:ObservationCount++
        if ($script:ObservationCount -eq 1) { return (Empty-State) }
        return $script:UnexpectedState
    }
    Set-Item Function:script:Invoke-BoundedTool {
        param([string]$Path, [string[]]$Arguments)
        return [ordered]@{exit = 0; output = 'independent control'}
    }
    Set-Item Function:script:Write-Receipt { param([string]$Path, $Receipt) }
    $receipt = [ordered]@{image = [ordered]@{name = 'NevroTick-0123456789ab.exe'}; expected = (Empty-State); status = 'prepared'; steps = @()}
    Reject { Invoke-OwnedStep 'unused.json' $receipt 'enable-appverif' 'unused.exe' @('/verify', 'NevroTick-0123456789ab.exe') } 'not adopted for cleanup'
    Assert-StateMatch $receipt.expected (Empty-State)
    Assert-True ($receipt.steps.Count -eq 1 -and $receipt.status -eq 'observed-unadmitted-enable-appverif')
    Assert-True (@($receipt.steps[0].observed_state.Registry64.values | Where-Object {$_.name -eq 'Debugger'}).Count -eq 1)
}

Check 'only current-user preflight requires debugger configuration' {
    $script:ConfigRequirements = @()
    Set-Item Function:script:Get-AdmittedTools {
        param([bool]$RequireDebuggerConfiguration = $true)
        $script:ConfigRequirements += $RequireDebuggerConfiguration
        return @{}
    }
    Set-Item Function:script:Test-Elevated { return $false }
    $script:Action = 'Preflight'
    $preflight = Invoke-MemoryWorkflow
    Assert-True ($preflight.status -eq 'admitted')
    foreach ($operation in @('Enable', 'Disable')) {
        $script:Action = $operation
        Reject { Invoke-MemoryWorkflow } 'elevated configuration shell'
    }
    Assert-True (($script:ConfigRequirements -join ',') -eq 'True,False,False')
}

Write-Output "Passed $script:Passed helper controls. No IFEO mutations or diagnostic tool execution."
