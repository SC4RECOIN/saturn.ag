use reactive_stores::Store;
use wasi_sol::core::wallet::BaseWalletAdapter;

#[derive(Clone, Default, Store)]
pub struct GlobalState {
    pub wallet_connected: bool,
    pub wallet_adapter: BaseWalletAdapter,
}
