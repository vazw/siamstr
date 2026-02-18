use crate::app::components::loading::LoadingIndi;
use crate::app::components::user::UserPage;
use crate::app::core_api::api::*;
use core::time::Duration;
use leptos::*;
use nostr_sdk::prelude::*;

#[component]
pub fn RegisterPage(
    show_register: RwSignal<bool>,
    show_user: RwSignal<bool>,
    pub_key: RwSignal<String>,
    username: RwSignal<String>,
    use_lnurl: RwSignal<bool>,
    lnurl: RwSignal<String>,
) -> impl IntoView {
    let user = create_resource(move || username.get(), check_username);
    view! {
        <AnimatedShow
            when={show_register}
            show_class="fade-in-1000"
            hide_class="fade-out-1000"
            hide_delay={Duration::from_millis(100)}
        >
            <div>
                <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg">
                    "สวัสดี!! "
                </label>
                <Transition fallback={move || {
                    view! { <div>"Loading..."</div> }
                }}>
                    <div>
                        <UserGood username={username} user_resource={user} />
                        <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                            "ตั้งชื่อ"
                            <input
                                type="text"
                                class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                                prop:placeholder="username"
                                on:input={move |ev| {
                                    let val = event_target_value(&ev)
                                        .parse::<String>()
                                        .unwrap_or("".to_string());
                                    username.set(val);
                                }}
                            /> "@siamstr.com"
                        </label>
                        <br />
                        <label class="relative inline-flex items-center cursor-pointer">
                            <input
                                type="checkbox"
                                value=""
                                class="sr-only peer"
                                on:change={move |_| {
                                    if use_lnurl.get() {
                                        use_lnurl.set(false)
                                    } else {
                                        use_lnurl.set(true)
                                    };
                                }}
                            />

                            <div class="w-9 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
                            <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                                ใช้เป็น LightningURL
                            </span>
                        </label>
                        <AnimatedShow
                            when={use_lnurl}
                            show_class="fade-in-1000"
                            hide_class="fade-out-1000"
                            hide_delay={Duration::from_millis(50)}
                        >
                            <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                                "โปรดกรอก LightningURL ที่มีอยู่"
                            </label>
                            <br />
                            <input
                                type="text"
                                class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                                prop:placeholder="vazw@getalby.com"
                                on:input={move |ev| {
                                    let val = event_target_value(&ev)
                                        .parse::<String>()
                                        .unwrap_or("".to_string());
                                    lnurl.set(val);
                                }}
                            />

                            <br />
                            <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-sm md:text-lg">
                                "กระบวนการนี้จะใช้การ Redirect ไปยังที่อยู่ LightningURL นี้"
                            </label>
                        </AnimatedShow>
                        <br />
                        <ButtonGood
                            show_register={show_register}
                            show_user={show_user}
                            username={username}
                            pub_key={pub_key}
                            lnurl={lnurl}
                            user_resource={user}
                        />
                    </div>
                </Transition>
            </div>
        </AnimatedShow>
        <AnimatedShow
            when={show_user}
            show_class="fade-in-1000"
            hide_class="fade-out-1000"
            hide_delay={Duration::from_millis(100)}
        >
            <UserPage
                pub_key={pub_key}
                username={username}
                lnurl={lnurl}
                use_lnurl={use_lnurl}
                show_user={show_user}
                show_register={show_register}
            />
        </AnimatedShow>
    }
}

#[component]
fn ButtonGood(
    show_register: RwSignal<bool>,
    show_user: RwSignal<bool>,
    username: RwSignal<String>,
    pub_key: RwSignal<String>,
    lnurl: RwSignal<String>,
    user_resource: Resource<String, Result<BoolRespons, ServerFnError>>, // # spellchecker:disable-line
) -> impl IntoView {
    let on_summit = move |_| {
        let pubk = pub_key.get();
        let name = username.get();
        let lnurlp = lnurl.get();
        let consume_pubket = pub_key.get();
        spawn_local(async move {
            let respon = add_user(name, consume_pubket, lnurlp).await;
            match respon {
                Ok(result) => {
                    if result.status == 1 {
                        let window = web_sys::window().unwrap();
                        window.alert_with_message("Done").unwrap();
                        show_register.set(false);
                        pub_key.set(pubk);
                        show_user.set(true);
                    } else {
                        let window = web_sys::window().unwrap();
                        window
                            .alert_with_message(
                                "Something went wrong :( Please Refresh and Try again",
                            )
                            .unwrap();
                        let _ = window.location().reload();
                    }
                }
                Err(_) => {
                    let window = web_sys::window().unwrap();
                    window
                        .alert_with_message("Something went wrong :( Please Refresh and Try again")
                        .unwrap();
                    let _ = window.location().reload();
                }
            }
        });
    };
    view! {
        <div class="text-xs text-red-500 relative">
            <Suspense fallback={move || {
                view! { <LoadingIndi /> }
            }}>
                // handles the error from the resource
                <ErrorBoundary fallback={|_| {
                    view! {
                        <p>"เกิดข้อผิดพลาด"</p>
                    }
                }}>
                    {move || {
                        user_resource
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
                                                "คิดชื่อนานอะ 😢"
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
                                                        "เอาใหม่ ๆ"
                                                    </button>
                                                }
                                            }
                                            0 => {
                                                view! {
                                                    <button class="btn btn--primary" on:click={on_summit}>
                                                        "สมัครรับ Nip-05"
                                                    </button>
                                                }
                                            }
                                            _ => {
                                                view! {
                                                    <button
                                                        class="btn btn--secondary cursor-not-allowed"
                                                        disabled
                                                    >
                                                        "😿"
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
pub fn UserGood(
    username: RwSignal<String>,
    user_resource: Resource<String, Result<BoolRespons, ServerFnError>>, // # spellchecker:disable-line
) -> impl IntoView {
    view! {
        <div class="text-xs relative">
            <Suspense fallback={move || {
                view! { <LoadingIndi /> }
            }}>
                // handles the error from the resource
                <ErrorBoundary fallback={|_| {
                    view! {
                        <p>"เกิดข้อผิดพลาด"</p>
                    }
                }}>
                    {move || {
                        user_resource
                            .clone()
                            .get()
                            .map(|view| {
                                view.map(|value| {
                                    if username.get().is_empty() {
                                        view! { <p>""</p> }
                                    } else {
                                        match value.status {
                                            1 => {
                                                view! {
                                                    <p class="text-red-500">
                                                        "มีคนใช้แล้ว"
                                                    </p>
                                                }
                                            }
                                            0 => {
                                                view! { <p class="text-green-500">"ใช้ได้"</p> }
                                            }
                                            _ => {
                                                view! {
                                                    <p class="text-red-500">
                                                        "เกิดข้อผิดพลาด"
                                                    </p>
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
