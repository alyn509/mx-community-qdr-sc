use multiversx_sc::imports::*;

/// Contains all storage mapper that the contract works with.
#[multiversx_sc::module]
pub trait QdrClaim: crate::qdr_views::QdrViews {
    #[endpoint(claimPromoRewards)]
    fn claim_promo_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let promo_reward = self.calculate_promo_rewards(&caller);
        require!(promo_reward > 0, "you have nothing to claim"); // you want to make a require so that is someone doesn't have something to claim to get at least the error message telling him that
        self.send().direct_egld(&caller, &promo_reward);
    }
}
