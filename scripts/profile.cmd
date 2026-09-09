@echo off
setlocal

call "%~dp0foundation-game.cmd" tools run tracy
if errorlevel 1 exit /b %ERRORLEVEL%

call "%~dp0foundation-game.cmd" run --features profiling %*
exit /b %ERRORLEVEL%
