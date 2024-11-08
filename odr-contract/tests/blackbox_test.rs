use multiversx_sc_scenario::imports::*;

use odr_contract::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const CODE_PATH: MxscPath = MxscPath::new("output/odr-contract.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);
}
