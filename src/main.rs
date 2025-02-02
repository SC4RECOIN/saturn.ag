use crate::components::wallet::WalletConnect;
use crate::database::tokens::TokenInfo;
use components::{providers::Providers, swap::Swap};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use gloo_storage::{LocalStorage, Storage};
use wallet_adapter::WalletAdapter;

pub mod components;
pub mod database;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct DioxusWalletAdapter {
    connection: WalletAdapter,
    show_connect_modal: bool,
    favorite_assets: Vec<String>,
    tokens: Vec<TokenInfo>,
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(DioxusWalletAdapter {
            connection: WalletAdapter::init().unwrap(),
            show_connect_modal: false,
            favorite_assets: LocalStorage::get::<Vec<String>>("favorite_assets")
                .unwrap_or_default(),
            tokens: vec![],
        })
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/output.css") }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Teko:wght@300..700&display=swa",
        }

        div { class: "min-w-[min(1200px,100vw)] px-2 py-4",
            div { class: "flex justify-between mb-10 px-4",
                div { class: "font-bold font-teko", "Saturn 🪐" }
                WalletConnect {}
            }
            div { class: "flex flex-col lg:flex-row gap-8 justify-center",
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
