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
        <Space vertical=true>
            <h3>Swap</h3>
            <AssetSelector mint=input_mint amount=input_value />
            <AssetSelector mint=output_mint amount=output_value />
        </Space>
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
            <Space>
                <Space>
                    <Image
                        src="https://www.okx.com/cdn/web3/currency/token/784-0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC-1.png/type=default_350_0?v=1735272019523&x-oss-process=image/format,webp/ignore-error,1"
                        alt="Asset Icon"
                        width="32px"
                        height="32px"
                    />
                    <div>"USDC"</div>
                    <div>"▼"</div>
                </Space>
                <Input value=amount parser />
            </Space>
        </div>
    }
}
