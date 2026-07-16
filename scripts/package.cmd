@echo off
setlocal

call "%~dp0foundation-game.cmd" package %*
exit /b %ERRORLEVEL%
