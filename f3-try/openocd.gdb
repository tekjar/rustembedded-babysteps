target remote :3333
load
monitor tpiu config internal itm.txt uart off 8000000
monitor itm port 0 on
break main
continue