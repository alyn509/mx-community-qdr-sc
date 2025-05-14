use multiversx_sc_scenario::imports::*;

use qdr_contract::*;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");

pub const QDR_MAG_ADDRESS: TestAddress = TestAddress::new("qdr-mag");
pub const MA_ADDRESS: TestAddress = TestAddress::new("ma");
pub const TT_ADDRESS: TestAddress = TestAddress::new("tt");
pub const MB_ADDRESS: TestAddress = TestAddress::new("mb");
pub const PROMO_ADDRESS: TestAddress = TestAddress::new("promo");

pub const BUYER1: TestAddress = TestAddress::new("buyer1");
pub const BUYER2: TestAddress = TestAddress::new("buyer2");
pub const BUYER3: TestAddress = TestAddress::new("buyer3");
pub const BUYER4: TestAddress = TestAddress::new("buyer4");

pub const QDR_SC_ADDRESS: TestSCAddress = TestSCAddress::new("qdr-sc");
pub const CODE_PATH: MxscPath = MxscPath::new("output/qdr-contract.mxsc.json");

pub const INIT_APY: u64 = 1200;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, qdr_contract::ContractBuilder);
    blockchain
}

pub struct QdrScTestState {
    pub world: ScenarioWorld,
}

impl QdrScTestState {
    pub fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);

        world.account(QDR_MAG_ADDRESS).nonce(1);
        world.account(MA_ADDRESS).nonce(1);
        world.account(TT_ADDRESS).nonce(1);
        world.account(MB_ADDRESS).nonce(1);
        world.account(PROMO_ADDRESS).nonce(1);

        world.account(BUYER1).nonce(1).balance(10_000_000);
        world.account(BUYER2).nonce(1).balance(10_000_000);
        world.account(BUYER3).nonce(1).balance(10_000_000);
        world.account(BUYER4).nonce(1).balance(10_000_000);

        Self { world }
    }

    pub fn deploy(&mut self) -> &mut Self {
        let deployed_address = self
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .init(INIT_APY)
            .code(CODE_PATH)
            .new_address(QDR_SC_ADDRESS)
            .returns(ReturnsNewAddress)
            .run();
        assert_eq!(deployed_address, QDR_SC_ADDRESS);
        self.make_setup();
        self
    }

    fn make_setup(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .set_qdr_mag_address(QDR_MAG_ADDRESS)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .set_ma_address(MA_ADDRESS)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .set_tt_address(TT_ADDRESS)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .set_mb_address(MB_ADDRESS)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .set_promo_address(PROMO_ADDRESS)
            .run();
    }

    pub fn purchase(&mut self, caller: TestAddress, egld_payment: u64, expected_error: &str) {
        let transaction = self
            .world
            .tx()
            .from(caller)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .purchase()
            .egld(egld_payment);

        if !expected_error.is_empty() {
            transaction
                .with_result(ExpectError(4, expected_error))
                .run();
        } else {
            transaction.run();
        }
    }

    pub fn claim_promo_rewards(&mut self, caller: TestAddress, expected_error: &str) {
        let transaction = self
            .world
            .tx()
            .from(caller)
            .to(QDR_SC_ADDRESS)
            .typed(qdr_contract_proxy::QdrContractProxy)
            .claim_promo_rewards();

        if !expected_error.is_empty() {
            transaction
                .with_result(ExpectError(4, expected_error))
                .run();
        } else {
            transaction.run();
        }
    }

    pub fn check_address_balance(&mut self, address: TestAddress, balance: u64) {
        self.world.check_account(address).balance(balance);
    }
}
