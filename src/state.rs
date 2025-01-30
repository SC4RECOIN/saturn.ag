use reactive_stores::Store;

#[derive(Store)]
pub struct GlobalState {
    pub wallet_connected: bool,
}
