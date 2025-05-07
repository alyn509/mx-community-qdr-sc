use multiversx_sc::imports::*;

use crate::qdr_constants::PERCENTAGE_DIVISOR;
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct PurchasePosition<M: ManagedTypeApi> {
    pub purchase_amount: BigUint<M>,
    pub promo_reward_percentage: u64,
    pub has_reached_promo_min: bool,
}

/// Contains all views and storage mapper that the contract works with
#[multiversx_sc::module]
pub trait QdrViews {
    #[view(calculatePromoRewards)]
    fn calculate_promo_rewards(&self, addr: &ManagedAddress) -> BigUint {
        let purchase_pos = self.purchase_position(addr).get();
        let total_promo_purchased = self.total_promo_purchased().get();
        let promo_percentage = purchase_pos.promo_reward_percentage;
        // here where we calculate the reward straigth up we need to remember to get rid of the PERCENTAGE_DIVISOR we inserted before in order to get rid of floats
        self.get_percentage(total_promo_purchased * promo_percentage)
    }

    fn get_percentage(&self, amount: BigUint) -> BigUint {
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
    fn total_promo_purchased(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("qdrMagAddress")]
    fn qdr_mag_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("maAddress")]
    fn ma_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("ttAddress")]
    fn tt_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("mbAddress")]
    fn mb_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("promoAddress")]
    fn promo_address(&self) -> SingleValueMapper<ManagedAddress>;
}
