import config

from multiversx_sdk import (Account, Address,
                             ProxyNetworkProvider,
                             TransactionsFactoryConfig,
                             SmartContractTransactionsFactory, find_events_by_identifier,
                             )
from multiversx_sdk.abi import Abi

class Context:
    def __init__(self):
        self.deployer_account = Account.new_from_pem(config.DEFAULT_OWNER)
        self.qdr_mag_account = Account.new_from_pem(config.QDR_MAG)
        self.ma_account = Account.new_from_pem(config.MA)
        self.tt_account = Account.new_from_pem(config.TT)
        self.mb_account = Account.new_from_pem(config.MB)
        self.promo_account = Account.new_from_pem(config.PROMO)

        self.proxy = ProxyNetworkProvider(config.DEFAULT_PROXY)
        self.factoryConfig = TransactionsFactoryConfig(self.proxy.get_network_config().chain_id)
        self.abiObj = Abi.load(config.QDR_ABI)
        self.factorySC = SmartContractTransactionsFactory(self.factoryConfig, self.abiObj)

    def deploy(self, init_apy: int):
        self.deployer_account.nonce = self.proxy.get_account(self.deployer_account.address).nonce
        transaction = self.factorySC.create_transaction_for_deploy(self.deployer_account.address,
                                                                    config.QDR_WASM_PATH, 
                                                                    config.GAS_DEPLOY,
                                                                    [init_apy],
                                                                    0,
                                                                    True,
                                                                    True,
                                                                    False,
                                                                    True)
        transaction.nonce = self.deployer_account.nonce
        transaction.signature = self.deployer_account.sign_transaction(transaction)
        hash = self.proxy.send_transaction(transaction).hex()
        self.sc_address = find_events_by_identifier(hash, "SCDeploy")[0].address.to_bech32

        with open("sc_address.txt", "W") as f:
            f.write(self.sc_address)

        print(f'Deploy successful. tx_hash: {hash} sc_address: {self.sc_address}')
        
        self.address_setup()
    

    def deploy_or_set_sc_address(self, init_apy: int):
        with open("demofile.txt") as f:
            self.sc_address = f.read()
        if not self.sc_address:
            self.deploy(init_apy)

    def address_setup(self):
        if not hasattr(self, 'sc_address'):
            print(f'setup failed: No SC deployed yet')
            return
        self.set_address("setQdrMagAddress", self.qdr_mag_account.address)
        self.set_address("setMaAddress", self.ma_account.address)
        self.set_address("setTtAddress", self.tt_account.address)
        self.set_address("setMbAddress", self.mb_account.address)
        self.set_address("setPromoAddress", self.promo_account.address)


    def call(self, caller: Account, endpoint: str, transfer_amount: int):
        if not hasattr(self, 'sc_address'):
            print(f'{endpoint} call failed: No SC deployed yet')
            return
        
        caller.nonce = self.proxy.get_account(caller.address).nonce
        transaction = self.factorySC.create_transaction_for_execute(caller.address,
                                                                self.sc_address, 
                                                                endpoint,
                                                                config.GAS_DEPLOY,
                                                                [],
                                                                transfer_amount)
        transaction.nonce = caller.nonce
        transaction.signature = caller.sign_transaction(transaction)
        hash = self.proxy.send_transaction(transaction).hex()
        
        print(f'{endpoint} call successful. tx_hash: {hash}')

    def set_address(self, endpoint: str, address: Address):
        if not hasattr(self, 'sc_address'):
            print(f'{endpoint} call failed: No SC deployed yet')
            return
        
        self.deployer_account.nonce = self.proxy.get_account(self.deployer_account.address).nonce
        transaction = self.factorySC.create_transaction_for_execute(self.deployer_account,
                                                                self.sc_address, 
                                                                endpoint,
                                                                config.GAS_DEPLOY,
                                                                [address],
                                                                0)
        transaction.nonce = self.deployer_account.nonce
        transaction.signature = self.deployer_account.sign_transaction(transaction)
        self.proxy.send_transaction(transaction)
        
        print(f'address setup successful')

