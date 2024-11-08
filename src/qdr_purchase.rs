use crate::qdr_constants::*;
use multiversx_sc::imports::*;

// the purchase logic of the contract
// since the whole logic of purchase gets hard to follow we opted to isolate it from the rest of the contract

#[multiversx_sc::module]
pub trait QdrPurchase: crate::qdr_views::QdrViews {
    #[payable("*")]
    #[endpoint]
    fn purchase(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount > 0, "Must pay more than 0");
        let caller = self.blockchain().get_caller();
        self.handle_promo_purchase(&caller, &payment_amount);
        self.purchased_addresses().insert(caller);
        self.handle_fixed_rewards(payment_amount);
    }

    fn handle_promo_purchase(&self, caller: &ManagedAddress, payment_amount: &BigUint) {
        let mut total_promo_purchased = self.total_promo_purchased().get();
        let new_promo_purchase = total_promo_purchased + payment_amount.to_u64().unwrap();
        self.purchase_position(caller).update(|purchase_pos| {
            purchase_pos.purchase_amount += payment_amount;
            if !purchase_pos.has_reached_promo_min {
                let previous_promo_purchase = purchase_pos.purchase_amount.to_u64().unwrap();
                purchase_pos.purchase_amount += payment_amount.to_u64().unwrap();
                if purchase_pos.purchase_amount.to_u64().unwrap() >= PROMO_MIN_QDR {
                    purchase_pos.promo_reward_percentage = self
                        .calculate_promo_percentage(purchase_pos.purchase_amount.to_u64().unwrap());
                    purchase_pos.has_reached_promo_min = true;
                } else {
                    purchase_pos.promo_reward_percentage = self.calculate_promo_percentage(
                        previous_promo_purchase + payment_amount.to_u64().unwrap(),
                    )
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
        let _ = self.compute_and_send_reward(&payment_amount, QDR_MAG_ADDRESS, QDR_MAG_REWARD);
        let _ = self.compute_and_send_reward(&payment_amount, MA_ADDRESS, MA_FIX_REWARD);
        let _ = self.compute_and_send_reward(&payment_amount, TT_ADDRESS, TT_FIX_REWARD);
        let _ = self.compute_and_send_reward(&payment_amount, MB_ADDRESS, MB_FIX_REWARD);
        let promo_reward =
            self.compute_and_send_reward(&payment_amount, PROMO_ADDRESS, MAX_PERCENTAGE);

        self.handle_promo_rewards(promo_reward);
    }

    // Some code deduplication was required in handle_fixed_rewards since the formula was the same for all the addresses
    fn compute_and_send_reward(
        &self,
        payment_amount: &BigUint,
        address_bytes: [u8; 32],
        percentage: u64,
    ) -> BigUint {
        let managed_address = ManagedAddress::new_from_bytes(&address_bytes);
        let reward = payment_amount * percentage;
        self.send().direct_egld(&managed_address, &reward);
        reward
    }

    fn handle_promo_rewards(&self, total_promo_reward: BigUint) {
        let promo_address = ManagedAddress::new_from_bytes(&PROMO_ADDRESS);
        let total_promo_purchased = self.total_promo_purchased().get();
        let mut remaining_reward = total_promo_reward;
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
}
