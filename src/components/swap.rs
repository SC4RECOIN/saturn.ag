use std::str::FromStr;

use leptos::*;
use thaw::{Image, Input, Space};
use wasi_sol::pubkey::Pubkey;

#[component]
pub fn Swap() -> impl IntoView {
    let input_mint =
        create_rw_signal(Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap());
    let output_mint =
        create_rw_signal(Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap());

    let input_value = create_rw_signal("0".to_string());
    let output_value = create_rw_signal("0".to_string());

    view! {
        <div class="swap-card">
            <div class="swap-header">
                <h3>"Swap"</h3>
                <div class="swap-settings">"Priority fee:"<span>"Market"</span></div>
            </div>
            <AssetSelector mint=input_mint amount=input_value />
            <div class="swap-arrow">"↓"</div>
            <AssetSelector mint=output_mint amount=output_value />
            <div class="swap-details">
                <div class="row">
                    <span>"Rate"</span>
                    <b>"1 USDC = 0.003683 SOL"</b>
                </div>
                <div class="row">
                    <span>"Slippage"</span>
                    <b>"0.1% ›"</b>
                </div>
                <div class="row">
                    <span>"Minimum received"</span>
                    <b>"1.839922 SOL"</b>
                </div>
            </div>
            <button class="connect-wallet">"Connect wallet"</button>
        </div>
    }
}

#[component]
pub fn AssetSelector(mint: RwSignal<Pubkey>, amount: RwSignal<String>) -> impl IntoView {
    let parser = Callback::<String, String>::new(move |v: String| {
        v.chars()
            .filter(|c| c.is_numeric() || *c == '.')
            .collect::<String>()
    });

    view! {
        <div class="asset-selector">
            <div class="token-selector">
                <img
                    src="https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png"
                    alt="Asset Icon"
                    class="token-icon"
                />
                <span class="token-symbol">"USDC"</span>
                <span class="dropdown-arrow">"▼"</span>
            </div>
            <Input value=amount parser class="amount-input" />
            <div class="usd-value">"$499.95"</div>
        </div>
    }
}
