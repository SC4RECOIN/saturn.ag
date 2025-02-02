use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum ProviderSource {
    OKX,
    Jupiter,
    FillCity,
}

impl ProviderSource {
    pub fn get_icon(&self) -> Asset {
        match self {
            ProviderSource::OKX => asset!("/assets/okx.webp"),
            ProviderSource::Jupiter => asset!("/assets/jupiter.webp"),
            ProviderSource::FillCity => asset!("/assets/fillcity.jpg"),
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            ProviderSource::OKX => "OKX DEX Aggregator",
            ProviderSource::Jupiter => "Jupiter",
            ProviderSource::FillCity => "FillCity",
        }
    }
}

#[component]
pub fn Providers() -> Element {
    rsx! {
        div { class: "rounded-2xl p-6 w-full shadow-md border border-gray-200",
            div { class: "flex justify-between mb-2",
                h3 { class: "font-bold text-lg", "Select provider" }
            }
            p { class: "text-gray-500 text-xs mb-5",
                "Available providers and aggregators are ranked by the estimated received value after network fees from this transaction. Select your choice of providers below."
            }
            div { class: "flex flex-col gap-4",
                for provider in [ProviderSource::OKX, ProviderSource::Jupiter] {
                    ProviderCard { provider }
                }
            }
        }
    }
}

#[component]
pub fn ProviderCard(provider: ProviderSource) -> Element {
    rsx! {
        div { class: "rounded-xl p-4 border border-gray-200 hover:border-black cursor-pointer transition-colors",
            div { class: "flex justify-between items-center mb-4",
                div { class: "flex items-center gap-2 font-bold text-sm",
                    img {
                        src: provider.get_icon(),
                        alt: "OKX",
                        class: "w-5 h-5 rounded-full",
                    }
                    div { "{provider.get_name()}" }
                }
                span { class: "px-2 py-1 rounded text-xs font-medium bg-green-500", "Best" }
            }
            span { class: "block text-lg font-medium text-gray-900 mb-1", "0.01952016 SOL" }
            span { class: "text-gray-500 text-sm", "≈ $5.01" }
            div { class: "mt-6 pt-4 border-t border-gray-200",
                div { class: "flex justify-between text-sm",
                    span { class: "text-gray-500", "Est network fee" }
                    span { "$0.0012" }
                }
            }
        }
    }
}
