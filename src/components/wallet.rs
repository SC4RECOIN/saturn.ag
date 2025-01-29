use std::str::FromStr;
use std::time::Duration;

use leptos::{  prelude::*, task::spawn_local};
use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use wasi_sol::core::{
        error::WalletError, traits::WalletAdapter, wallet::{BaseWalletAdapter, Wallet}
    };
use crate::state::{GlobalState, GlobalStateStoreFields};
use codee::{Decoder, Encoder};

#[component]
pub fn WalletConnect(#[prop(optional, default = false)] large: bool) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let (_, set_wallet_name, _) = use_local_storage::<WalletWrapper, WalletEncoder>("wallet_name");
    let (error, set_error) = signal(String::default());
    let show_modal = RwSignal::new(false);

    let connect_wallet = move |wallet: Wallet| {
        set_error.set(String::default());
        set_wallet_name.set(WalletWrapper::from(wallet.clone()));

        spawn_local(async move {
            let mut wallet_info = BaseWalletAdapter::from(wallet);
            let connected = wallet_info.connect().await;

            match connected {
                Ok(true) => {
                    state.wallet_adapter().set(wallet_info);
                    show_modal.set(false);
                    state.wallet_connected().set(true);
                }
                Ok(false) | Err(_) => {
                    let err = connected.err().unwrap_or(WalletError::WalletConnectionError);
                    log::error!("{}", err);
                    set_error.set(err.to_string());
                    set_wallet_name.set(WalletWrapper::default());
                }
            }
        });
    };

    let disconnect_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = state.wallet_adapter().get();

            match wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        state.wallet_adapter().set(wallet_info);
                        set_wallet_name.set(WalletWrapper::default());
                    }
                    state.wallet_connected().set(!confirmed);
                }
                Err(_err) => {}
            }
        });
    };

    view! {
        <div>
            <div class="bg-black text-white rounded-3xl w-full hover:bg-gray-800 text-center py-1">
                {move || {
                    if !state.wallet_connected().get() {
                        Some(
                            view! {
                                <button
                                    class=move || {
                                        if large {
                                            "w-full h-full py-4 text-lg"
                                        } else {
                                            "w-full h-full px-4 py-1"
                                        }
                                    }
                                    on:click=move |_| show_modal.set(true)
                                >
                                    "Connect Wallet"
                                </button>
                            },
                        )
                    } else {
                        None
                    }
                }}
                {move || {
                    if state.wallet_connected().get() {
                        Some(
                            view! {
                                <button
                                    on:click=disconnect_wallet
                                    class="flex items-center gap-1 px-4"
                                >
                                    <img
                                        class="w-8 h-8"
                                        src=state.wallet_adapter().get().icon()
                                        alt="Wallet Icon"
                                    />
                                    "Disconnect"
                                </button>
                            },
                        )
                    } else {
                        None
                    }
                }}
            </div>
            {move || {
                if show_modal.get() {
                    Some(
                        view! {
                            <div class="modal fixed inset-0 bg-black/50 flex justify-center items-center">
                                <div class="bg-white p-5 rounded-lg w-[400px] relative">
                                    <button
                                        class="absolute top-2 right-5 text-gray-500 hover:text-black"
                                        on:click=move |_| show_modal.set(false)
                                    >
                                        <span class="text-3xl">"×"</span>
                                    </button>
                                    <h2 class="font-bold text-lg mb-4">"Connect Wallet"</h2>
                                    <div class="flex flex-col gap-4">
                                        <div class="flex justify-around gap-2">
                                            <button
                                                class="border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center"
                                                on:click=move |_| connect_wallet(Wallet::Phantom)
                                            >
                                                <img
                                                    src="images/phantom_logo.png"
                                                    alt="Phantom Wallet"
                                                    class="w-20 h-20"
                                                />
                                                <p>"Phantom"</p>
                                            </button>
                                            <button
                                                class="border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center"
                                                on:click=move |_| connect_wallet(Wallet::Solflare)
                                            >
                                                <img
                                                    src="images/solflare_logo.png"
                                                    alt="Solflare Wallet"
                                                    class="w-20 h-20"
                                                />
                                                <p>"Solflare"</p>
                                            </button>
                                            <button
                                                class="border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center"
                                                on:click=move |_| connect_wallet(Wallet::Backpack)
                                            >
                                                <img
                                                    src="images/backpack_logo.png"
                                                    alt="Backpack Wallet"
                                                    class="w-20 h-20"
                                                />
                                                <p>"Backpack"</p>
                                            </button>
                                        </div>
                                        {move || {
                                            if !error.get().is_empty() {
                                                Some(
                                                    view! {
                                                        <div class="alert-error bg-red-50 border border-red-400 p-3 rounded">
                                                            <h4>"Error connecting wallet"</h4>
                                                            <p>{error.get()}</p>
                                                        </div>
                                                    },
                                                )
                                            } else {
                                                None
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                        },
                    )
                } else {
                    None
                }
            }}
        </div>
    }
}

pub fn use_autoconnect_wallet() {
    let state = expect_context::<Store<GlobalState>>();
    let (wallet_name, set_wallet_name, _) = use_local_storage::<WalletWrapper, WalletEncoder>("wallet_name");

    if wallet_name.get() != WalletWrapper::Unknown && !state.wallet_connected().get() {
        set_timeout(move || {
            spawn_local(async move {
                let wallet: Wallet = wallet_name.get().into();
                let mut wallet_info = BaseWalletAdapter::from(wallet);

                let connected = wallet_info.connect().await;
                match connected {
                    Ok(true) => {
                        state.wallet_adapter().set(wallet_info);
                        state.wallet_connected().set(true);
                    }
                    Ok(false) | Err(_) => {
                        let err = connected.err().unwrap_or(WalletError::WalletConnectionError);
                        log::error!("{}", err);
                        set_wallet_name.set(WalletWrapper::default());
                    }
                }
            });
        }, Duration::from_secs(1));
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum WalletWrapper {
    #[default]
    Unknown,
    Phantom,
    Solflare,
    Backpack,
}


impl From<Wallet> for WalletWrapper {
    fn from(val: Wallet) -> Self {
        match val {
            Wallet::Phantom => WalletWrapper::Phantom,
            Wallet::Solflare => WalletWrapper::Solflare,
            Wallet::Backpack => WalletWrapper::Backpack,
        }
    }
}

impl  Into<Wallet> for WalletWrapper {
    fn into(self) -> Wallet {
        match self {
            WalletWrapper::Phantom => Wallet::Phantom,
            WalletWrapper::Solflare => Wallet::Solflare,
            WalletWrapper::Backpack => Wallet::Backpack,
            WalletWrapper::Unknown => Wallet::Phantom,
        }
    }
}

impl FromStr for WalletWrapper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Phantom" => Ok(WalletWrapper::Phantom),
            "Solflare" => Ok(WalletWrapper::Solflare),
            "Backpack" => Ok(WalletWrapper::Backpack),
            _ => Ok(WalletWrapper::Unknown),
        }
    }
}

pub struct WalletEncoder;

impl Encoder<WalletWrapper> for WalletEncoder {
    type Error = String;
    type Encoded = String;

    fn encode(val: &WalletWrapper) -> Result<Self::Encoded, Self::Error> {
        Ok(format!("{:?}", val))
    }
}

impl Decoder<WalletWrapper> for WalletEncoder {
    type Error = String;
    type Encoded = str;

    fn decode(val: &Self::Encoded) -> Result<WalletWrapper, Self::Error> {
        WalletWrapper::from_str(val)
    }
}
