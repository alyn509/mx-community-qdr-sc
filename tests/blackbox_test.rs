mod contract_setup;
use contract_setup::*;
use multiversx_sc::types::ReturnsRawResult;
use multiversx_sc_scenario::{managed_biguint, ScenarioTxWhitebox};
use qdr_contract::qdr_views::QdrViews;

#[test]
fn test_purchase() {
    let mut state = QdrScTestState::new();
    state.deploy();
    state.purchase(BUYER1, 100, "");
    state.check_address_balance(QDR_MAG_ADDRESS, 5);
    state.check_address_balance(MA_ADDRESS, 2);
    state.check_address_balance(TT_ADDRESS, 2);
    state.check_address_balance(MB_ADDRESS, 0);
    state.check_address_balance(PROMO_ADDRESS, 2);
    state
        .world
        .query()
        .to(QDR_SC_ADDRESS)
        .returns(ReturnsRawResult)
        .whitebox(qdr_contract::contract_obj, |sc| {
            let buyer1_managed = BUYER1.to_managed_address();
            assert_eq!(sc.purchased_addresses().len(), 1);
            assert!(sc.purchased_addresses().contains(&buyer1_managed));

            let position = sc.purchase_position(&buyer1_managed).get();
            assert_eq!(position.purchase_amount, managed_biguint!(200));
            assert_eq!(position.promo_reward_percentage, 0);
            assert_eq!(position.has_reached_promo_min, false);

            let total_promo = sc.total_promo_purchased().get();
            assert_eq!(total_promo, managed_biguint!(100));
        });

    state.purchase(BUYER1, 1_000, "");
    state.purchase(BUYER2, 100, "");
    state.purchase(BUYER3, 100, "");
    state.check_address_balance(QDR_MAG_ADDRESS, 65);
    state.check_address_balance(MA_ADDRESS, 26);
    state.check_address_balance(TT_ADDRESS, 26);
    state.check_address_balance(MB_ADDRESS, 2);
    state.check_address_balance(PROMO_ADDRESS, 26);
    state
        .world
        .query()
        .to(QDR_SC_ADDRESS)
        .returns(ReturnsRawResult)
        .whitebox(qdr_contract::contract_obj, |sc| {
            let buyer1_managed = BUYER1.to_managed_address();
            let buyer2_managed = BUYER2.to_managed_address();
            let buyer3_managed = BUYER3.to_managed_address();
            assert_eq!(sc.purchased_addresses().len(), 3);
            assert!(sc.purchased_addresses().contains(&buyer1_managed));
            assert!(sc.purchased_addresses().contains(&buyer2_managed));
            assert!(sc.purchased_addresses().contains(&buyer3_managed));

            let position_buyer1 = sc.purchase_position(&buyer1_managed).get();
            assert_eq!(position_buyer1.purchase_amount, managed_biguint!(2200));
            assert_eq!(position_buyer1.promo_reward_percentage, 30);
            assert_eq!(position_buyer1.has_reached_promo_min, true);

            let position_buyer2 = sc.purchase_position(&buyer2_managed).get();
            assert_eq!(position_buyer2.purchase_amount, managed_biguint!(200));
            assert_eq!(position_buyer2.promo_reward_percentage, 0);
            assert_eq!(position_buyer2.has_reached_promo_min, false);

            let position_buyer3 = sc.purchase_position(&buyer3_managed).get();
            assert_eq!(position_buyer3.purchase_amount, managed_biguint!(200));
            assert_eq!(position_buyer3.promo_reward_percentage, 0);
            assert_eq!(position_buyer3.has_reached_promo_min, false);

            let total_promo = sc.total_promo_purchased().get();
            assert_eq!(total_promo, managed_biguint!(1_300));
        });

    state.purchase(BUYER4, 200_000, "");
    state.check_address_balance(QDR_MAG_ADDRESS, 10_065);
    state.check_address_balance(MA_ADDRESS, 4_026);
    state.check_address_balance(TT_ADDRESS, 4_026);
    state.check_address_balance(MB_ADDRESS, 402);
    state.check_address_balance(PROMO_ADDRESS, 4_017);
    state
        .world
        .query()
        .to(QDR_SC_ADDRESS)
        .returns(ReturnsRawResult)
        .whitebox(qdr_contract::contract_obj, |sc| {
            let buyer4_managed = BUYER4.to_managed_address();
            assert_eq!(sc.purchased_addresses().len(), 4);
            assert!(sc.purchased_addresses().contains(&buyer4_managed));

            let position = sc.purchase_position(&buyer4_managed).get();
            assert_eq!(position.purchase_amount, managed_biguint!(400_000));
            assert_eq!(position.promo_reward_percentage, 15);
            assert_eq!(position.has_reached_promo_min, true);

            let total_promo = sc.total_promo_purchased().get();
            assert_eq!(total_promo, managed_biguint!(201_300));
        });

    state.check_address_balance(BUYER1, 9_998_906);
    state.check_address_balance(BUYER4, 9_800_003);

    state.claim_promo_rewards(BUYER1, "");
    state.claim_promo_rewards(BUYER4, "");

    state.check_address_balance(BUYER1, 9_998_912); // 6 promo reward claimed
    state.check_address_balance(BUYER4, 9_800_006); // 3 promo reward claimed
}
