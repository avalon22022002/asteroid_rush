# Finds your WSL distro's audio server, for docker-compose.yaml
# - WSLg's audio socket lives inside one WSL distro, name varies per user
# - Writes its full path to .env as WSLG_PULSE_SOURCE
# - Runs automatically before the container starts (devcontainer.json)
# - Windows-only: bails out immediately elsewhere (Mac/Linux/pwsh), so
#   docker-compose.yaml's own placeholder default takes over instead

if (-not $IsWindows -and $PSVersionTable.PSVersion.Major -ge 6) {
    return
}
if ((Get-Command wsl.exe -ErrorAction SilentlyContinue) -eq $null) {
    return
}

$envFile = Join-Path $PSScriptRoot ".env"

$distros = @()
try {
    $distros = (wsl.exe -l -q 2>$null) |
        ForEach-Object { ($_ -replace "`0", "").Trim() } |
        Where-Object { $_ -ne "" -and $_ -notlike "docker-desktop*" }
} catch {
    $distros = @()
}

$found = $null
foreach ($distro in $distros) {
    wsl.exe -d $distro -- test -S /mnt/wslg/PulseServer 2>$null
    if ($LASTEXITCODE -eq 0) {
        $found = $distro
        break
    }
}

if ($found) {
    "WSLG_PULSE_SOURCE=\\wsl.localhost\$found\mnt\wslg\PulseServer" |
        Set-Content -Path $envFile -Encoding ascii -NoNewline
}
