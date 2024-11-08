#![no_std]

use multiversx_sc::hex_literal::hex;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const BLOCKS_IN_YEAR: u64 = 60 * 60 * 24 * 365 / 6;

pub const MAX_PERCENTAGE: u64 = 10000;
pub const QDR_MAG_REWARD: u64 = 50000;
pub const MA_FIX_REWARD: u64 = 20000;
pub const TT_FIX_REWARD: u64 = 20000;
pub const MB_FIX_REWARD: u64 = 2000;
pub const PROMO_TOTAL_REWARD: u64 = 7500;
pub const PROMO_BASE_PERCENTAGE: u64 = 15;

pub const PERCENTAGE_DIVISOR: u64 = 1000000;

pub const PROMO_MIN_QDR: u64 = 1000;
pub const PROMO_MAX_QDR: u64 = 100000;
pub const PROMO_MAX_TOTAL_QDR: u64 = 50000000;
pub const QDR_MAG_ADDRESS: [u8; 32] =
    hex!("4a726379c0115a57ea6cc92790d7f58f1391b1216cea6530af1f9f7946bbb791"); // erd1ffexx7wqz9d906nveynep4l43ufervfpdn4x2v90r70hj34mk7gsqz9jdj
pub const MA_ADDRESS: [u8; 32] =
    hex!("c94f2baa9c1a1b5d195e051e11f60ecb0159f1f555afb8834ee7a286881d93d2"); // erd1e98jh25urgd46x27q50praswevq4nu042khm3q6wu73gdzqaj0fq89vxq4
pub const TT_ADDRESS: [u8; 32] =
    hex!("8007fba31724158eab8a6f1795e7a91a0d5aeeaaf0ccfee475b3306db5d7092a"); // erd1sqrlhgchys2ca2u2duteteafrgx44m427rx0aer4kvcxmdwhpy4qe7cn79
pub const MB_ADDRESS: [u8; 32] =
    hex!("2ece63a63fe92bbed4909a57983fc08a7ecc9cbc4e2cf9be2d21d611f16d1115"); // erd19m8x8f3lay4ma4ysnftes07q3flve89ufck0n03dy8tprutdzy2sq2uy8p
pub const PROMO_ADDRESS: [u8; 32] =
    hex!("58a57453e0aae3b8b68025e02a7633624b70e121f0ba72aa7458d85dbc45c31e"); // erd1tzjhg5lq4t3m3d5qyhsz5a3nvf9hpcfp7za892n5trv9m0z9cv0q6ym3m6

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct PurchasePosition<M: ManagedTypeApi> {
    pub purchase_amount: BigUint<M>,
    pub promo_reward_percentage: u64,
    pub has_reached_promo_min: bool,
}

#[multiversx_sc::contract]
pub trait QdrContract {
    #[init]
    fn init(&self, apy: u64) {
        self.apy().set(apy);
        self.total_promo_purchased().set(0u64);
    }

    #[payable("*")]
    #[endpoint]
    fn purchase(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount > 0, "Must pay more than 0");
        let caller = self.blockchain().get_caller();
        self.purchase_position(&caller).update(|purchase_pos| {
            purchase_pos.purchase_amount += payment_amount.clone();
        });
        self.handle_promo_purchase(caller.clone(), payment_amount.clone());
        self.purchased_addresses().insert(caller.clone());
        self.handle_fixed_rewards(payment_amount);
    }

    #[endpoint(claimPromoRewards)]
    fn claim_promo_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let promo_reward = self.calculate_promo_rewards(&caller);
        if promo_reward > 0 {
            self.send().direct_egld(&caller, &promo_reward);
        }
    }

    #[view(calculatePromoRewards)]
    fn calculate_promo_rewards(&self, addr: &ManagedAddress) -> BigUint {
        let purchase_pos = self.purchase_position(addr).get();
        let total_promo_purchased = self.total_promo_purchased().get();
        let promo_percentage = purchase_pos.promo_reward_percentage;
        let promo_reward = self.get_percentage(total_promo_purchased * promo_percentage);
        BigUint::from(promo_reward)
    }

    fn get_percentage(&self, amount: u64) -> u64 {
        amount / PERCENTAGE_DIVISOR
    }

    fn handle_promo_purchase(&self, caller: ManagedAddress, payment_amount: BigUint) {
        let mut total_promo_purchased = self.total_promo_purchased().get();
        let new_promo_purchase = total_promo_purchased + payment_amount.to_u64().unwrap();
        self.purchase_position(&caller).update(|purchase_pos| {
            if !purchase_pos.has_reached_promo_min {
                let previous_promo_purchase = purchase_pos.purchase_amount.to_u64().unwrap();
                purchase_pos.purchase_amount += payment_amount.to_u64().unwrap();
                if purchase_pos.purchase_amount.to_u64().unwrap() >= PROMO_MIN_QDR {
                    purchase_pos.promo_reward_percentage = self
                        .calculate_promo_percentage(purchase_pos.purchase_amount.to_u64().unwrap())
                        .into();
                    purchase_pos.has_reached_promo_min = true;
                } else {
                    purchase_pos.promo_reward_percentage = self
                        .calculate_promo_percentage(
                            previous_promo_purchase + payment_amount.to_u64().unwrap(),
                        )
                        .into();
                }
            }
        });
        if new_promo_purchase <= PROMO_MAX_TOTAL_QDR {
            total_promo_purchased = new_promo_purchase;
            self.total_promo_purchased().set(total_promo_purchased);
        }
    }

    fn calculate_promo_percentage(&self, promo_purchase_amount: u64) -> u64 {
        if promo_purchase_amount < PROMO_MIN_QDR {
            return 0;
        }
        if promo_purchase_amount > PROMO_MAX_QDR {
            return PROMO_BASE_PERCENTAGE;
        }
        PROMO_BASE_PERCENTAGE * (promo_purchase_amount / PROMO_MIN_QDR)
    }

    fn handle_fixed_rewards(&self, payment_amount: BigUint) {
        let mag_address = ManagedAddress::new_from_bytes(&QDR_MAG_ADDRESS);
        let ma_address = ManagedAddress::new_from_bytes(&MA_ADDRESS);
        let tt_address = ManagedAddress::new_from_bytes(&TT_ADDRESS);
        let mb_address = ManagedAddress::new_from_bytes(&MB_ADDRESS);
        let promo_address = ManagedAddress::new_from_bytes(&PROMO_ADDRESS);
        let promo_reward = &payment_amount * MAX_PERCENTAGE;
        self.send().direct_egld(&promo_address, &promo_reward);
        let mb_reward = &payment_amount * MB_FIX_REWARD;
        self.send().direct_egld(&mb_address, &mb_reward);
        let mag_reward = &payment_amount * QDR_MAG_REWARD;
        let ma_reward = &payment_amount * MA_FIX_REWARD;
        let tt_reward = &payment_amount * TT_FIX_REWARD;
        self.send().direct_egld(&mag_address, &mag_reward);
        self.send().direct_egld(&ma_address, &ma_reward);
        self.send().direct_egld(&tt_address, &tt_reward);
        self.handle_promo_rewards(promo_reward);
    }

    fn handle_promo_rewards(&self, total_promo_reward: BigUint) {
        let promo_address = ManagedAddress::new_from_bytes(&PROMO_ADDRESS);
        let total_promo_purchased = self.total_promo_purchased().get();
        let mut remaining_reward = total_promo_reward.clone();
        for addr in self.purchased_addresses().iter() {
            let purchase_pos = self.purchase_position(&addr).get();
            let user_reward =
                self.get_percentage(total_promo_purchased * purchase_pos.promo_reward_percentage);
            if user_reward > 0 && remaining_reward >= user_reward {
                self.send().direct_egld(&addr, &BigUint::from(user_reward));
                remaining_reward -= user_reward;
            }
        }
        self.send().direct_egld(&promo_address, &remaining_reward);
    }

    #[view(getPurchasedAddresses)]
    #[storage_mapper("purchasedAddresses")]
    fn purchased_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getPurchasePosition)]
    #[storage_mapper("purchasePosition")]
    fn purchase_position(
        &self,
        addr: &ManagedAddress,
    ) -> SingleValueMapper<PurchasePosition<Self::Api>>;

    #[view(getApy)]
    #[storage_mapper("apy")]
    fn apy(&self) -> SingleValueMapper<u64>;

    #[view(getTotalPromoPurchased)]
    #[storage_mapper("totalPromoPurchased")]
    fn total_promo_purchased(&self) -> SingleValueMapper<u64>;
}
