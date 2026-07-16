@echo off
setlocal

call "%~dp0foundation-game.cmd" build %*
exit /b %ERRORLEVEL%
