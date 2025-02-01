use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::{
    components::{
        asset_select::{AssetSelect, AssetSelectMode},
        wallet::WalletConnect,
    },
    DioxusWalletAdapter,
};

#[component]
pub fn Swap() -> Element {
    let adapter: Signal<DioxusWalletAdapter> = use_context();
    let connected = adapter.read().connection.is_connected();

    let input_mint =
        use_signal(|| Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap());
    let output_mint =
        use_signal(|| Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap());

    let input_value = use_signal(|| String::new());
    let output_value = use_signal(|| String::new());

    rsx! {
        div { class: "rounded-2xl p-4 md:p-6 w-full shadow-md border border-gray-200",
            div { class: "flex justify-between mb-2",
                h3 { class: "font-bold text-lg", "Swap" }
                div { class: "flex items-center text-sm",
                    "Priority fee:"
                    span { class: "text-green-600 ml-1 cursor-pointer", "Market" }
                }
            }
            SwapEntry { mint: input_mint, amount: input_value }
            div { class: "text-center my-2 text-gray-500", "↓" }
            SwapEntry { mint: output_mint, amount: output_value, disabled: true }
            div { class: "border border-gray-200 rounded-xl p-4 my-6 text-sm flex flex-col gap-2",
                div { class: "flex justify-between",
                    span { class: "text-gray-500", "Rate" }
                    span { "1 USDC = 0.003683 SOL" }
                }
                div { class: "flex justify-between",
                    span { class: "text-gray-500", "Slippage" }
                    span { "0.1% ›" }
                }
                div { class: "flex justify-between",
                    span { class: "text-gray-500", "Minimum received" }
                    span { "1.839922 SOL" }
                }
            }
            if !connected {
                WalletConnect { is_large: true }
            } else {
                button {
                    class: "bg-black text-white rounded-3xl w-full hover:bg-gray-800 text-center py-1 py-4 text-lg",
                    onclick: move |_| {},
                    "Swap"
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct AssetSelectorProps {
    mint: Signal<Pubkey>,
    amount: Signal<String>,
    #[props(default = false)]
    disabled: bool,
}

#[component]
pub fn SwapEntry(mut props: AssetSelectorProps) -> Element {
    let mut mode = use_signal(|| AssetSelectMode::None);

    rsx! {
        div { class: "bg-gray-50 rounded-xl p-4 mb-3 flex justify-between items-center max-w-1/2",
            div {
                class: "flex items-center gap-2 cursor-pointer font-medium  active:scale-95 duration-100",
                onclick: move |_| mode.set(AssetSelectMode::Input),
                img {
                    src: "https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png",
                    alt: "Asset Icon",
                    class: "w-6 h-6",
                }
                span { "USDC" }
                span { class: "text-gray-500 text-xs", "▼" }
            }
            div { class: "flex flex-col items-end",
                input {
                    r#type: "number",
                    value: "{props.amount}",
                    oninput: move |evt| {
                        props.amount.set(evt.value().clone());
                    },
                    class: "w-full text-right border-none bg-transparent text-2xl font-medium focus:outline-none [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                    placeholder: "0.0",
                    disabled: "{props.disabled}",
                }
                div { class: "text-gray-500 text-sm", "$499.95" }
            }
        }
        AssetSelect { mode }
    }
}
