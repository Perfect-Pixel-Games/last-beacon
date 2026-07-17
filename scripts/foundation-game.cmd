@echo off
setlocal EnableDelayedExpansion

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

set "FOUNDATION_ARGUMENTS="
set "FOUNDATION_RUNTIME_ARGUMENTS="
set "FOUNDATION_FOUND_RUNTIME_DELIMITER="

:collect_arguments
if "%~1"=="" goto run_foundation_command
if "%~1"=="--" (
    set "FOUNDATION_FOUND_RUNTIME_DELIMITER=1"
    set "FOUNDATION_RUNTIME_ARGUMENTS=!FOUNDATION_RUNTIME_ARGUMENTS! --"
) else if defined FOUNDATION_FOUND_RUNTIME_DELIMITER (
    set FOUNDATION_RUNTIME_ARGUMENTS=!FOUNDATION_RUNTIME_ARGUMENTS! "%~1"
) else (
    set FOUNDATION_ARGUMENTS=!FOUNDATION_ARGUMENTS! "%~1"
)
shift /1
goto collect_arguments

:run_foundation_command
call "%ENGINE_PATH%\scripts\foundation-build.cmd" !FOUNDATION_ARGUMENTS! --project "%GAME_PROJECT_PATH%" !FOUNDATION_RUNTIME_ARGUMENTS!
exit /b %ERRORLEVEL%
