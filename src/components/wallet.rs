use leptos::*;
use wasi_sol::{
    core::{
        traits::WalletAdapter, wallet::{BaseWalletAdapter, Wallet}
    },
    provider::leptos::wallet::use_wallet,
};

#[component]
pub fn WalletConnect(large: bool) -> impl IntoView {
    let (phantom_wallet_adapter, set_phantom_wallet_adapter) = create_signal(use_wallet(Wallet::Phantom));
    let (solflare_wallet_adapter, set_solflare_wallet_adapter) = create_signal( use_wallet(Wallet::Solflare));
    let (backpack_wallet_adapter, set_backpack_wallet_adapter) = create_signal(use_wallet(Wallet::Backpack));

    let (connected, set_connected) = create_signal(false);
    let (error, set_error) = create_signal(String::default());
    let show = create_rw_signal(false);

    let connect_wallet = move |wallet_adapter: ReadSignal<BaseWalletAdapter>, set_wallet_adapter:  WriteSignal<BaseWalletAdapter>| {
        spawn_local(async move {
            let mut wallet_info = wallet_adapter.get();

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_wallet_adapter.set(wallet_info);
                        show.set(false);
                    }
                    set_connected.set(confirmed);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                    set_error.set(err.to_string());
                }
            }
        });
    };

    let disconnect_wallet = move |_| {
        spawn_local(async move {
            let mut phantom_wallet_info = phantom_wallet_adapter.get();
            let mut solflare_wallet_info = solflare_wallet_adapter.get();
            let mut backpack_wallet_info = backpack_wallet_adapter.get();

            match phantom_wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(phantom_wallet_info);
                    }
                    set_connected.set(!confirmed);
                }
                Err(_err) => {}
            }
            match solflare_wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_solflare_wallet_adapter.set(solflare_wallet_info);
                    }
                    set_connected.set(!confirmed);
                }
                Err(_err) => {}
            }
            match backpack_wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_backpack_wallet_adapter.set(backpack_wallet_info);
                    }
                    set_connected.set(!confirmed);
                }
                Err(_err) => {}
            }
        });
    };

    view! {
        <div>
            <div class=move || {
                let classes = "bg-black text-white rounded-2xl w-full px-3 hover:bg-gray-800 text-center";
                if large {
                    format!("{} py-4 text-lg w-full", classes)
                } else {
                    format!("{} py-1", classes)
                }
            }>
                {move || {
                    if !connected.get() {
                        view! { <button on:click=move |_| show.set(true)>"Connect Wallet"</button> }
                    } else {
                        view! {
                            <button on:click=disconnect_wallet>
                                <div class="flex items-center gap-2">
                                    <img
                                        class="w-8 h-8"
                                        src=if phantom_wallet_adapter.get().public_key().is_some() {
                                            phantom_wallet_adapter.get().icon()
                                        } else if solflare_wallet_adapter
                                            .get()
                                            .public_key()
                                            .is_some()
                                        {
                                            solflare_wallet_adapter.get().icon()
                                        } else if backpack_wallet_adapter
                                            .get()
                                            .public_key()
                                            .is_some()
                                        {
                                            backpack_wallet_adapter.get().icon()
                                        } else {
                                            "".to_string()
                                        }
                                        alt="Wallet Icon"
                                    />
                                    "Disconnect"
                                </div>
                            </button>
                        }
                    }
                }}
            </div>
            {move || {
                if show.get() {
                    view! {
                        <div class="modal fixed inset-0 bg-black/50 flex justify-center items-center">
                            <div class="bg-white p-5 rounded-lg w-[400px] relative">
                                <button
                                    class="absolute top-2 right-5 text-gray-500 hover:text-black"
                                    on:click=move |_| show.set(false)
                                >
                                    <span class="text-3xl">"×"</span>
                                </button>
                                <h2 class="font-bold text-lg mb-4">"Connect Wallet"</h2>
                                <div class="flex flex-col gap-4">
                                    <div class="flex justify-around gap-2">
                                        <button
                                            class="border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center"
                                            on:click=move |_| connect_wallet(
                                                phantom_wallet_adapter,
                                                set_phantom_wallet_adapter,
                                            )
                                        >
                                            <img
                                                src=phantom_wallet_adapter.get().icon()
                                                alt="Phantom Wallet"
                                                class="w-20 h-20"
                                            />
                                            <p>"Phantom"</p>
                                        </button>
                                        <button
                                            class="border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center"
                                            on:click=move |_| connect_wallet(
                                                solflare_wallet_adapter,
                                                set_solflare_wallet_adapter,
                                            )
                                        >
                                            <img
                                                src=solflare_wallet_adapter.get().icon()
                                                alt="Solflare Wallet"
                                                class="w-20 h-20"
                                            />
                                            <p>"Solflare"</p>
                                        </button>
                                        <button
                                            class="border border-gray-300 p-2 rounded hover:border-black cursor-pointer transition-colors w-32 flex flex-col items-center justify-center"
                                            on:click=move |_| connect_wallet(
                                                backpack_wallet_adapter,
                                                set_backpack_wallet_adapter,
                                            )
                                        >
                                            <img
                                                src=backpack_wallet_adapter.get().icon()
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
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}
