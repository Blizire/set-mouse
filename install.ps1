#Requires -RunAsAdministrator

$ErrorActionPreference = 'Stop'

$installDir = Join-Path $env:ProgramFiles 'set-mouse'
$exeName = 'set-mouse.exe'

Write-Host "Building release binary..."
cargo build --release --manifest-path (Join-Path $PSScriptRoot 'Cargo.toml')
if ($LASTEXITCODE -ne 0) {
    throw "cargo build failed"
}

$sourceExe = Join-Path $PSScriptRoot "target\release\$exeName"
if (-not (Test-Path $sourceExe)) {
    throw "Built executable not found at $sourceExe"
}

if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

Copy-Item $sourceExe (Join-Path $installDir $exeName) -Force

$machinePath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
$pathEntries = $machinePath -split ';'
if ($pathEntries -notcontains $installDir) {
    $separator = if ($machinePath.EndsWith(';')) { '' } else { ';' }
    [Environment]::SetEnvironmentVariable('Path', "$machinePath$separator$installDir", 'Machine')
    Write-Host "Added $installDir to the system PATH."
} else {
    Write-Host "$installDir is already on the system PATH."
}

Write-Host "set-mouse installed to $installDir"
Write-Host "Open a new terminal window to use the 'set-mouse' command."
