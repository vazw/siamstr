use crate::app::core_api::api::*;
use leptos::*;
use nostr_sdk::prelude::*;
use crate::app::nostr::nip07::Nip07Signer;

#[component]
pub fn UserPage(
    pub_key: RwSignal<String>,
    username: RwSignal<String>,
    lnurl: RwSignal<String>,
    use_lnurl: RwSignal<bool>,
    show_user: RwSignal<bool>,
    show_register: RwSignal<bool>,
) -> impl IntoView {
    let usernamed = username.get_untracked();
    let new_username = create_rw_signal(usernamed);
    let user = create_resource(move || new_username.get(), check_username);
    view! {
        <div class="text-start">
            <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg text-start">
                "สวัสดี  " {move || username.get()} "!!"
            </label>
            <br/>
            <label class="mt-6 text-[7px] leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg">
                {move || pub_key.get()}
            </label>
            <Transition fallback=move || {
                view! { <div>"Loading..."</div> }
            }>
                <div>
                    <label class="flex mt-3 text-sm text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg text-start">
                        <input
                            type="text"
                            class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                            prop:value=move || username.get()
                            on:input=move |ev| {
                                let val = event_target_value(&ev)
                                    .parse::<String>()
                                    .unwrap_or("".to_string());
                                new_username.set(val);
                            }
                        />

                        "@siamstr.com"
                        <UserGood username=username new_username=new_username user_resouce=user/>
                    </label>
                    <br/>
                    <label class="text-sm text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg text-start">
                        "LightningURL ที่มีอยู่" <br/>
                        <input
                            type="text"
                            class="text-gray-900 dark:text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                            prop:value=move || lnurl.get()
                            on:input=move |ev| {
                                let val = event_target_value(&ev)
                                    .parse::<String>()
                                    .unwrap_or("".to_string());
                                lnurl.set(val);
                            }
                        />

                    </label>

                    <br/>
                    <button
                        class="btn btn--delete"
                        on:click=move |_| {
                            let pubk = pub_key.get();
                            spawn_local(async move {
                                let signer = Nip07Signer::new()
                                    .expect("Not Found Nostr Extensions");
                                let pubkey = signer.get_public_key().await.unwrap();
                                let event = EventBuilder::new(
                                        Kind::TextNote,
                                        "Goodbye siamstr.com",
                                        [],
                                    )
                                    .to_unsigned_event(pubkey);
                                let signed_event: Event = signer.sign_event(event).await.unwrap();
                                let respon = delete_user(pubk, signed_event.as_json()).await;
                                match respon {
                                    Ok(result) => {
                                        if result.status == 1 {
                                            let window = web_sys::window().unwrap();
                                            window.alert_with_message("Done").unwrap();
                                            show_register.set(true);
                                            lnurl.set("".to_string());
                                            show_user.set(false);
                                            use_lnurl.set(false);
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
                                            .alert_with_message(
                                                "Something went wrong :( Please Refresh and Try again",
                                            )
                                            .unwrap();
                                        let _ = window.location().reload();
                                    }
                                }
                            });
                        }
                    >

                        "Delete"
                    </button>
                    <ButtonGood
                        username=username
                        new_username=new_username
                        pub_key=pub_key
                        lnurl=lnurl
                        user_resouce=user
                    />
                </div>
            </Transition>
        </div>
    }
}

#[component]
pub fn UserGood(
    username: RwSignal<String>,
    new_username: RwSignal<String>,
    user_resouce: Resource<String, Result<BoolRespons, ServerFnError>>,
) -> impl IntoView {
    view! {
        <div class="text-xs mt-2 pl-6">
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
                                    if new_username.get().is_empty() {
                                        view! { <p>"ใช้ไม่ได้"</p> }
                                    } else if username.get() == new_username.get() {
                                        view! { <p class="text-green-500">"ใช้ได้"</p> }
                                    } else {
                                        match value.status {
                                            1 => {
                                                view! { <p class="text-red-500">"X"</p> }
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

#[component]
fn ButtonGood(
    username: RwSignal<String>,
    new_username: RwSignal<String>,
    pub_key: RwSignal<String>,
    lnurl: RwSignal<String>,
    user_resouce: Resource<String,
    Result<BoolRespons, ServerFnError>>
) -> impl IntoView {
    let on_summit = move |_| {
        let pubk = pub_key.get();
        let lnurlp = lnurl.get();
        let name = new_username.get();
        spawn_local(async move {
            let signer = Nip07Signer::new().expect("Not Found Nostr Extensions");
            let pubkey = signer.get_public_key().await.unwrap();
            let event = EventBuilder::new(Kind::TextNote, "Edit user siamstr.com", []).to_unsigned_event(pubkey);
            let signed_event: Event = signer.sign_event(event).await.unwrap();
            let respon = edit_user(name, pubk, lnurlp, signed_event.as_json()).await;
            match respon {
                Ok(result) => {
                    if result.status == 1 {
                        let window = web_sys::window().unwrap();
                        window.alert_with_message("Done").unwrap();
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
                        .alert_with_message(
                            "Something went wrong :( Please Refresh and Try again",
                        )
                        .unwrap();
                    let _ = window.location().reload();
                }
            }
        })
    };
    view! {
        <Suspense fallback=move || {
            view! {
                <svg
                    aria-hidden="true"
                    class="inline w-4 h-4 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600 relative"
                    viewBox="0 0 100 101"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M100 50.5908C100 78.2051 77.X6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                        fill="currentColor"
                    ></path>
                    <path
                        d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                        fill="currentFill"
                    ></path>
                </svg>
                <span class="sr-only">Loading...</span>
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
                                if new_username.get().is_empty() {
                                    view! {
                                        <button class="btn btn--edit cursor-not-allowed" disabled>
                                            "ยังกดแก้ไม่ได้นะ"
                                        </button>
                                    }
                                } else if new_username.get() == username.get() {
                                    view! {
                                        <button class="btn btn--edit" on:click=on_summit>
                                            "Edit"
                                        </button>
                                    }
                                } else {
                                    match value.status {
                                        1 => {
                                            view! {
                                                <button class="btn btn--edit cursor-not-allowed" disabled>
                                                    "ยังกดแก้ไม่ได้นะ"
                                                </button>
                                            }
                                        }
                                        0 => {
                                            view! {
                                                <button class="btn btn--edit" on:click=on_summit>
                                                    "Edit"
                                                </button>
                                            }
                                        }
                                        _ => {
                                            view! {
                                                <button class="btn btn--edit cursor-not-allowed" disabled>
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
    }
}




























