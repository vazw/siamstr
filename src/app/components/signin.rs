use crate::app::components::register::*;
use crate::app::core_api::api::check_npub;
use core::time::Duration;
use leptos::*;
use nostr_sdk::prelude::*;
use crate::app::nostr::nip07::Nip07Signer;


async fn nostr_sign_event(_key: String) -> Event {
    let signer = Nip07Signer::new().expect("Not Found Nostr Extensions");
    let pubkey = signer.get_public_key().await.unwrap();
    let event = EventBuilder::new(Kind::Authentication, "Login siamstr.com", []).to_unsigned_event(pubkey);
    let signed_event: Event = signer.sign_event(event).await.unwrap();
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
    let on_click = move |_| {
        spawn_local(async move {
            if Nip07Signer::is_available() {
                let events = nostr_sign_event(pub_key.get()).await;
                let key = events.pubkey.to_string();
                match check_npub(key.clone().to_owned()).await {
                    Ok(user) => {
                        match user.user {
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
                        }
                    }
                    Err(_e) => {
                        let window = web_sys::window().unwrap();
                        window
                            .alert_with_message(
                                "Something went wrong :( Please Refresh and Try again",
                            )
                            .unwrap();
                        let _ = window.location().reload();
                    },
                }
            } else {
                let window = web_sys::window().unwrap();
                window
                    .alert_with_message(
                        "Nostr Extensions Not Found",
                    )
                    .unwrap();

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
            </AnimatedShow>
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
fn Blank() -> impl IntoView {
    view! { <p></p> }
}
