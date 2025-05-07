import sys

from argparse import ArgumentParser
from logging import config
from pathlib import Path
from typing import List

from py_scripts import config
from py_scripts.context import Context

def main(cli_args: List[str]):
    parser = ArgumentParser()
    parser.add_argument("--endpoint", required=True, choices=["purchase", "claimPromoRewards"]) 
    parser.add_argument("--caller", default=config.DEFAULT_OWNER, type=Path) 
    parser.add_argument("--transfer-amount", type=int, default=0)
    args = parser.parse_args(cli_args)

    context = Context()
    context.deploy(config.INIT_APY)
    context.call(args.caller, args.endpoint, args.transfer_amount)

if __name__ == "__main__":
    main(sys.argv[1:])
