mod contract_setup;
use contract_setup::*;

#[test]
fn test_purchase() {
    let mut state = QdrScTestState::new();
    state.deploy();
    state.purchase(BUYER1, 100, "");
    state.purchase(BUYER1, 100, "");
    state.purchase(BUYER2, 100, "");
    state.purchase(BUYER3, 100, "");
    state.purchase(BUYER4, 100, "");

    state.claim_promo_rewards(BUYER2, "");
}
