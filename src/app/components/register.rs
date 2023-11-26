use core::time::Duration;
use leptos::*;
use crate::app::core_api::api::*;
use crate::app::components::user::UserPage;

#[component]
pub fn RegisterPage(show_register: RwSignal<bool>, show_user: RwSignal<bool>, pub_key: RwSignal<String>, username: RwSignal<String>, use_lnurl: RwSignal<bool>, lnurl: RwSignal<String>) -> impl IntoView {
    let user = create_resource(move || username.get(),check_username);
    view! {
        <AnimatedShow
            when=show_register
            show_class="fade-in-1000"
            hide_class="fade-out-1000"
            hide_delay=Duration::from_millis(100)
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
                                    if val.is_empty() {
                                        username.set(val);
                                    } else {
                                        username.set(val);
                                    }
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

                            <div class="w-9 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
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
                            <label class="text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
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
                            show_register=show_register
                            show_user=show_user
                            username=username
                            pub_key=pub_key
                            lnurl=lnurl
                            user_resouce=user
                        />
                    </div>
                </Transition>
            </div>
        </AnimatedShow>
        <AnimatedShow
            when=show_user
            show_class="fade-in-1000"
            hide_class="fade-out-1000"
            hide_delay=Duration::from_millis(100)
        >
            <UserPage
                pub_key=pub_key
                username=username
                lnurl=lnurl
                use_lnurl=use_lnurl
                show_user=show_user
                show_register=show_register
            />
        </AnimatedShow>
    }
}

#[component]
fn ButtonGood(show_register: RwSignal<bool>, show_user: RwSignal<bool>, username: RwSignal<String>, pub_key: RwSignal<String>, lnurl: RwSignal<String>,user_resouce: Resource<String, Result<BoolRespons, ServerFnError>>) -> impl IntoView {
    let on_summit = move |_| {
        let pubk = pub_key.get();
        let name = username.get();
        let lnurlp = lnurl.get();
        let consume_pubket = pub_key.get();
        spawn_local(async move {
            let _ = add_user(name, consume_pubket, lnurlp).await;
        });
        show_register.set(false);
        pub_key.set(pubk);
        show_user.set(true);
    };
    view! {
        <div class="text-xs text-red-500 relative pt-5">
            <Suspense fallback=move || {
                view! {
                    <div role="status">
                        <svg
                            aria-hidden="true"
                            class="inline w-4 h-4 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
                            viewBox="0 0 100 101"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                fill="currentColor"
                            ></path>
                            <path
                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                fill="currentFill"
                            ></path>
                        </svg>
                        <span class="sr-only">Loading...</span>
                    </div>
                }
            }>
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
                                                    <button class="btn btn--primary" on:click=on_summit>
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
pub fn UserGood(username: RwSignal<String>, user_resouce: Resource<String, Result<BoolRespons, ServerFnError>>) -> impl IntoView {
    view! {
        <div class="text-xs relative">
            <Suspense fallback=move || {
                view! {
                    <div role="status">
                        <svg
                            aria-hidden="true"
                            class="inline w-4 h-4 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
                            viewBox="0 0 100 101"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                fill="currentColor"
                            ></path>
                            <path
                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                fill="currentFill"
                            ></path>
                        </svg>
                        <span class="sr-only">Loading...</span>
                    </div>
                }
            }>
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




















