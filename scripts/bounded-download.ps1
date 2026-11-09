#requires -Version 5.1
<#
One bounded HTTPS acquisition for the existing setup scripts. No installation,
elevation, trust, security-policy or environment mutation occurs in this helper.
Callers own the fixed URL, byte ceiling and optional expected SHA256. Redirects
are refused rather than allowing an unreviewed target to expand that authority.
The selected stream must support synchronous read timeouts. Elapsed checks reject
late completion; a native read timeout is not a measured hard total wall limit.
#>

function Invoke-BoundedHttpsDownload {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)][string]$Uri,
        [Parameter(Mandatory = $true)][ValidateRange(1, 4294967296)][long]$MaximumBytes,
        [string]$Destination,
        [ValidatePattern('^[a-fA-F0-9]{64}$')][string]$ExpectedSha256,
        [ValidateRange(1, 300)][int]$TimeoutSeconds = 120
    )
    if ($Uri.Length -gt 8192 -or $Uri -match '[\x00-\x20\x7f-\uffff\\]') {
        throw 'Download URL has an unadmitted representation.'
    }
    $parsed = [Uri]$Uri
    if (-not $parsed.IsAbsoluteUri -or $parsed.Scheme -cne 'https' -or
        $parsed.UserInfo -or $Uri -match '^https://[^/?#]*@' -or
        $parsed.Fragment -or $parsed.Port -ne 443) {
        throw 'Download requires HTTPS without credentials, fragment or alternate port.'
    }
    if (-not $Destination -and $MaximumBytes -gt 1048576) {
        throw 'In-memory downloads are limited to small metadata; stream artifacts to a file.'
    }
    if ($Destination -and (Test-Path -LiteralPath $Destination)) {
        $existing = Get-Item -LiteralPath $Destination -Force
        if ($existing.PSIsContainer -or ($existing.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            throw 'Download destination must be a regular file or absent.'
        }
    }
    Add-Type -AssemblyName System.Net.Http
    $handler = New-Object System.Net.Http.HttpClientHandler
    $handler.AllowAutoRedirect = $false
    $handler.AutomaticDecompression = [Net.DecompressionMethods]::None
    $protocols = [Security.Authentication.SslProtocols]::Tls12
    if ([Enum]::IsDefined([Security.Authentication.SslProtocols], 'Tls13')) {
        $protocols = $protocols -bor [Enum]::Parse([Security.Authentication.SslProtocols], 'Tls13')
    }
    $handler.SslProtocols = $protocols
    $client = New-Object System.Net.Http.HttpClient($handler)
    $client.Timeout = [TimeSpan]::FromSeconds($TimeoutSeconds)
    $cancellation = New-Object Threading.CancellationTokenSource
    $cancellation.CancelAfter($TimeoutSeconds * 1000)
    $elapsed = [Diagnostics.Stopwatch]::StartNew()
    $response = $null
    $inputStream = $null
    $outputStream = $null
    $temporary = $null
    $hasher = [Security.Cryptography.SHA256]::Create()
    try {
        $task = $client.GetAsync($parsed, [Net.Http.HttpCompletionOption]::ResponseHeadersRead, $cancellation.Token)
        $response = $task.GetAwaiter().GetResult()
        if ([int]$response.StatusCode -ne 200) {
            throw 'Download returned a non-success status or unadmitted redirect.'
        }
        $declaredLength = $response.Content.Headers.ContentLength
        if ($null -ne $declaredLength -and $declaredLength -gt $MaximumBytes) {
            throw 'Download exceeds its admitted byte ceiling.'
        }
        if (@($response.Content.Headers.ContentEncoding).Count -ne 0) {
            throw 'Encoded download bodies are not admitted.'
        }
        $inputStream = $response.Content.ReadAsStreamAsync().GetAwaiter().GetResult()
        # Framework's inherited Stream.ReadAsync may check cancellation only
        # before beginning a read. Require an actual synchronous socket timeout
        # and reject late completion separately; caller timeout alone does not
        # establish termination of pending native I/O.
        if (-not $inputStream.CanTimeout) {
            throw 'The response stream cannot enforce the required bounded read timeout.'
        }
        if ($Destination) {
            $fullDestination = [IO.Path]::GetFullPath($Destination)
            $temporary = $fullDestination + '.nevrotick-' + [Guid]::NewGuid().ToString('N') + '.tmp'
            $outputStream = [IO.File]::Open($temporary, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
        }
        else { $outputStream = New-Object IO.MemoryStream }
        $buffer = New-Object byte[] 65536
        [long]$total = 0
        while ($true) {
            $cancellation.Token.ThrowIfCancellationRequested()
            $remaining = [Math]::Ceiling(($TimeoutSeconds * 1000) - $elapsed.Elapsed.TotalMilliseconds)
            if ($remaining -le 0) { throw 'Download exceeded its elapsed-time limit.' }
            $inputStream.ReadTimeout = [int]$remaining
            $capacity = [int][Math]::Min(65536, $MaximumBytes - $total + 1)
            $count = $inputStream.Read($buffer, 0, $capacity)
            $cancellation.Token.ThrowIfCancellationRequested()
            if ($elapsed.Elapsed.TotalMilliseconds -ge ($TimeoutSeconds * 1000)) {
                throw 'Download exceeded its elapsed-time limit.'
            }
            if ($count -gt $MaximumBytes - $total) {
                throw 'Download exceeds its admitted byte ceiling.'
            }
            if ($count -eq 0) { break }
            $outputStream.Write($buffer, 0, $count)
            $null = $hasher.TransformBlock($buffer, 0, $count, $buffer, 0)
            $total += $count
        }
        if ($null -ne $declaredLength -and $total -ne $declaredLength) {
            throw 'Download body does not match its declared length.'
        }
        $null = $hasher.TransformFinalBlock((New-Object byte[] 0), 0, 0)
        $actual = ([BitConverter]::ToString($hasher.Hash)).Replace('-', '').ToLowerInvariant()
        if ($ExpectedSha256 -and $actual -cne $ExpectedSha256.ToLowerInvariant()) {
            throw 'Download SHA256 mismatch; prior destination preserved.'
        }
        if ($Destination) {
            $outputStream.Flush($true)
            $outputStream.Dispose()
            $outputStream = $null
            if (Test-Path -LiteralPath $fullDestination) {
                $existing = Get-Item -LiteralPath $fullDestination -Force
                if ($existing.PSIsContainer -or ($existing.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
                    throw 'Download destination changed to an unadmitted object.'
                }
                [IO.File]::Replace($temporary, $fullDestination, $null)
            }
            else { [IO.File]::Move($temporary, $fullDestination) }
        }
        else { return ,($outputStream.ToArray()) }
    }
    finally {
        if ($null -ne $inputStream) { $inputStream.Dispose() }
        if ($null -ne $outputStream) { $outputStream.Dispose() }
        if ($null -ne $response) { $response.Dispose() }
        $hasher.Dispose()
        $cancellation.Dispose()
        $client.Dispose()
        $handler.Dispose()
        if ($temporary -and [IO.File]::Exists($temporary)) { [IO.File]::Delete($temporary) }
    }
}
