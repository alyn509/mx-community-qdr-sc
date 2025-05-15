# mx-community-qdr-sc
A smart contract from MultiversX's community


### Smart Contract build

If not having installed `sc-meta` one can install it by

```
cargo install multiversx-sc-meta --locked
```

afterwards build the smart contract by calling from the project root

```
sc-meta all build
```

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
mxpy wallet new --format pem --outfile wallets/qdr.pem
mxpy wallet new --format pem --outfile wallets/ma.pem
mxpy wallet new --format pem --outfile wallets/tt.pem
mxpy wallet new --format pem --outfile wallets/mb.pem
mxpy wallet new --format pem --outfile wallets/promo.pem
mxpy wallet new --format pem --outfile wallets/buyer1.pem
mxpy wallet new --format pem --outfile wallets/buyer2.pem
mxpy wallet new --format pem --outfile wallets/buyer3.pem
mxpy wallet new --format pem --outfile wallets/buyer4.pem
```

### Making transactions

You can make simple transactions by simply calling the corresponding  python files.

for deploy:

```
python3 interaction/deploy.py 
```

for upgrade:

```
python3 interaction/upgrade.py 

```
for calling endpoints:

```
python3 interaction/call.py --endpoint <ENDPOINT> --caller <CALLER> --transfer-amount <OPTIONAL_TRANSFER_AMOUNT> 
```

for example: if `buyer1` wants to make a purchase we will call

```
python3 interaction/call.py --endpoint purchase --caller buyer1 --transfer-amount 100

```

All the endpoints and wallets are configured in `call.py`. If you want to add extra elements just change the file and you can simply call them just the same.

Alternativelly, you can make yourself scenarios with multiple transactions and just call all of them at the same time. Check the example scenario.

### Testing without the blockchain

Sometimes when you just want to test locally a simple functionality of your smart contract you can just do it in a blackbox test (Check `tests/blackbox_test.rs`), to reduce the time invested in deploying the smart contract, adding funds to wallets etc.

Just make sure you have sc-meta installed and the smart contract build and you can simply test a scenario by clicking the build button from under the `#[test]` annotation. You can also set breakpoints in the smart contract and debug step by step if you need a more precise approach.
