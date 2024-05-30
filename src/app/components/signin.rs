use crate::app::components::loading::LoadingIndi;
use crate::app::components::register::*;
use crate::app::core_api::api::{check_npub, UserRespons};
use crate::app::nostr::nip07::Nip07Signer;
use core::time::Duration;
use leptos::*;
use nostr_sdk::prelude::*;
use std::str::FromStr;

async fn nostr_sign_event(_key: String) -> Event {
    let signer = Nip07Signer::new().expect("Not Found Nostr Extensions");
    let pubkey = signer
        .get_public_key()
        .await
        .expect("users allow access public key");
    let event =
        EventBuilder::new(Kind::Authentication, "Login siamstr.com", []).to_unsigned_event(pubkey);
    let signed_event: Event = signer
        .sign_event(event)
        .await
        .expect("users click sign event");
    signed_event
}

#[component]
pub fn SignInPage() -> impl IntoView {
    let use_lnurl = create_rw_signal(false);
    let lnurl = create_rw_signal("".to_string());
    let show_login = create_rw_signal(true);
    let show_user = create_rw_signal(false);
    let show_register = create_rw_signal(false);
    let pub_key = create_rw_signal("".to_string());
    let username = create_rw_signal("".to_string());
    let npub_check_res = create_resource(move || pub_key.get(), check_npub);
    let on_click = move |_| {
        let xpubkey = pub_key.get();
        spawn_local(async move {
            if Nip07Signer::is_available() {
                let events = nostr_sign_event(xpubkey).await;
                let key = events.pubkey.to_string();
                match check_npub(key.clone().to_owned()).await {
                    Ok(user) => match user.user {
                        Some(user) => {
                            pub_key.set(user.pubkey);
                            username.set(user.name);
                            lnurl.set(user.lightning_url);
                            show_login.set(false);
                            show_register.set(false);
                            show_user.set(true);
                        }
                        None => {
                            pub_key.set(key);
                            show_login.set(false);
                            show_register.set(true);
                        }
                    },
                    Err(_e) => {
                        window()
                            .alert_with_message(
                                "Something went wrong :( Please Refresh and Try again",
                            )
                            .expect("browser allow alert");
                        let _ = window().location().reload();
                    }
                }
            } else {
                window()
                    .alert_with_message("Nostr Extensions Not Found")
                    .expect(" browser allow alert ");
            }
        })
    };

    view! {
        <div class="block w-9/12 max-h-fit max-w-full bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 text-center justify-items-center p-5">
            <AnimatedShow
                when=show_login
                show_class="fade-in-1000"
                hide_class="fade-out-100"
                hide_delay=Duration::from_millis(100)
            >
                <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg">
                    "กด Login ด้วย Nostr Extensions"
                </label>
                <br/>
                <button class="btn btn--primary" on:click=on_click>
                    "Login"
                </button>
                <br/>
                <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg">
                    "- หรือ -"
                </label>
                <br/>
                <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                    "กรอก Nostr Public Key(npub)"
                </label>
                <br/>
                <input
                    type="text"
                    class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                    prop:placeholder="Nostr Public Key / npub"
                    on:input=move |ev| {
                        let val = event_target_value(&ev)
                            .parse::<String>()
                            .unwrap_or("".to_string());
                        pub_key.set(val);
                    }
                />

                <br/>
                <ButtonGood
                    show_register=show_register
                    show_login=show_login
                    pub_key=pub_key
                    user_resouce=npub_check_res
                />
            </AnimatedShow>
            <Suspense fallback=move || {
                view! {
                    <LoadingIndi/>
                }
            }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"เกิดข้อผิดพลาด"</p> }
                }>
                    {move || {
                        view! {
                            <RegisterPage
                                show_register=show_register
                                show_user=show_user
                                pub_key=pub_key
                                username=username
                                use_lnurl=use_lnurl
                                lnurl=lnurl
                            />
                        }
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn ButtonGood(
    show_register: RwSignal<bool>,
    show_login: RwSignal<bool>,
    pub_key: RwSignal<String>,
    user_resouce: Resource<String, Result<UserRespons, ServerFnError>>,
) -> impl IntoView {
    let on_click_regis = move |_| {
        let public_key = pub_key.get();
        let keys = PublicKey::from_str(&public_key);
        match keys {
            Ok(pubkey) => {
                pub_key.set(pubkey.to_hex());
                show_login.set(false);
                show_register.set(true);
            }
            Err(_) => {
                window()
                    .alert_with_message("Public Key ไม่ถูกต้อง")
                    .expect("alert!");
            }
        };
    };
    view! {
        <div class="text-xs text-red-500">
            <Suspense fallback=move || {
                view! {
                    <LoadingIndi/>
                }
            }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"เกิดข้อผิดพลาด"</p> }
                }>
                    {move || {
                        if pub_key.get().is_empty() {
                            view! {
                                <button class="btn btn--secondary cursor-not-allowed" disabled>
                                    "สมัครรับ Nip-05"
                                </button>
                            }
                        } else {
                            match user_resouce.clone().get().expect("server respon") {
                                Ok(value) => {
                                    match value.user {
                                        Some(_) => {
                                            view! {
                                                <button
                                                    class="btn btn--secondary cursor-not-allowed"
                                                    disabled
                                                >
                                                    "Public Key นี้ได้สมัครไว้แล้ว"
                                                </button>
                                            }
                                        }
                                        None => {
                                            view! {
                                                <button class="btn btn--primary" on:click=on_click_regis>
                                                    "สมัครรับ Nip-05"
                                                </button>
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    view! {
                                        <button
                                            class="btn btn--secondary cursor-not-allowed"
                                            disabled
                                        >
                                            "สมัครรับ Nip-05"
                                        </button>
                                    }
                                }
                            }
                        }
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn Blank() -> impl IntoView {
    view! { <p></p> }
}
