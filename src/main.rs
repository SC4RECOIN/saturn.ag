use crate::components::wallet::WalletConnect;
use components::{providers::Providers, swap::Swap};
use leptos::prelude::*;
use reactive_stores::Store;
use state::GlobalState;

pub mod components;
pub mod state;

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(GlobalState {
        wallet_connected: false,
    }));

    view! {
        <div class="min-w-[min(1200px,100vw)] px-2 py-4">
            <div class="flex justify-between mb-10 px-4">
                <div class="text-xl font-bold">"Saturn 🪐"</div>
                <WalletConnect />
            </div>
            <div class="flex flex-col lg:flex-row gap-8 justify-center">
                <Swap />
                <Providers />
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App /> })
}
