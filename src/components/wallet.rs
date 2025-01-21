use leptos::*;
use thaw::{Alert, AlertVariant, Button, ButtonVariant, Image, Modal, Space, SpaceJustify};
use wasi_sol::{
    core::{
        wallet::Wallet,
        traits::WalletAdapter
    },
    provider::leptos::wallet::use_wallet,
};

#[component]
pub fn WalletConnect() -> impl IntoView {
    let (phantom_wallet_adapter, set_phantom_wallet_adapter) = create_signal(use_wallet(Wallet::Phantom));
    let (solflare_wallet_adapter, set_solflare_wallet_adapter) = create_signal( use_wallet(Wallet::Solflare));
    let (backpack_wallet_adapter, set_backpack_wallet_adapter) = create_signal(use_wallet(Wallet::Backpack));

    let (connected, set_connected) = create_signal(false);
    let show = create_rw_signal(false);
    let (error, set_error) = create_signal(String::default());

    let connect_phantom_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = phantom_wallet_adapter.get();

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(wallet_info);
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

    let connect_solflare_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = solflare_wallet_adapter.get();

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(wallet_info);
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

    let connect_backpack_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = backpack_wallet_adapter.get();

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(wallet_info);
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
            {move || {
                if !connected.get() {
                    view! { <Button on_click=move |_| show.set(true)>"Connect Wallet"</Button> }
                } else {
                    view! {
                        <Button on:click=disconnect_wallet>
                            <Image
                                width="32px"
                                height="32px"
                                src=if phantom_wallet_adapter.get().public_key().is_some() {
                                    phantom_wallet_adapter.get().icon()
                                } else if solflare_wallet_adapter.get().public_key().is_some() {
                                    solflare_wallet_adapter.get().icon()
                                } else if backpack_wallet_adapter.get().public_key().is_some() {
                                    backpack_wallet_adapter.get().icon()
                                } else {
                                    "".to_string()
                                }
                                alt="Wallet Icon"
                            />
                            "Disconnect"
                        </Button>
                    }
                }
            }} <Modal title="Connect Wallet" show width="400px">
                <Space vertical=true>
                    {move || {
                        view! {
                            <Space justify=SpaceJustify::SpaceAround>
                                <Button
                                    variant=ButtonVariant::Outlined
                                    on:click=connect_phantom_wallet
                                    class="wallet-button"
                                >
                                    <Image
                                        src=phantom_wallet_adapter.get().icon()
                                        alt="Phantom Wallet"
                                    />
                                    <p>"Phantom"</p>
                                </Button>
                                <Button
                                    on:click=connect_solflare_wallet
                                    variant=ButtonVariant::Outlined
                                    class="wallet-button"
                                >
                                    <Image
                                        src=solflare_wallet_adapter.get().icon()
                                        alt="Solflare Wallet"
                                    />
                                    <p>"Solflare"</p>
                                </Button>
                                <Button
                                    on:click=connect_backpack_wallet
                                    variant=ButtonVariant::Outlined
                                    class="wallet-button"
                                >
                                    <Image
                                        src=backpack_wallet_adapter.get().icon()
                                        alt="Backpack Wallet"
                                    />
                                    <p>"Backpack"</p>
                                </Button>
                            </Space>
                        }
                    }}
                    {move || {
                        if !error.get().is_empty() {
                            Some(
                                view! {
                                    <Alert
                                        variant=AlertVariant::Error
                                        title="Error connecting wallet"
                                    >
                                        {error.get()}
                                    </Alert>
                                },
                            )
                        } else {
                            None
                        }
                    }}
                </Space>
            </Modal>
        </div>
    }
}
