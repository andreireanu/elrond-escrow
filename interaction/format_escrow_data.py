import sys
import os 
import json
import re

type = sys.argv[1]
address = sys.argv[2] 

if type == 'send':
    with open(os.getcwd() + '/elrond-escrow/interaction/getSendData.json', 'r') as f:
        data = json.load(f)
    f = open(os.getcwd() + '/elrond-escrow/interaction/SEND-' + address + '.txt', 'w+')
if type == 'receive':
    with open(os.getcwd() + '/elrond-escrow/interaction/getReceiveData.json', 'r') as f:
        data = json.load(f)
    f = open(os.getcwd() + '/elrond-escrow/interaction/RECEIVE-'  + address + '.txt', 'w+')

out = ''
for record in data:
    splt = re.sub(r'0000000(?!0).',',', record['hex']).split(',')
    stream = os.popen('erdpy wallet bech32 --encode ' + str(splt[0]))
    wallet = stream.read().rstrip() 
    token_to = bytearray.fromhex(splt[1]).decode()
    amount_to = int(splt[2], 16)
    token_from = bytearray.fromhex(splt[3]).decode()
    amount_from = int(splt[4], 16)
    f.write(out)
    f.write(wallet + ', ' + token_to + ', ' + str(amount_to) + ', ' + token_from + ', ' + str(amount_from) + '\n')

f.close()