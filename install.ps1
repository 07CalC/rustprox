$BinaryUrl = "https://github.com/07CalC/rustprox/releases/download/v1.0_win/rustprox.exe"
$InstallDir = "$env:USERPROFILE\AppData\Local\Programs\proxy-tool"
$BinaryPath = "$InstallDir\proxy-tool.exe"

if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Invoke-WebRequest -Uri $BinaryUrl -OutFile $BinaryPath
Write-Output "`n✅ Installed rustprox.exe to $InstallDir"

$env:Path += ";$InstallDir"
[System.Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::User)

Write-Output "➡️  Added '$InstallDir' to your PATH."
Write-Output "`nYou can now run the proxy tool directly from the terminal with 'rustprox'."
