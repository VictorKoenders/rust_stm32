set PATH=%PATH%;D:\Tools\OpenOCD\bin;D:\Tools\OpenOCD
start openocd -s D:\Tools\OpenOCD\share\openocd\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
start setup_itmdump.bat