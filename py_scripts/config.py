from pathlib import Path

HOME = Path().home()
PY_SCRIPTS_WORKSPACE = Path(__file__).parent
DEFAULT_WORKSPACE = PY_SCRIPTS_WORKSPACE.parent


DEFAULT_PROXY = "https://devnet-gateway.multiversx.com"    
DEFAULT_API = "https://devnet-api.multiversx.com"                              # API to be used for ALL operations=

# Operation wallets
DEFAULT_OWNER = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "deployer.pem"         # owner address
QDR_MAG = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "qdr.pem" 
MA = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "ma.pem" 
TT = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "tt.pem" 
MB = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "mb.pem" 
PROMO = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "promo.pem" 

BUYER1 = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "buyer1.pem" 
BUYER2 = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "buyer2.pem" 
BUYER3 = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "buyer3.pem" 
BUYER4 = PY_SCRIPTS_WORKSPACE.absolute() / "wallets" / "buyer4.pem" 


# QDR_MAG_ADDRESS = "erd1ffexx7wqz9d906nveynep4l43ufervfpdn4x2v90r70hj34mk7gsqz9jdj"  
# MA_ADDRESS = "erd1e98jh25urgd46x27q50praswevq4nu042khm3q6wu73gdzqaj0fq89vxq4"  
# TT_ADDRESS = "erd1sqrlhgchys2ca2u2duteteafrgx44m427rx0aer4kvcxmdwhpy4qe7cn79"
# MB_ADDRESS = "erd19m8x8f3lay4ma4ysnftes07q3flve89ufck0n03dy8tprutdzy2sq2uy8p"
# PROMO_ADDRESS = "erd1tzjhg5lq4t3m3d5qyhsz5a3nvf9hpcfp7za892n5trv9m0z9cv0q6ym3m6"

QDR_WASM_PATH = DEFAULT_WORKSPACE.absolute() / "output" / "qdr-contract.wasm"
QDR_ABI = DEFAULT_WORKSPACE.absolute() / "output" / "qdr-contract.abi.json"

rounds_per_epoch = "50"

GAS_DEPLOY = 60000000
GAS_CALL = 15000000

INIT_APY = 1200
