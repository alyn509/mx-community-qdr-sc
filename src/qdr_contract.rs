#![no_std]

multiversx_sc::imports!();

pub mod qdr_claim;
pub mod qdr_constants;
pub mod qdr_purchase;
pub mod qdr_views;

#[multiversx_sc::contract]
pub trait QdrContract:
    qdr_views::QdrViews + qdr_purchase::QdrPurchase + qdr_claim::QdrClaim
{
    #[init]
    fn init(&self, apy: u64) {
        self.apy().set(apy);
        self.total_promo_purchased().set(0u64);
    }
}
