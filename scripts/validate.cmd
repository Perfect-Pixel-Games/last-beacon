@echo off
setlocal

cargo fmt --manifest-path "%~dp0..\game\Cargo.toml" -- --check
if errorlevel 1 exit /b %ERRORLEVEL%

cargo clippy --manifest-path "%~dp0..\game\Cargo.toml" --all-targets --all-features -- -D warnings
if errorlevel 1 exit /b %ERRORLEVEL%

cargo test --manifest-path "%~dp0..\game\Cargo.toml" --all-features
if errorlevel 1 exit /b %ERRORLEVEL%

cargo build --manifest-path "%~dp0..\game\Cargo.toml" --all-features
if errorlevel 1 exit /b %ERRORLEVEL%

cargo doc --manifest-path "%~dp0..\game\Cargo.toml" --all-features --no-deps
exit /b %ERRORLEVEL%
