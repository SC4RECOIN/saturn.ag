use crate::components::wallet::WalletConnect;
use components::{providers::Providers, swap::Swap};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use wallet_adapter::WalletAdapter;

pub mod components;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct DioxusWalletAdapter {
    connection: WalletAdapter,
    show_modal: bool,
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(DioxusWalletAdapter {
            connection: WalletAdapter::init().unwrap(),
            show_modal: false,
        })
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/output.css") }

        div {
            class: "min-w-[min(1200px,100vw)] px-2 py-4",
            div {
                class: "flex justify-between mb-10 px-4",
                div {
                    class: "text-xl font-bold",
                    "Saturn 🪐"
                }
                WalletConnect {}
            }
            div {
                class: "flex flex-col lg:flex-row gap-8 justify-center",
                Swap {}
                Providers {}
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
