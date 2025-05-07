# mx-community-qdr-sc
A smart contract from MultiversX's community

### Virtual environment

Navigate to `py_scripts`, create a virtual environment and install the dependencies:

```
python3 -m venv ./.venv
source ./.venv/bin/activate
pip install -r ./requirements.txt --upgrade
```

### Web wallet

Create your web wallets to interract with your smart contract:

```
mxpy wallet new --format pem --outfile wallets/deployer.pem
```
