use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait QdrClaim: crate::qdr_views::QdrViews {
    #[endpoint(claimPromoRewards)]
    fn claim_promo_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let promo_reward = self.calculate_promo_rewards(&caller);

        // you want to make a require so that is someone doesn't have something to claim to get at least the error message telling him that
        require!(promo_reward > 0, "you have nothing to claim");
        self.send().direct_egld(&caller, &promo_reward);
    }
}
