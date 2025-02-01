use dioxus::prelude::*;

use crate::DioxusWalletAdapter;

#[derive(PartialEq, Clone)]
pub enum AssetSelectMode {
    None,
    Input,
    Output,
}

#[component]
pub fn AssetSelect(mode: Signal<AssetSelectMode>) -> Element {
    let mut search_query = use_signal(|| String::new());

    if *mode.read() == AssetSelectMode::None {
        return rsx! {};
    }

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            onclick: move |_| mode.set(AssetSelectMode::None),

            // Modal content
            div {
                class: "bg-white rounded-lg w-[400px] max-h-[600px] flex flex-col",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "flex items-center justify-between px-4 py-2 border-b border-gray-200",
                    h3 { class: "text-lg font-medium",
                        if *mode.read() == AssetSelectMode::Input {
                            "Select from token"
                        } else {
                            "Select to token"
                        }
                    }
                    button {
                        class: "text-gray-500 hover:text-gray-700 text-2xl mb-2",
                        onclick: move |_| mode.set(AssetSelectMode::None),
                        "×"
                    }
                }

                // Search bar
                div { class: "p-4",
                    div { class: "relative",
                        span { class: "absolute inset-y-0 left-0 pl-3 flex items-center",
                            // Search icon
                            svg {
                                class: "h-5 w-5 text-gray-400",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                                }
                            }
                        }
                        input {
                            class: "w-full pl-10 pr-4 py-2 rounded-lg bg-gray-100 focus:outline-gray-500 focus:outline-1",
                            placeholder: "Search token name or address",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value().clone()),
                        }
                    }
                }

                // Token list
                div { class: "flex flex-col mb-2",
                    // Token items
                    TokenItem {
                        symbol: "USDC",
                        name: "Tether",
                        icon: "https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png",
                    }
                    TokenItem {
                        symbol: "USDC",
                        name: "USD Coin",
                        icon: "https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png",
                    }
                    TokenItem {
                        symbol: "SOL",
                        name: "Solana",
                        icon: "https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png",
                    }
                }
            }
        }
    }
}

#[component]
fn TokenItem(symbol: &'static str, name: &'static str, icon: &'static str) -> Element {
    let mut adapter: Signal<DioxusWalletAdapter> = use_context();
    let is_favorite = adapter.read().favorite_assets.contains(&symbol);

    let update_favorite = move |_| {
        let mut adapter = adapter.write();
        match is_favorite {
            true => adapter.favorite_assets.retain(|&s| s != symbol),
            false => adapter.favorite_assets.push(symbol),
        }
    };

    rsx! {
        div { class: "flex items-center p-4 hover:bg-gray-50 cursor-pointer",
            img { src: icon, alt: "Asset Icon", class: "w-6 h-6 mr-2" }
            div { class: "flex-1",
                div { class: "font-medium", "{symbol}" }
                div { class: "text-sm text-gray-500", "{name}" }
            }
            if is_favorite {
                button {
                    class: "text-yellow-400 hover:text-gray-600",
                    onclick: update_favorite,
                    "★"
                }
            } else {
                button {
                    class: "text-gray-400 hover:text-gray-600",
                    onclick: update_favorite,
                    "☆"
                }
            }
        }
    }
}
