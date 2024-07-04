@echo off
setlocal

set APP_NAME=danalyze

echo Cleaning up old build artifacts
cargo clean
mkdir dist
rm -rf dist

echo Building for Windows
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Windows build failed!
    exit /b %ERRORLEVEL%
)

echo Packaging Windows binary
cd target\release
if exist %APP_NAME%.exe (
    if exist %APP_NAME%-windows.zip del %APP_NAME%-windows.zip
    powershell -Command "Compress-Archive -Path %APP_NAME%.exe -DestinationPath %APP_NAME%-windows.zip"
    move %APP_NAME%-windows.zip ..\..\dist
) else (
    echo Windows binary not found!
    exit /b 1
)
cd ..\..

echo Build and packaging complete.
exit /b 0
