import sys
sys.path.append("..")

from py_scripts.scenario_helper import execute_with_args



execute_with_args("deploy")

execute_with_args("call", ["--endpoint", "purchase", "--caller", "buyer1", "--transfer-amount", "100"])
execute_with_args("call", ["--endpoint", "purchase", "--caller", "buyer1", "--transfer-amount", "1000"])
execute_with_args("call", ["--endpoint", "purchase", "--caller", "buyer2", "--transfer-amount", "100"])
execute_with_args("call", ["--endpoint", "purchase", "--caller", "buyer3", "--transfer-amount", "100"])
execute_with_args("call", ["--endpoint", "purchase", "--caller", "buyer4", "--transfer-amount", "200000"])

execute_with_args("call", ["--endpoint", "claimPromoRewards", "--caller", "buyer1"])
execute_with_args("call", ["--endpoint", "claimPromoRewards", "--caller", "buyer4"])
