use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use wallet_adapter::Wallet;

use crate::DioxusWalletAdapter;

#[component]
pub fn WalletConnect(is_large: Option<bool>) -> Element {
    let mut adapter: Signal<DioxusWalletAdapter> = use_context();
    let mut error = use_signal(|| String::new());

    let build_wallet_btn = |wallet: Wallet| -> Element {
        let icon = wallet.icon().as_ref().unwrap().to_string();
        rsx! {
            button {
                onclick: move |_| {
                    let wallet = wallet.clone();
                    spawn(async move {
                        let res = adapter.write().connection.connect(wallet).await;
                        match res {
                            Ok(_) => adapter.write().show_connect_modal = false,
                            Err(e) => error.set(e.to_string()),
                        }
                    });
                },
                class: "border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center",
                img { src: icon, alt: "wallet logo", class: "w-20 h-20" }
                {wallet.name()}
            }
        }
    };

    let disconnect_wallet = move |_| {
        spawn(async move {  
        let res = adapter.write().connection.disconnect().await;
        match res {
            Ok(_) => adapter.write().show_connect_modal = false,
                Err(e) => info!("Error disconnecting wallet: {}", e),
            }
        });
    };

    rsx! {
        div {
            div { class: "bg-black text-white rounded-3xl w-full hover:bg-gray-800 text-center py-1",
                if !adapter.read().connection.is_connected() {
                    button {
                        class: if is_large.unwrap_or(false) { "w-full h-full py-4 text-lg" } else { "w-full h-full px-4 py-1" },
                        onclick: move |_| adapter.write().show_connect_modal = true,
                        "Connect Wallet"
                    }
                } else {
                    button {
                        onclick: disconnect_wallet,
                        class: "flex items-center gap-1 px-4",
                        img {
                            class: "w-8 h-8",
                            src: adapter
                                .read()
                                .connection
                                .connected_wallet()
                                .as_ref()
                                .unwrap()
                                .icon()
                                .as_ref()
                                .unwrap()
                                .to_string(),
                            alt: "Wallet Icon",
                        }
                        "Disconnect"
                    }
                }
            }
            if adapter.read().show_connect_modal {
                div { class: "modal fixed inset-0 bg-black/50 flex justify-center items-center",
                    div { class: "bg-white p-5 rounded-lg w-[400px] relative",
                        button {
                            class: "absolute top-2 right-5 text-gray-500 hover:text-black",
                            onclick: move |_| adapter.write().show_connect_modal = false,
                            span { class: "text-3xl", "×" }
                        }
                        h2 { class: "font-bold text-lg mb-4", "Connect Wallet" }
                        div { class: "flex flex-row gap-4",
                            for wallet in adapter.read().connection.wallets() {
                                {build_wallet_btn(wallet)}
                            }
                            if !error.read().is_empty() {
                                div { class: "alert-error bg-red-50 border border-red-400 p-3 rounded",
                                    h4 { "Error connecting wallet" }
                                    p { "{error.read()}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
