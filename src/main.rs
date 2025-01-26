use crate::components::wallet::WalletConnect;
use components::{providers::Providers, swap::Swap};
use leptos::*;
use leptos_query::provide_query_client;
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
                <div class="min-w-[min(1200px,100vw)] px-2 py-4">
                    <div class="flex justify-between mb-10 px-4">
                        <div class="text-xl font-bold">"Saturn 🪐"</div>
                        <div>
                            <WalletConnect />
                        </div>
                    </div>
                    <div class="flex flex-col lg:flex-row gap-8 justify-center">
                        <Swap />
                        <Providers />
                    </div>
                </div>
            </WalletProvider>
        </ConnectionProvider>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| view! { <App /> })
}
