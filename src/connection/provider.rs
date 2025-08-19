use alloy::providers::{ProviderBuilder, DynProvider, Provider};
use tokio::sync::{OnceCell};
use once_cell::sync::Lazy;


// ---------- STATICS ----------
static ALCHEMY_API_KEY: Lazy<String> = Lazy::new(||{
    std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set")
});

// ---------- Getting API Keys ----------
fn get_alchemy_api_key() -> &'static str {
    &ALCHEMY_API_KEY
}



static _SEI_TESTNET: OnceCell<DynProvider> = OnceCell::const_new();
static _SEI_MAINNET: OnceCell<DynProvider> = OnceCell::const_new();



// https://sei-mainnet.g.alchemy.com/v2/NzinNDU09QHZRFLQb1dia_8XhAqL0zR7
// https://sei-testnet.g.alchemy.com/v2/NzinNDU09QHZRFLQb1dia_8XhAqL0zR7

// ---------- Providers ----------
pub async fn init_sei_mainnet() -> &'static DynProvider {
    _SEI_MAINNET.get_or_init(||async{
        ProviderBuilder::new()
        .connect(format!(
            "https://sei-mainnet.g.alchemy.com/v2/{}",
            get_alchemy_api_key()
        ).as_str())
        .await
        .expect("RPC init failed")
        .erased()
    }).await
}




pub async fn init_sei_testnet()-> &'static DynProvider {
    _SEI_TESTNET.get_or_init(||async{
        ProviderBuilder::new()
        .connect(format!(
            "https://sei-testnet.g.alchemy.com/v2/{}",
            get_alchemy_api_key()
        ).as_str())
        .await
        .expect("RPC init failed")
        .erased()
    }).await

    
}





// // ---------- Public Getters ----------
// pub fn get_scroll_mainnet_provider() -> &'static DynProvider {
//     init_scroll_mainnet()
// }

// pub fn get_scroll_sepolia_provider() -> &'static DynProvider {
//     init_scroll_testnet()
// }


