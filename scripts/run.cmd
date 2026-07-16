@echo off
setlocal

call "%~dp0foundation-game.cmd" run %*
exit /b %ERRORLEVEL%
