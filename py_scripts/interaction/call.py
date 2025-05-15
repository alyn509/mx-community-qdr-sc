import sys
sys.path.append("..")


from argparse import ArgumentParser
from logging import config
from pathlib import Path
from typing import List

from multiversx_sdk import Account

from py_scripts import config
from py_scripts.context import Context

def main(cli_args: List[str]):
    context = Context()

    parser = ArgumentParser()
    parser.add_argument("--endpoint", required=True, choices=["purchase", "claimPromoRewards"]) 
    parser.add_argument("--caller", default="owner", type=str) 
    parser.add_argument("--transfer-amount", type=int, default=0)
    args = parser.parse_args(cli_args)

    match args.caller:
        case "owner":
            caller = context.deployer_account
        case "qdr_mag":
            caller = context.qdr_mag_account
        case "ma": 
            caller = context.ma_account
        case "tt":
            caller = context.tt_account
        case "mb":
            caller = context.mb_account
        case "buyer1":
            caller = context.buyer1
        case "buyer2":
            caller = context.buyer2
        case "buyer3":
            caller = context.buyer3
        case "buyer4":
            caller = context.buyer4
        case _:
            print("invalid account")
            return
        
    context.call(caller, args.endpoint, args.transfer_amount)
    

if __name__ == "__main__":
    main(sys.argv[1:])

