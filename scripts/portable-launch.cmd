@echo off
cd /d "%~dp0"
powershell.exe -NoProfile -Command "Start-Process -FilePath '.\noctave.exe' -WindowStyle Hidden"
