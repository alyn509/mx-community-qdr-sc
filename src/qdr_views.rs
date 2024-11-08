use multiversx_sc::imports::*;

use crate::qdr_constants::PERCENTAGE_DIVISOR;
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct PurchasePosition<M: ManagedTypeApi> {
    pub purchase_amount: BigUint<M>,
    pub promo_reward_percentage: u64,
    pub has_reached_promo_min: bool,
}

/// Contains all storage mapper that the contract works with.
#[multiversx_sc::module]
pub trait QdrViews {
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
