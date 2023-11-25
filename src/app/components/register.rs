use core::time::Duration;
use leptos::*;
use crate::app::nostr::nip07::{get_public_key, sign_event};
use serde_wasm_bindgen;
use crate::app::core_api::api::*;

async fn nostr_pubkey(_key: String) -> String {
    let key = get_public_key().await;
    let key: String = serde_wasm_bindgen::from_value(key).unwrap();
    key
}

#[component]
pub fn SignInPage() -> impl IntoView {
    let use_lnurl = create_rw_signal(false);
    let lnurl = create_rw_signal("".to_string());
    let show_input = create_rw_signal(false);
    let show_login = create_rw_signal(true);
    let (pub_key, set_pubkey) = create_signal("".to_string());
    let username = create_rw_signal("".to_string());
    let nostr_public_key = create_local_resource(pub_key, nostr_pubkey);
    let on_click = move |_| {
        let key = nostr_public_key.get().expect("Not Found Nostr Extensions");
        show_input.set(true);
        show_login.set(false);
        set_pubkey(key);
    };

    view! {
        <div class="block w-9/12 max-w-full bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 text-center justify-items-center p-5">
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
            </AnimatedShow>
            <RegisterPage
                show_input=show_input
                pub_key=pub_key
                username=username
                use_lnurl=use_lnurl
                lnurl=lnurl
            />
        </div>
    }
}

#[component]
fn ButtonGood(username: RwSignal<String>, pub_key: ReadSignal<String>, lnurl: RwSignal<String>,user_resouce: Resource<String, Result<BoolRespons, ServerFnError>>) -> impl IntoView {
    let on_summit = move |_| {
        spawn_local(async move {
            let _ = add_user(username.get(), pub_key.get(), lnurl.get()).await;
        });

    };
    view! {
        <div class="text-xs text-red-500 relative pt-5">
            <Suspense fallback=move || view! { <p>"กำลังตรวจสอบ"</p> }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"เกิดข้อผิดพลาด"</p> }
                }>
                    {move || {
                        user_resouce
                            .clone()
                            .get()
                            .map(|view| {
                                view.map(|value| {
                                    if username.get().is_empty() {
                                        view! {
                                            <button
                                                class="btn btn--secondary cursor-not-allowed"
                                                disabled
                                            >
                                                "Register"
                                            </button>
                                        }
                                    } else {
                                        match value.status {
                                            1 => {
                                                view! {
                                                    <button
                                                        class="btn btn--secondary cursor-not-allowed"
                                                        disabled
                                                    >
                                                        "Register"
                                                    </button>
                                                }
                                            }
                                            0 => {
                                                view! {
                                                    <button class="btn btn--primary" on:click=on_summit>
                                                        "Register"
                                                    </button>
                                                }
                                            }
                                            _ => {
                                                view! {
                                                    <button
                                                        class="btn btn--secondary cursor-not-allowed"
                                                        disabled
                                                    >
                                                        "Register"
                                                    </button>
                                                }
                                            }
                                        }
                                    }
                                })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
#[component]
fn UserGood(username: RwSignal<String>, user_resouce: Resource<String, Result<BoolRespons, ServerFnError>>) -> impl IntoView {
    view! {
        <div class="text-xs text-red-500 relative pt-5">
            <Suspense fallback=move || view! { <p>"กำลังตรวจสอบ"</p> }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"เกิดข้อผิดพลาด"</p> }
                }>
                    {move || {
                        user_resouce
                            .clone()
                            .get()
                            .map(|view| {
                                view.map(|value| {
                                    if username.get().is_empty() {
                                        view! { <p>""</p> }
                                    } else {
                                        match value.status {
                                            1 => view! { <p>"มีคนใช้แล้ว"</p> },
                                            0 => view! { <p>"ใช้ได้"</p> },
                                            _ => {
                                                view! {
                                                    <p>"เกิดข้อผิดพลาด"</p>
                                                }
                                            }
                                        }
                                    }
                                })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
pub fn RegisterPage(show_input: RwSignal<bool>, pub_key: ReadSignal<String>, username: RwSignal<String>, use_lnurl: RwSignal<bool>, lnurl: RwSignal<String>) -> impl IntoView {
    let user = create_resource(move || username.get(),check_username);
    view! {
        <AnimatedShow
            when=show_input
            show_class="fade-in-1000"
            hide_class="fade-out-1000"
            hide_delay=Duration::from_millis(1000)
        >
            <div>
                <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg">
                    "สวัสดี!! "
                </label>
                <Transition fallback=move || {
                    view! { <div>"Loading..."</div> }
                }>
                    <div>
                        <UserGood username=username user_resouce=user/>
                        <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                            "ตั้งชื่อ"
                            <input
                                type="text"
                                class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                                id="username"
                                prop:placeholder="username"
                                on:input=move |ev| {
                                    let val = event_target_value(&ev)
                                        .parse::<String>()
                                        .unwrap_or("".to_string());
                                    username.set(val);
                                }
                            />
                            "@siamstr.com"
                        </label>
                        <br/>
                        <label class="relative inline-flex items-center mt-5 cursor-pointer">
                            <input
                                type="checkbox"
                                value=""
                                class="sr-only peer"
                                on:change=move |_| {
                                    if use_lnurl.get() {
                                        use_lnurl.set(false)
                                    } else {
                                        use_lnurl.set(true)
                                    };
                                }
                            />

                            <div class="pt-2 w-9 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
                            <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                                ใช้เป็น LightningURL
                            </span>
                        </label>
                        <AnimatedShow
                            when=use_lnurl
                            show_class="fade-in-1000"
                            hide_class="fade-out-1000"
                            hide_delay=Duration::from_millis(50)
                        >
                            <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-sm md:text-lg">
                                "โปรดกรอก LightningURL ที่มีอยู่"
                            </label>
                            <br/>
                            <input
                                type="text"
                                class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                                id="lnurl"
                                prop:placeholder="vazw@getalby.com"
                                on:input=move |ev| {
                                    let val = event_target_value(&ev)
                                        .parse::<String>()
                                        .unwrap_or("".to_string());
                                    lnurl.set(val);
                                }
                            />

                            <br/>
                            <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-sm md:text-lg">
                                "กระบวนการนี้จะใช้การ Redirect ไปยังที่อยู่ LightningURL นี้"
                            </label>
                        </AnimatedShow>
                        <br/>
                        <ButtonGood
                            username=username
                            pub_key=pub_key
                            lnurl=lnurl
                            user_resouce=user
                        />
                    </div>
                </Transition>
            </div>
        </AnimatedShow>
    }
}


