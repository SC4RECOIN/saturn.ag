use leptos::{  prelude::*, task::spawn_local};
use reactive_stores::Store;
use crate::state::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn WalletConnect(#[prop(optional, default = false)] large: bool) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let (error, set_error) = signal(String::default());
    let show_modal = RwSignal::new(false);

    let connect_wallet = move || {
        set_error.set(String::default());
    };

    let disconnect_wallet = move |_| {
        spawn_local(async move {
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
                                        src="https://www.istockphoto.com/photos/fire"
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
                                                on:click=move |_| connect_wallet()
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
                                                on:click=move |_| connect_wallet()
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
                                                on:click=move |_| connect_wallet()
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
