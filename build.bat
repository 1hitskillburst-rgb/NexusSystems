@echo off
echo ============================================
echo   Building NEXUS / SYSTEMS Release Binary
echo ============================================
cargo build --release
if errorlevel 1 (
    echo [ERROR] Build failed!
    exit /b %errorlevel%
)
copy /y target\release\nexus_systems.exe NexusSystems.exe >nul
echo [SUCCESS] Updated NexusSystems.exe in root directory!
