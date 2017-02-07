target remote :3333
load
break main
continue
monitor tpiu config internal /itm.txt uart off 8000000
