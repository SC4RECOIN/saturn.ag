use dioxus::prelude::*;

use crate::DioxusWalletAdapter;

enum Wallet {
    Phantom,
    Solflare,
    Backpack,
}

impl Wallet {
    fn as_str(&self) -> &'static str {
        match self {
            Wallet::Phantom => "Phantom",
            Wallet::Solflare => "Solflare",
            Wallet::Backpack => "Backpack",
        }
    }

    fn as_logo(&self) -> Asset {
        match self {
            Wallet::Phantom => asset!("/assets/phantom_logo.png"),
            Wallet::Solflare => asset!("/assets/solflare_logo.png"),
            Wallet::Backpack => asset!("/assets/backpack_logo.png"),
        }
    }
}

#[component]
pub fn WalletConnect(is_large: Option<bool>) -> Element {
    let mut adapter: Signal<DioxusWalletAdapter> = use_context();
    let connected = adapter.read().connection.is_connected();
    let show_modal = adapter.read().show_connect_modal;

    let mut error = use_signal(|| String::new());

    let mut connect_wallet = move |wallet: Wallet| {
        error.set(String::new());
    };

    let disconnect_wallet = move |_| {};

    rsx! {
        div {
            div {
                class: "bg-black text-white rounded-3xl w-full hover:bg-gray-800 text-center py-1",
                if !connected {
                    button {
                        class: if is_large.unwrap_or(false) {
                            "w-full h-full py-4 text-lg"
                        } else {
                            "w-full h-full px-4 py-1"
                        },
                        onclick: move |_| adapter.write().show_connect_modal = true,
                        "Connect Wallet"
                    }
                } else {
                    button {
                        onclick: disconnect_wallet,
                        class: "flex items-center gap-1 px-4",
                        img {
                            class: "w-8 h-8",
                            src: "https://www.istockphoto.com/photos/fire",
                            alt: "Wallet Icon"
                        }
                        "Disconnect"
                    }
                }
            }
            if show_modal {
                div {
                    class: "modal fixed inset-0 bg-black/50 flex justify-center items-center",
                    div {
                        class: "bg-white p-5 rounded-lg w-[400px] relative",
                        button {
                            class: "absolute top-2 right-5 text-gray-500 hover:text-black",
                            onclick: move |_| adapter.write().show_connect_modal = false,
                            span {
                                class: "text-3xl",
                                "×"
                            }
                        }
                        h2 {
                            class: "font-bold text-lg mb-4",
                            "Connect Wallet"
                        }
                        div {
                            class: "flex flex-row gap-4",
                            for wallet in [Wallet::Phantom, Wallet::Solflare, Wallet::Backpack].iter() {
                                button {
                                    class: "border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center",
                                    onclick: move |_| connect_wallet(*wallet),
                                    img {
                                        src: wallet.as_logo(),
                                        alt: "wallet logo",
                                        class: "w-20 h-20"
                                    }
                                    p { "{wallet.as_str()}" }
                                }
                            }
                            if !error.read().is_empty() {
                                div {
                                    class: "alert-error bg-red-50 border border-red-400 p-3 rounded",
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
