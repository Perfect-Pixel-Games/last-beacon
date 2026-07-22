@echo off
setlocal

set "GAME_REPOSITORY_ROOT=%~dp0.."
set "GAME_PROJECT_PATH=%GAME_REPOSITORY_ROOT%\game\Cargo.toml"

if "%FOUNDATION_BSN_PROFILE_MS%"=="" set "FOUNDATION_BSN_PROFILE_MS=1"
if "%LAST_BEACON_WIDGET_PROFILE_MS%"=="" set "LAST_BEACON_WIDGET_PROFILE_MS=1"

echo Profiling Last Beacon scene transitions.
echo FOUNDATION_BSN_PROFILE_MS=%FOUNDATION_BSN_PROFILE_MS%
echo LAST_BEACON_WIDGET_PROFILE_MS=%LAST_BEACON_WIDGET_PROFILE_MS%
echo Bevy Chrome trace output will be written by the bevy/trace_chrome subscriber.
echo Pass runtime arguments after this script, for example: --scene last-beacon/main_menu

cargo run --manifest-path "%GAME_PROJECT_PATH%" --profile foundation-test --features "bevy/trace_chrome,bevy/debug" -- %*
exit /b %ERRORLEVEL%
