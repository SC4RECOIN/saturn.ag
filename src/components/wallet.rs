use dioxus::prelude::*;

use crate::DioxusWalletAdapter;

#[component]
pub fn WalletConnect(is_large: Option<bool>) -> Element {
    let mut adapter: Signal<DioxusWalletAdapter> = use_context();
    let connected = adapter.read().connection.is_connected();
    let show_modal = adapter.read().show_modal;

    let mut error = use_signal(|| String::new());

    let connect_wallet = move |_| error.set(String::new());

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
                        onclick: move |_| adapter.write().show_modal = true,
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
                            onclick: move |_| adapter.write().show_modal = false,
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
                            class: "flex flex-col gap-4",
                            div {
                                class: "flex justify-around gap-2",
                                button {
                                    class: "border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center",
                                    onclick: connect_wallet,
                                    img {
                                        src: asset!("/assets/phantom_logo.png"),
                                        alt: "Phantom Wallet",
                                        class: "w-20 h-20"
                                    }
                                    p { "Phantom" }
                                }
                                button {
                                    class: "border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center",
                                    onclick: connect_wallet,
                                    img {
                                        src: asset!("/assets/solflare_logo.png"),
                                        alt: "Solflare Wallet",
                                        class: "w-20 h-20"
                                    }
                                    p { "Solflare" }
                                }
                                button {
                                    class: "border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center",
                                    onclick: connect_wallet,
                                    img {
                                        src: asset!("/assets/backpack_logo.png"),
                                        alt: "Backpack Wallet",
                                        class: "w-20 h-20"
                                    }
                                    p { "Backpack" }
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
