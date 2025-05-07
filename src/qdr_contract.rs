#![no_std]

multiversx_sc::imports!();

pub mod qdr_claim;
pub mod qdr_constants;
pub mod qdr_purchase;
pub mod qdr_setup;
pub mod qdr_views;

// not a general rule, but we usually leave the main file to contain just the init and the upgrade functions.
// Most of the logic of the SC we split it between some additional files so that the certain feature can be easier evaluated

#[multiversx_sc::contract]
pub trait QdrContract:
    qdr_views::QdrViews + qdr_purchase::QdrPurchase + qdr_claim::QdrClaim + qdr_setup::QdrSetup
{
    #[init]
    fn init(&self, apy: u64) {
        self.apy().set(apy);
        self.total_promo_purchased().set(BigUint::zero());
    }
}
