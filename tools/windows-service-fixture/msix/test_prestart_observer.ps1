# SPDX-License-Identifier: GPL-2.0-only
# Inert CHECK-1/2/26/27 controls for the S0.6 launch owner in an offline Windows Sandbox.
param(
    [switch]$ExpectChildActorMismatch,
    [ValidateNotNullOrEmpty()][string]$ExpectedStandardUserSid
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (![Environment]::Is64BitProcess -or $PSVersionTable.PSEdition -cne 'Desktop' -or
    $PSVersionTable.PSVersion -lt [version]'5.1') {
    throw 'These controls require 64-bit Windows PowerShell 5.1'
}

$fixtureSource = @'
using System;
using System.ComponentModel;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;

public static class S06ObserverFixture
{
    private static readonly string DirectoryPath = AppDomain.CurrentDomain.BaseDirectory;

    [DllImport("kernel32.dll", ExactSpelling = true)]
    private static extern IntPtr GetStdHandle(int standardHandle);

    [DllImport("kernel32.dll", ExactSpelling = true)]
    private static extern bool CloseHandle(IntPtr handle);

    private static void RecordPid(string name)
    {
        using (Process self = Process.GetCurrentProcess())
            File.WriteAllText(Path.Combine(DirectoryPath, name), self.Id.ToString(CultureInfo.InvariantCulture));
    }

    private static void Write(Stream stream, byte[] bytes)
    {
        stream.Write(bytes, 0, bytes.Length);
        stream.Flush();
    }

    private static int TrySelfSpawn()
    {
        ProcessStartInfo start = new ProcessStartInfo(Path.Combine(DirectoryPath, "nevrotick-prestart-observer.exe"));
        start.Arguments = "--fixture-descendant";
        start.UseShellExecute = false;
        start.CreateNoWindow = true;
        Process descendant;
        try
        {
            descendant = Process.Start(start);
        }
        catch (Win32Exception failure)
        {
            Write(Console.OpenStandardOutput(), BitConverter.GetBytes(failure.NativeErrorCode));
            return 40;
        }
        using (descendant)
        {
            if (!descendant.WaitForExit(5000))
            {
                descendant.Kill();
                return 86;
            }
            return descendant.ExitCode == 19 ? 41 : 87;
        }
    }

    public static int Main(string[] args)
    {
        if (args.Length == 1 && args[0] == "--fixture-descendant")
        {
            RecordPid("descendant.pid");
            return 19;
        }

        RecordPid("observer.pid");
        if (args.Length != 9 || args[1] != "--exact" || args[2] != "--ignored" ||
            args[3] != "--nocapture" || args[4] != "--test-threads=1" || args[5] != "--format" ||
            args[6] != "pretty" || args[7] != "--color" || args[8] != "never")
            return 90;

        byte index;
        switch (args[0])
        {
            case "scm_observation_tests::native_installed_roles_prestart_observation": index = 1; break;
            case "scm_access_probe::windows::native_installed_roles_prestart_access": index = 2; break;
            case "scm_access_probe::windows::native_standard_user_prestart_change_config_refused": index = 3; break;
            default: return 91;
        }

        switch (Path.GetFileName(DirectoryPath.TrimEnd(Path.DirectorySeparatorChar)))
        {
            case "self-spawn":
                return TrySelfSpawn();
            case "stdout-over-cap":
                Write(Console.OpenStandardOutput(), new byte[8193]);
                break;
            case "stderr-over-cap":
                Write(Console.OpenStandardError(), new byte[8193]);
                break;
            case "open-pipe-timeout":
                Write(Console.OpenStandardOutput(), new byte[] { 3, 0, 255 });
                Write(Console.OpenStandardError(), new byte[] { 254, 4 });
                File.WriteAllText(Path.Combine(DirectoryPath, "waiting"), "open");
                break;
            case "eof-before-exit":
                if (!CloseHandle(GetStdHandle(-11)) || !CloseHandle(GetStdHandle(-12)))
                    return 92;
                File.WriteAllText(Path.Combine(DirectoryPath, "waiting"), "closed");
                Thread.Sleep(7000);
                File.WriteAllText(Path.Combine(DirectoryPath, "observer.natural-exit"), "natural");
                return 93;
            default:
                byte inherited = (byte)(Environment.GetEnvironmentVariable("NEVROTICK_S06_OBSERVER_SENTINEL") == null ? 0 : 1);
                Write(Console.OpenStandardOutput(), new byte[] { 0, 255, 13, 10, 128, index, inherited });
                Write(Console.OpenStandardError(), new byte[] { 254, 0, 10, index });
                return 23;
        }

        Thread.Sleep(20000);
        return 93;
    }
}
'@

$imageName = 'nevrotick-prestart-observer.exe'
$fixtureRoot = Join-Path ([IO.Path]::GetTempPath()) ('NevroTick observer controls ' + [Guid]::NewGuid().ToString('N'))
$sourceImage = Join-Path $fixtureRoot $imageName
$selectors = @(
    'scm_observation_tests::native_installed_roles_prestart_observation'
    'scm_access_probe::windows::native_installed_roles_prestart_access'
    'scm_access_probe::windows::native_standard_user_prestart_change_config_refused'
)

function New-FixtureCopy {
    param([string]$Directory)

    $null = [IO.Directory]::CreateDirectory($Directory)
    $image = Join-Path $Directory $imageName
    [IO.File]::Copy($sourceImage, $image)
    return $image
}

function Get-OwnedLiveObservers {
    param([string]$Directory)

    $prefix = $Directory + [IO.Path]::DirectorySeparatorChar
    foreach ($process in [Diagnostics.Process]::GetProcessesByName('nevrotick-prestart-observer')) {
        $retained = $false
        try {
            if (!$process.HasExited -and $process.MainModule.FileName.StartsWith($prefix, [StringComparison]::OrdinalIgnoreCase)) {
                $null = $process.Handle
                $retained = $true
                $process
            }
        } catch {
            if (!$process.HasExited) { throw }
        } finally {
            if (!$retained) { $process.Dispose() }
        }
    }
}

function Assert-FixtureQuiescent {
    param([string]$Directory, [string]$Label)

    $live = @(Get-OwnedLiveObservers $Directory)
    if ($live.Count -gt 0) {
        foreach ($process in $live) { $process.Dispose() }
        throw "$Label left a test-owned image running before teardown"
    }
}

function Assert-Bytes {
    param([object]$Actual, [byte[]]$Expected, [string]$Label)

    if ($Actual -isnot [byte[]] -or $Actual.Length -ne $Expected.Length) {
        throw "$Label did not return the expected raw byte-array extent"
    }
    for ($index = 0; $index -lt $Expected.Length; $index++) {
        if ($Actual[$index] -ne $Expected[$index]) { throw "$Label differs at byte $index" }
    }
}

function Read-FixturePid {
    param([string]$Marker)

    $processId = 0
    if (![int]::TryParse([IO.File]::ReadAllText($Marker), [ref]$processId) -or $processId -le 0) {
        throw 'Invalid test-owned process marker'
    }
    return $processId
}

function Get-RefusalFact {
    param([Exception]$Refusal, [string]$Key)

    for ($cause = $Refusal; $null -ne $cause; $cause = $cause.InnerException) {
        if ($cause.Data.Contains($Key)) { return $cause.Data[$Key] }
    }
}

function Invoke-ObserverControl {
    param(
        [Parameter(Mandatory = $true)][object]$Actor,
        [string]$Label,
        [string]$ImagePath,
        [string]$Selector,
        [string]$StandardUserSid,
        [string]$ProvisioningUserSid,
        [string]$Digest = $imageDigest,
        [int]$ExitCode = 23,
        [byte[]]$Stdout = [byte[]]::new(0),
        [byte[]]$Stderr = [byte[]]::new(0),
        [string]$RefusalMessage,
        [string]$DisposedObjectName,
        [int]$TimeoutMilliseconds = 12000,
        [int]$ClockAgeMilliseconds = 0,
        [string]$WaitingState,
        [switch]$AlreadyExpired,
        [switch]$RefuseBeforeLaunch,
        [switch]$RequireNoNativeCreation,
        [switch]$RefuseBeforeEntry,
        [switch]$RequireSettledPipeReads,
        [switch]$RefuseDescendant,
        [switch]$DirectLauncher,
        [switch]$RestartAfterLaunch
    )

    $directory = Split-Path -LiteralPath $ImagePath
    $marker = Join-Path $directory 'observer.pid'
    $descendantMarker = Join-Path $directory 'descendant.pid'
    $records = [Collections.Generic.List[object]]::new()
    $refusal = $null
    $expectsRefusal = ![string]::IsNullOrEmpty($RefusalMessage) -or ![string]::IsNullOrEmpty($DisposedObjectName)
    [Diagnostics.Stopwatch]$originalClock = [Diagnostics.Stopwatch]::StartNew()
    [Diagnostics.Stopwatch]$clock = $originalClock
    if ($RestartAfterLaunch) { $clock = [Diagnostics.Stopwatch]::StartNew() }
    if ($ClockAgeMilliseconds -gt 0) { Start-Sleep -Milliseconds $ClockAgeMilliseconds }
    if ($AlreadyExpired -and $clock.ElapsedMilliseconds -lt $TimeoutMilliseconds) {
        throw "$Label clock is still live"
    }
    [hashtable]$subject = @{}
    if ($PSBoundParameters.ContainsKey('StandardUserSid')) {
        $subject.ExpectedStandardUserSid = $StandardUserSid
    }
    if ($PSBoundParameters.ContainsKey('ProvisioningUserSid')) {
        $subject.ExpectedProvisioningUserSid = $ProvisioningUserSid
    }
    try {
        & {
            if ($DirectLauncher -or $RestartAfterLaunch) {
                if ($subject.ContainsKey('ExpectedProvisioningUserSid')) {
                    $child = [NevroTick.PrestartObserverChild]::Start($ImagePath, $imageBytes, $Digest, $Selector,
                        $Actor, $clock, $TimeoutMilliseconds, $StandardUserSid, $ProvisioningUserSid)
                } else {
                    $child = [NevroTick.PrestartObserverChild]::Start($ImagePath, $imageBytes, $Digest, $Selector,
                        $Actor, $clock, $TimeoutMilliseconds, $StandardUserSid)
                }
                if ($RestartAfterLaunch) { $clock.Restart() }
                & $module {
                    param($Child)
                    Receive-OwnedPrestartChild $Child
                } $child
            } else {
                Invoke-PrestartObserver -Actor $Actor -ImagePath $ImagePath -ExpectedBytes $imageBytes -ExpectedSha256 $Digest `
                    -Selector $Selector -Clock $clock -TimeoutMilliseconds $TimeoutMilliseconds @subject
            }
        } | ForEach-Object { $records.Add($_) }
    } catch {
        $refusal = $_.Exception
        $cause = $refusal
        while ($null -ne $cause) {
            if (![string]::IsNullOrEmpty($DisposedObjectName)) {
                if ($cause -is [ObjectDisposedException] -and $cause.ObjectName -ceq $DisposedObjectName) { break }
            } elseif ($cause.Message -ceq $RefusalMessage) { break }
            $cause = $cause.InnerException
        }
        if (!$expectsRefusal -or $null -eq $cause) { throw }
    }

    if ($RequireSettledPipeReads) {
        $pending = @(Get-RefusalFact $refusal 'PrestartChildPendingReads')
        if ($pending.Count -ne 2) { throw "$Label did not retain both original pipe reads" }
        foreach ($read in $pending) {
            if ($read -isnot [Threading.Tasks.Task[int]] -or !$read.IsCompleted) {
                throw "$Label returned before an original pipe read settled"
            }
        }
    }
    # A successful exclusive reopen demonstrates release before fixture teardown.
    $reopened = [IO.File]::Open($ImagePath, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::None)
    $reopened.Dispose()
    Assert-FixtureQuiescent $directory $Label
    if ($expectsRefusal) {
        if ($null -eq $refusal -or $records.Count -ne 0) { throw "$Label published a child capture" }
        $refusedPid = Get-RefusalFact $refusal 'PrestartObserverProcessId'
        if ($RefuseBeforeLaunch -or $RequireNoNativeCreation) {
            if ($null -ne $refusedPid -or (Get-RefusalFact $refusal 'PrestartObserverProcessCreated') -eq $true -or
                [IO.File]::Exists($marker) -or [IO.File]::Exists($descendantMarker)) {
                throw "$Label launched an observer"
            }
            if ($RequireNoNativeCreation) {
                [object]$jobCreated = Get-RefusalFact $refusal 'PrestartObserverJobCreated'
                [object]$processCreated = Get-RefusalFact $refusal 'PrestartObserverProcessCreated'
                [object]$released = Get-RefusalFact $refusal 'PrestartObserverReleased'
                if ($jobCreated -isnot [bool] -or $jobCreated -or $processCreated -isnot [bool] -or $processCreated -or
                    $released -isnot [bool] -or !$released) {
                    throw "$Label did not confirm refusal before job and process creation and release of the image"
                }
            }
        } elseif ($RefuseBeforeEntry) {
            $created = Get-RefusalFact $refusal 'PrestartObserverProcessCreated'
            $retired = Get-RefusalFact $refusal 'PrestartObserverRetired'
            $released = Get-RefusalFact $refusal 'PrestartObserverReleased'
            if ($created -isnot [bool] -or !$created -or $refusedPid -isnot [int] -or $refusedPid -le 0 -or
                $retired -isnot [bool] -or !$retired -or $released -isnot [bool] -or !$released -or
                $null -ne (Get-RefusalFact $refusal 'PrestartObserverChild') -or
                [IO.File]::Exists($marker) -or [IO.File]::Exists($descendantMarker)) {
                throw "$Label did not retire and release the created observer before fixture entry"
            }
        } else {
            $released = Get-RefusalFact $refusal 'PrestartObserverReleased'
            if ($refusedPid -isnot [int] -or $refusedPid -ne (Read-FixturePid $marker) -or
                $released -isnot [bool] -or !$released -or
                ($RestartAfterLaunch -and (Get-RefusalFact $refusal 'PrestartObserverRetired') -ne $true)) {
                throw "$Label did not confirm retirement of the process that entered the fixture"
            }
            if ($null -ne (Get-RefusalFact $refusal 'PrestartObserverCleanupFailure') -or
                $null -ne (Get-RefusalFact $refusal 'PrestartObserverChild')) {
                throw "$Label retained uncertain retirement state"
            }
        }
        if (![string]::IsNullOrEmpty($WaitingState)) {
            if ($originalClock.ElapsedMilliseconds -lt $TimeoutMilliseconds -or
                ($RestartAfterLaunch -and $clock.ElapsedMilliseconds -ge $TimeoutMilliseconds) -or
                [IO.File]::ReadAllText((Join-Path $directory 'waiting')) -cne $WaitingState) {
                throw "$Label did not reach the required pipe state before its original cutoff"
            }
        }
        if ($ClockAgeMilliseconds -gt 0 -and [IO.File]::Exists((Join-Path $directory 'observer.natural-exit'))) {
            throw "$Label let the observer reach its natural exit after the original cutoff"
        }
    } else {
        if ($records.Count -ne 1) { throw "$Label did not return one completed observer" }
        $result = $records[0]
        if ($result.ProcessId -isnot [int] -or $result.ProcessId -ne (Read-FixturePid $marker) -or
            $result.ExitCode -isnot [int] -or $result.ExitCode -ne $ExitCode) {
            throw "$Label did not return the entered process ID and expected exit code"
        }
        if ($result.InputSha256Before -cne $imageDigest -or $result.InputSha256After -cne $imageDigest) {
            throw "$Label did not return the admitted observer digest at both bookends"
        }
        Assert-Bytes $result.Stdout $Stdout "$Label stdout"
        Assert-Bytes $result.Stderr $Stderr "$Label stderr"
        if ($RefuseDescendant -and [IO.File]::Exists($descendantMarker)) { throw 'The contained descendant entered its fixture' }
    }
    "s06-prestart-observer $Label passed"
}

$module = $null
$actor = $null
$firstFailure = $null
[int]$controls = 16
$sentinel = 'NEVROTICK_S06_OBSERVER_SENTINEL'
$priorSentinel = [Environment]::GetEnvironmentVariable($sentinel, 'Process')
try {
    $module = Import-Module (Join-Path $PSScriptRoot 'prestart_child.psm1') -Force -PassThru
    $null = [IO.Directory]::CreateDirectory($fixtureRoot)
    $sourcePath = Join-Path $fixtureRoot 'observer.cs'
    [IO.File]::WriteAllText($sourcePath, $fixtureSource)

    $compiler = Join-Path ([Runtime.InteropServices.RuntimeEnvironment]::GetRuntimeDirectory()) 'csc.exe'
    $compilerOutput = @(& $compiler /nologo /target:exe /platform:x64 "/out:$sourceImage" $sourcePath 2>&1)
    if ($LASTEXITCODE -ne 0 -or ![IO.File]::Exists($sourceImage)) {
        $detail = $compilerOutput -join ' '
        if ($detail.Length -gt 1024) { $detail = $detail.Substring(0, 1024) }
        throw "Fixture compilation failed ($LASTEXITCODE) $detail"
    }

    $imageBytes = (Get-Item -LiteralPath $sourceImage).Length
    $imageDigest = (Get-FileHash -LiteralPath $sourceImage -Algorithm SHA256).Hash.ToLowerInvariant()
    [Environment]::SetEnvironmentVariable($sentinel, 'inert-present-in-parent', 'Process')
    $actor = Open-PrestartObserverActor -Clock ([Diagnostics.Stopwatch]::StartNew()) -TimeoutMilliseconds 12000
    if ($ExpectChildActorMismatch) {
        # This invocation uses the directed comparison fault in the launcher source.
        $image = New-FixtureCopy (Join-Path $fixtureRoot 'child-actor-refused')
        Invoke-ObserverControl -Actor $actor -Label 'child-actor-refused' -ImagePath $image -Selector $selectors[0] `
            -RefusalMessage 'prestart-observer:child-actor' -RefuseBeforeEntry
        return
    }

    $image = New-FixtureCopy (Join-Path $fixtureRoot 'observation-raw')
    Invoke-ObserverControl -Actor $actor -Label 'observation-raw' -ImagePath $image -Selector $selectors[0] `
        -Stdout @(0, 255, 13, 10, 128, 1, 0) -Stderr @(254, 0, 10, 1)

    # Observe the current subject independently of the launcher's token code.
    [Security.Principal.WindowsIdentity]$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    try { [string]$currentSid = $identity.User.Value } finally { $identity.Dispose() }
    $wrongDigest = '0' * 64
    if ($imageDigest -ceq $wrongDigest) { $wrongDigest = '1' * 64 }
    $accessRefusals = @(
        @{ Label = 'access-missing-sid' }
        @{ Label = 'access-malformed-sid'; ProvisioningUserSid = 'invalid SID' }
        @{ Label = 'access-noncanonical-sid'; ProvisioningUserSid = $currentSid.ToLowerInvariant() }
        @{ Label = 'access-foreign-sid'; ProvisioningUserSid = 'S-1-0-0' }
        @{ Label = 'access-standard-slot-sid'; StandardUserSid = $currentSid }
    )
    foreach ($direct in @($false, $true)) {
        $prefix = ''
        if ($direct) { $prefix = 'direct-' }
        $label = $prefix + 'access-raw'
        $image = New-FixtureCopy (Join-Path $fixtureRoot $label)
        Invoke-ObserverControl -Actor $actor -Label $label -ImagePath $image -Selector $selectors[1] `
            -ProvisioningUserSid $currentSid -DirectLauncher:$direct `
            -Stdout @(0, 255, 13, 10, 128, 2, 0) -Stderr @(254, 0, 10, 2)
        $controls++
        foreach ($accessRefusal in $accessRefusals) {
            $control = $accessRefusal.Clone()
            $control.Label = $prefix + $control.Label
            $image = New-FixtureCopy (Join-Path $fixtureRoot $control.Label)
            Invoke-ObserverControl -Actor $actor -ImagePath $image -Selector $selectors[1] `
                -DirectLauncher:$direct -RefusalMessage 'prestart-observer:provisioning-subject-actor' `
                -RequireNoNativeCreation @control
            $controls++
        }
        $label = $prefix + 'access-wrong-digest'
        $image = New-FixtureCopy (Join-Path $fixtureRoot $label)
        Invoke-ObserverControl -Actor $actor -Label $label -ImagePath $image -Selector $selectors[1] `
            -DirectLauncher:$direct -Digest $wrongDigest -RefusalMessage 'prestart-observer:image-digest' `
            -RequireNoNativeCreation
        $controls++
    }

    [string]$standardSelector = $selectors[2]
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'standard-missing-sid')
    Invoke-ObserverControl -Actor $actor -Label 'standard-missing-sid' -ImagePath $image -Selector $standardSelector `
        -RefusalMessage 'prestart-observer:standard-user-subject-actor' -RequireNoNativeCreation
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'standard-foreign-sid')
    Invoke-ObserverControl -Actor $actor -Label 'standard-foreign-sid' -ImagePath $image -Selector $standardSelector `
        -StandardUserSid 'S-1-0-0' -RefusalMessage 'prestart-observer:standard-user-subject-actor' `
        -RequireNoNativeCreation

    $image = New-FixtureCopy (Join-Path $fixtureRoot 'standard-noncanonical-sid')
    Invoke-ObserverControl -Actor $actor -Label 'standard-noncanonical-sid' -ImagePath $image -Selector $standardSelector `
        -StandardUserSid $currentSid.ToLowerInvariant() -RefusalMessage 'prestart-observer:standard-user-subject-actor' `
        -RequireNoNativeCreation

    # Read the actual account's groups independently of the launcher using the SID column.
    [string]$whoami = Join-Path ([Environment]::SystemDirectory) 'whoami.exe'
    [string[]]$groupOutput = @(& $whoami /groups /fo csv /nh)
    if ($LASTEXITCODE -ne 0) { throw 'Could not observe the current actor groups' }
    [object[]]$groups = @($groupOutput | ConvertFrom-Csv -Header Name, Type, Sid, Attributes)
    if ($groups.Count -eq 0 -or @($groups | Where-Object { $_.Sid -cnotmatch '^S-1-[0-9]+(?:-[0-9]+)+$' }).Count -ne 0) {
        throw 'Invalid SID column in the current actor group observation'
    }
    [bool]$isAdministrator = @($groups | Where-Object { $_.Sid -ceq 'S-1-5-32-544' }).Count -ne 0
    if ($isAdministrator) {
        $image = New-FixtureCopy (Join-Path $fixtureRoot 'standard-admin-same-sid')
        Invoke-ObserverControl -Actor $actor -Label 'standard-admin-same-sid' -ImagePath $image -Selector $standardSelector `
            -StandardUserSid $currentSid -RefusalMessage 'prestart-observer:standard-user-token-profile' `
            -RequireNoNativeCreation
        $controls++
    } else {
        's06-prestart-observer administrator profile unavailable'
    }
    if ($PSBoundParameters.ContainsKey('ExpectedStandardUserSid')) {
        if ($isAdministrator -or $ExpectedStandardUserSid -cne $currentSid) {
            throw 'The independently admitted standard-user SID does not identify this ordinary actor'
        }
        $image = New-FixtureCopy (Join-Path $fixtureRoot 'standard-raw')
        Invoke-ObserverControl -Actor $actor -Label 'standard-raw' -ImagePath $image -Selector $standardSelector `
            -StandardUserSid $ExpectedStandardUserSid -Stdout @(0, 255, 13, 10, 128, 3, 0) -Stderr @(254, 0, 10, 3)
        $controls++
    } else {
        's06-prestart-observer independently admitted standard-user SID unavailable'
    }

    $selector = $selectors[0]
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'wrong-digest')
    Invoke-ObserverControl -Actor $actor -Label 'wrong-digest' -ImagePath $image -Selector $selector -Digest $wrongDigest `
        -RefusalMessage 'prestart-observer:image-digest' -RefuseBeforeLaunch
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'standard-wrong-digest')
    Invoke-ObserverControl -Actor $actor -Label 'standard-wrong-digest' -ImagePath $image -Selector $standardSelector `
        -Digest $wrongDigest -RefusalMessage 'prestart-observer:image-digest' -RequireNoNativeCreation
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'unknown-selector')
    Invoke-ObserverControl -Actor $actor -Label 'unknown-selector' -ImagePath $image -Selector '--fixture-descendant' `
        -RefusalMessage 'prestart-observer:selector' -RefuseBeforeLaunch
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'expired-before-launch')
    Invoke-ObserverControl -Actor $actor -Label 'expired-before-launch' -ImagePath $image -Selector $selector `
        -RefusalMessage 'prestart-observer:launch-deadline' -TimeoutMilliseconds 1 -ClockAgeMilliseconds 40 `
        -AlreadyExpired -RefuseBeforeLaunch

    foreach ($stream in @('stdout', 'stderr')) {
        $label = "$stream-over-cap"
        $image = New-FixtureCopy (Join-Path $fixtureRoot $label)
        Invoke-ObserverControl -Actor $actor -Label $label -ImagePath $image -Selector $selector `
            -RefusalMessage "prestart-child:$stream-limit"
    }

    $image = New-FixtureCopy (Join-Path $fixtureRoot 'open-pipe-timeout')
    Invoke-ObserverControl -Actor $actor -Label 'open-pipe-timeout' -ImagePath $image -Selector $selector `
        -RefusalMessage 'prestart-child:collection-deadline' -TimeoutMilliseconds 5000 -WaitingState 'open' `
        -RequireSettledPipeReads
    $image = New-FixtureCopy (Join-Path $fixtureRoot 'eof-before-exit')
    # The child exits naturally after seven seconds. Starting a fresh clock at
    # launch would accept that exit, while the original clock has four seconds left.
    Invoke-ObserverControl -Actor $actor -Label 'eof-before-exit' -ImagePath $image -Selector $selector `
        -RefusalMessage 'prestart-child:collection-deadline' -TimeoutMilliseconds 11000 -ClockAgeMilliseconds 7000 `
        -WaitingState 'closed'
    $image = New-FixtureCopy (Join-Path (Join-Path $fixtureRoot 'restarted') 'eof-before-exit')
    # Restart the same clock after native launch. Only the original cutoff
    # can expire before the entered observer reaches its natural exit.
    Invoke-ObserverControl -Actor $actor -Label 'restarted-eof-before-exit' -ImagePath $image -Selector $selector `
        -RefusalMessage 'prestart-child:collection-deadline' -TimeoutMilliseconds 11000 -ClockAgeMilliseconds 7000 `
        -WaitingState 'closed' -RestartAfterLaunch

    $directDirectory = Join-Path (Join-Path $fixtureRoot 'direct') 'self-spawn'
    $directImage = New-FixtureCopy $directDirectory
    $start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $directImage
    $start.Arguments = $selector + ' --exact --ignored --nocapture --test-threads=1 --format pretty --color never'
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $direct = [Diagnostics.Process]::Start($start)
    try {
        if (!$direct.WaitForExit(8000) -or $direct.ExitCode -ne 41 -or
            $direct.Id -ne (Read-FixturePid (Join-Path $directDirectory 'observer.pid'))) {
            throw 'Direct self-spawn failed to run the same inert image as a child'
        }
        $null = Read-FixturePid (Join-Path $directDirectory 'descendant.pid')
    } finally {
        $direct.Dispose()
    }
    Assert-FixtureQuiescent $directDirectory 'direct-self-spawn'
    's06-prestart-observer direct-self-spawn passed'

    $containedDirectory = Join-Path $fixtureRoot 'self-spawn'
    $image = New-FixtureCopy $containedDirectory
    # Expect the Win32 active-process-limit denial ERROR_NOT_ENOUGH_QUOTA (1816).
    Invoke-ObserverControl -Actor $actor -Label 'contained-self-spawn' -ImagePath $image -Selector $selector `
        -ExitCode 40 -Stdout @(24, 7, 0, 0) -RefuseDescendant

    $image = New-FixtureCopy (Join-Path $fixtureRoot 'actor-disposed')
    $disposedActor = $actor
    $actor = $null
    $disposedActor.Dispose()
    Invoke-ObserverControl -Actor $disposedActor -Label 'actor-disposed' -ImagePath $image -Selector $selector `
        -DisposedObjectName 'PrestartActor' -RefuseBeforeLaunch
} catch {
    $firstFailure = $_.Exception
    throw
} finally {
    try {
        # A failed test remains a failure even if emergency teardown succeeds.
        foreach ($process in @(Get-OwnedLiveObservers $fixtureRoot)) {
            try {
                if (!$process.HasExited) { $process.Kill() }
                if (!$process.WaitForExit(5000)) { throw 'A test-owned image survived emergency teardown' }
            } finally {
                $process.Dispose()
            }
        }
        if ([IO.Directory]::Exists($fixtureRoot)) { Remove-Item -LiteralPath $fixtureRoot -Recurse -Force }
    } catch {
        if ($null -ne $firstFailure) {
            throw [AggregateException]::new('Observer controls and their teardown failed', [Exception[]]@($firstFailure, $_.Exception))
        }
        throw
    } finally {
        try {
            if ($null -ne $actor) { $actor.Dispose() }
        } finally {
            [Environment]::SetEnvironmentVariable($sentinel, $priorSentinel, 'Process')
            if ($null -ne $module) { Remove-Module $module }
        }
    }
}

"s06-prestart-observer controls=$controls"
