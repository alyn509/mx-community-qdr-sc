use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/odr-contract.mxsc.json", odr_contract::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/odr_contract.scen.json");
}
