use leptos::*;
use leptos_query::provide_query_client;
use thaw::Button;
use wasi_sol::{
    core::wallet::Wallet,
    forms::leptos::login::LoginForm,
    provider::leptos::{
        connection::ConnectionProvider,
        wallet::{use_wallet, WalletProvider},
    },
};

#[component]
pub fn App() -> impl IntoView {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        Wallet::Phantom.into(),
        Wallet::Solflare.into(),
        Wallet::Backpack.into(),
    ];

    provide_query_client();

    view! {
        <ConnectionProvider endpoint=endpoint>
            <WalletProvider wallets=wallets>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let phantom_context = use_wallet::<Wallet>(Wallet::Phantom);
    let solflare_context = use_wallet::<Wallet>(Wallet::Solflare);
    let backpack_context = use_wallet::<Wallet>(Wallet::Backpack);
    let (connected, set_connected) = create_signal(false);
    let (phantom_wallet_adapter, set_phantom_wallet_adapter) = create_signal(phantom_context);
    let (solflare_wallet_adapter, set_solflare_wallet_adapter) = create_signal(solflare_context);
    let (backpack_wallet_adapter, set_sbackpack_wallet_adapter) = create_signal(backpack_context);

    view! {
        <div>
            <div>
                <LoginForm
                    phantom=Some((phantom_wallet_adapter, set_phantom_wallet_adapter))
                    solflare=Some((solflare_wallet_adapter, set_solflare_wallet_adapter))
                    backpack=Some((backpack_wallet_adapter, set_sbackpack_wallet_adapter))
                    connected=(connected, set_connected)
                />
                <Button>"Click me"</Button>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| view! { <App /> })
}
