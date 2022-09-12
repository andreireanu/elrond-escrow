# Return hex formated value to be sent to address

import sys

hex_value = hex(int(sys.argv[1]))
if len(hex_value) % 2 == 1 :
    print( ("0" + hex_value)[2:]) 
else:
    print( hex_value[2:] )