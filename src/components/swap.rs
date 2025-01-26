use leptos::*;
use std::str::FromStr;
use wasi_sol::pubkey::Pubkey;

use crate::components::wallet::WalletConnect;

#[component]
pub fn Swap() -> impl IntoView {
    let input_mint =
        create_rw_signal(Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap());
    let output_mint =
        create_rw_signal(Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap());

    let input_value = create_rw_signal("".to_string());
    let output_value = create_rw_signal("".to_string());

    view! {
        <div class="rounded-2xl p-6 w-full shadow-md border border-gray-200">
            <div class="flex justify-between mb-2">
                <h3 class="font-bold text-lg">"Swap"</h3>
                <div class="flex items-center text-sm">
                    "Priority fee:" <span class="text-green-600 ml-1 cursor-pointer">"Market"</span>
                </div>
            </div>
            <AssetSelector mint=input_mint amount=input_value />
            <div class="text-center my-2 text-gray-500">"↓"</div>
            <AssetSelector mint=output_mint amount=output_value />
            <div class="border border-gray-200 rounded-xl p-4 my-6 text-sm flex flex-col gap-2">
                <div class="flex justify-between">
                    <span class="text-gray-500">"Rate"</span>
                    <span>"1 USDC = 0.003683 SOL"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-500">"Slippage"</span>
                    <span>"0.1% ›"</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-500">"Minimum received"</span>
                    <span>"1.839922 SOL"</span>
                </div>
            </div>
            {move || { if true { Some(view! { <WalletConnect large=true /> }) } else { None } }}
        </div>
    }
}

#[component]
pub fn AssetSelector(mint: RwSignal<Pubkey>, amount: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="bg-gray-50 rounded-xl p-4 mb-3 flex justify-between items-center">
            <div class="flex items-center gap-2 cursor-pointer font-medium">
                <img
                    src="https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png"
                    alt="Asset Icon"
                    class="w-6 h-6"
                />
                <span>"USDC"</span>
                <span class="text-gray-500 text-xs">"▼"</span>
            </div>
            <div class="flex flex-col items-end">
                <input
                    type="number"
                    value=amount
                    on:change=move |e| {
                        amount.set(event_target_value(&e));
                    }
                    class="text-right border-none bg-transparent text-2xl font-medium focus:outline-none [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                    placeholder="0.0"
                />
                <div class="text-gray-500 text-sm">"$499.95"</div>
            </div>
        </div>
    }
}
