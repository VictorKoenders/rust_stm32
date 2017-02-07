target remote :3333
load
continue
monitor tpiu config internal /itm.txt uart off 8000000
