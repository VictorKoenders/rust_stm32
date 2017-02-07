set PATH=%PATH%;D:\Tools\OpenOCD-0.9.0-Win32\bin;D:\Tools\OpenOCD-0.9.0-Win32
start openocd -s D:\Tools\OpenOCD-0.9.0-Win32\share\openocd\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
start setup_itmdump.bat