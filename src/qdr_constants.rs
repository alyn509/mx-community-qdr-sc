use multiversx_sc::hex_literal::hex;

// a file where you store all your constants can never be a bad idea :)

pub const BLOCKS_IN_YEAR: u64 = 60 * 60 * 24 * 365 / 6;

// in order to simplify the contract we opted to switch off the use of floats, but rather work straigth up with u64 and have a percentage divisor ready to plug in where needed
pub const MAX_PERCENTAGE: u64 = 10000;
pub const QDR_MAG_REWARD: u64 = 50000;
pub const MA_FIX_REWARD: u64 = 20000;
pub const TT_FIX_REWARD: u64 = 20000;
pub const MB_FIX_REWARD: u64 = 2000;
// pub const PROMO_TOTAL_REWARD: u64 = 7500;   // dead code
pub const PROMO_BASE_PERCENTAGE: u64 = 15;

pub const PERCENTAGE_DIVISOR: u64 = 1000000;

pub const PROMO_MIN_QDR: u64 = 1000;
pub const PROMO_MAX_QDR: u64 = 100000;
pub const PROMO_MAX_TOTAL_QDR: u64 = 50000000;

// here a converter was used in order to have the address as hex value so that we can easily put in in a ManagedAddress
// long term you would like to have storages for these addressed and set them by or after init.
// it is not encouraged to hardcode addresses into the smart contract, especially if you ever get to the point where you would like to change any

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
