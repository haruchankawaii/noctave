$ErrorActionPreference = 'Stop'
$projectDirectory = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $projectDirectory
try {
    cargo build --release --locked
    if ($LASTEXITCODE -ne 0) { throw 'Noctave release build failed.' }
    $packageDirectory = Join-Path $projectDirectory 'dist\noctave-beta-windows-x64'
    New-Item -ItemType Directory -Path $packageDirectory -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $projectDirectory 'target\release\noctave.exe') -Destination (Join-Path $packageDirectory 'noctave.exe')
    Copy-Item -LiteralPath (Join-Path $projectDirectory 'README.md') -Destination (Join-Path $packageDirectory 'README.md')
    Copy-Item -LiteralPath (Join-Path $projectDirectory 'LICENSE') -Destination (Join-Path $packageDirectory 'LICENSE')
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'portable-launch.cmd') -Destination (Join-Path $packageDirectory 'Start Noctave.cmd')
    Compress-Archive -LiteralPath $packageDirectory -DestinationPath (Join-Path $projectDirectory 'dist\noctave-beta-windows-x64.zip') -Force
    Write-Output "Portable app: $packageDirectory"
    Get-FileHash -LiteralPath (Join-Path $packageDirectory 'noctave.exe') -Algorithm SHA256
} finally { Pop-Location }
