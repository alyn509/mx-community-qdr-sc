use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait QdrSetup: crate::qdr_views::QdrViews {
    #[only_owner]
    #[endpoint(setQdrMagAddress)]
    fn set_qdr_mag_address(&self, address: ManagedAddress) {
        self.qdr_mag_address().set(address);
    }

    #[only_owner]
    #[endpoint(setMaAddress)]
    fn set_ma_address(&self, address: ManagedAddress) {
        self.ma_address().set(address);
    }

    #[only_owner]
    #[endpoint(setTtAddress)]
    fn set_tt_address(&self, address: ManagedAddress) {
        self.tt_address().set(address);
    }

    #[only_owner]
    #[endpoint(setMbAddress)]
    fn set_mb_address(&self, address: ManagedAddress) {
        self.mb_address().set(address);
    }

    #[only_owner]
    #[endpoint(setPromoAddress)]
    fn set_promo_address(&self, address: ManagedAddress) {
        self.promo_address().set(address);
    }
}
