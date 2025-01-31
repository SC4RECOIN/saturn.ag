use dioxus::prelude::*;

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
                div { class: "flex items-center justify-between p-4 border-b border-gray-200",
                    h3 { class: "text-lg font-medium", "Select pay token" }
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
                                xmlns: "http://www.w3.org/2000/svg",
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
                            class: "w-full pl-10 pr-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "Search token name or address",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value().clone()),
                        }
                    }
                }

                // Token list
                div { class: "flex-1 overflow-y-auto",
                    div { class: "divide-y",
                        // Token items
                        TokenItem { symbol: "USDT", name: "Tether", icon: "🟢" }
                        TokenItem { symbol: "USDC", name: "USD Coin", icon: "🔵" }
                        TokenItem { symbol: "SOLL", name: "Solana", icon: "◎" }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenItem(symbol: &'static str, name: &'static str, icon: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center p-4 hover:bg-gray-50 cursor-pointer",
            div { class: "w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 mr-3",
                "{icon}"
            }
            div { class: "flex-1",
                div { class: "font-medium", "{symbol}" }
                div { class: "text-sm text-gray-500", "{name}" }
            }
            button { class: "text-gray-400 hover:text-gray-600", "☆" }
        }
    }
}
