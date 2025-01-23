use crate::components::wallet::WalletConnect;
use components::{providers::Providers, swap::Swap};
use leptos::*;
use leptos_query::provide_query_client;
use thaw::{Grid, GridItem, Space, SpaceGap};
use wasi_sol::{
    core::wallet::Wallet,
    provider::leptos::{connection::ConnectionProvider, wallet::WalletProvider},
};

pub mod components;

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
    view! {
        <div class="content">
            <Grid cols=5>
                <GridItem offset=4>
                    <WalletConnect />
                </GridItem>
            </Grid>
            <Space gap=SpaceGap::Size(32)>
                <Swap />
                <Providers />
            </Space>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| view! { <App /> })
}
