@echo off
cd /d "%~dp0"
if not exist "target\release\noctave.exe" (
    cargo build --release --locked
    if errorlevel 1 (
        echo Build failed. Install Bun, the Rust toolchain and Windows C++ build tools, then try again.
        pause
        exit /b 1
    )
)
powershell.exe -NoProfile -Command "Start-Process -FilePath '.\target\release\noctave.exe' -WindowStyle Hidden"
