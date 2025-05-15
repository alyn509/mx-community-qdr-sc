import os
from time import sleep
import subprocess

# NOTE: we will make a 6 sec. break between every instruction so that we will not get in the situation of transactions overwriting each other


exec(open('interaction/deploy.py').read())
sleep(6)
exec(open('interaction/call.py').read(), { 'endpoint': "purchase", 'caller': "buyer1", 'transfer': 100 })
sleep(6)
exec(open('interaction/call.py').read(), { 'endpoint': "purchase", 'caller': "buyer1", 'transfer': 1000 })
sleep(6)
exec(open('interaction/call.py').read(), { 'endpoint': "purchase", 'caller': "buyer2", 'transfer': 100 })
sleep(6)
exec(open('interaction/call.py').read(), { 'endpoint': "purchase", 'caller': "buyer3", 'transfer': 100 })
sleep(6)
exec(open('interaction/call.py').read(), { 'endpoint': "purchase", 'caller': "buyer4", 'transfer': 200000 })
sleep(6)

exec(open('interaction/call.py').read(), { 'endpoint': "claimPromoRewards", 'caller': "buyer1" })
sleep(6)
exec(open('interaction/call.py').read(), { 'endpoint': "claimPromoRewards", 'caller': "buyer4" })
sleep(6)
