@echo off
set PATH=%PATH%;C:\Program Files (x86)\GNU Tools ARM Embedded\5.4 2016q3\bin
set CARGO_FEATURE_INTERRUPTS=1
set HOME=%USERPROFILE%
if "%1" == "release" (
    @REM echo xargo build --target thumbv7em-none-eabihf --release
    xargo build --target thumbv7em-none-eabihf --release
    echo %errorlevel%
) else (
    @REM echo xargo build --target thumbv7em-none-eabihf
    xargo build --target thumbv7em-none-eabihf
)
if "%errorlevel%" == "9009" ( GOTO end )
if "%errorlevel%" == "101" ( GOTO end )
echo %errorlevel%
if "%1" == "release" (
    @REM echo arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/release/rust_stm32
    arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/release/rust_stm32
) else (
    @REM echo arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/rust_stm32
    arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/rust_stm32
)
:end