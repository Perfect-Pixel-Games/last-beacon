@echo off
setlocal

set "GAME_REPOSITORY_ROOT=%~dp0.."
set "GAME_PROJECT_PATH=%GAME_REPOSITORY_ROOT%\game"

if not "%FOUNDATION_ENGINE_PATH%"=="" (
    set "ENGINE_PATH=%FOUNDATION_ENGINE_PATH%"
) else (
    set "ENGINE_PATH=%GAME_REPOSITORY_ROOT%\engine"
)

if not exist "%ENGINE_PATH%\scripts\foundation-build.cmd" (
    echo Foundation engine was not found at "%ENGINE_PATH%".
    echo Set FOUNDATION_ENGINE_PATH or initialize the engine submodule with:
    echo git submodule update --init --recursive
    exit /b 1
)

call "%ENGINE_PATH%\scripts\foundation-build.cmd" %* --project "%GAME_PROJECT_PATH%"
exit /b %ERRORLEVEL%
