pub mod nip07;
use core::time::Duration;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use nip07::{get_public_key, sign_event};
use serde::Deserialize;
use serde_wasm_bindgen;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Siamstr"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

async fn nostr_pubkey(_key: String) -> String {
    let key = get_public_key().await;
    let key: String = serde_wasm_bindgen::from_value(key).unwrap();
    key
}

#[component]
fn HomePage() -> impl IntoView {
    let dark_mode = create_rw_signal(false);
    view! {
        <NavBar/>
        <div class="bg-white dark:bg-zinc-950 max-w-full max-h-full min-w-screen min-h-screen bg-cover grid grid-cols-1 justify-items-center py-20 sm:py-20 md:py-20 lg:py-20">
            <SignInPage/>
        </div>
        <Footer dark_mode=dark_mode/>
    }
}

/// Renders the home page of your application.
#[component]
fn SignInPage() -> impl IntoView {
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
            <AnimatedShow
                when=show_input
                show_class="fade-in-1000"
                hide_class="fade-out-1000"
                hide_delay=Duration::from_millis(1000)
            >
                <div>
                    <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 break-words sm:text-xs md:text-lg">
                        "สวัสดี!! " {pub_key}
                    </label>
                    <Transition fallback=move || {
                        view! { <div>"Loading..."</div> }
                    }>
                        <div>
                            <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-xs md:text-lg">
                                "ตั้งชื่อ"
                                <input
                                    type="text"
                                    class="text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
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
                            <label class="relative inline-flex items-center mb-5 cursor-pointer">
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

                                <div class="w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-orange-300 dark:peer-focus:ring-orange-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-orange-600"></div>
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
                                <label class="mt-6 text-sm leading-3 text-gray-900 dark:text-gray-300 sm:text-sm md:text-lg">
                                    "Enter your existing lightning address to enable redirection.
                                    You can then use your nostr address as your lightning address."
                                    <input
                                        type="text"
                                        class="text-gray-100 rounded-lg bg-gray-100 dark:bg-gray-900 border-purple-600 border-2 w-7/12"
                                        id="lnurl"
                                        prop:placeholder="vazw@getalby.com"
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev)
                                                .parse::<String>()
                                                .unwrap_or("".to_string());
                                            lnurl.set(val);
                                        }
                                    />

                                </label>
                            </AnimatedShow>
                            <br/>
                            <button class="btn btn--primary">"Register"</button>
                        </div>
                    </Transition>
                </div>
            </AnimatedShow>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-white dark:bg-gray-900 fixed w-full z-12 top-0 start-0 border-b border-violet-500 dark:border-gray-600 scroll-pt-1">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-1">
                <a href="#" class="flex items-center space-x-3 rtl:space-x-reverse">
                    <span class="text-2xl font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 md:text-5xl sm:text-3xl">
                        "siamstr"
                        <small class="text-xs font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 pt-10 md:text-xl sm:text-xs pl-1 sm:pl-1 md:pl-5">
                            "FREE NOSTR ADDRESSES. เพื่อทุกคน เพื่ออิสระภาพ"
                        </small>
                    </span>
                </a>
                <div class="flex flex-col space-y-2 flex md:order-2 space-x-3 md:space-x-1 rtl:space-x-reverse justify-items-center content-center items-center"></div>
            </div>
        </nav>
    }
}

#[component]
fn Footer(
    dark_mode: RwSignal<bool>
) -> impl IntoView {
    let dark_mode_sw = move |_| {
        let web_dark_mode = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap()
            .class_list();
        if dark_mode.get() {
            web_dark_mode.remove_1("dark");
            dark_mode.set(false);
        } else {
            web_dark_mode.set_value("dark");
            dark_mode.set(true);
        };
    };
    view! {
        <footer class="bg-white dark:bg-gray-900 fixed w-full z-3 bottom-0 end-0 border-t border-violet-500 dark:border-gray-600 max-h-10">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-1">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400 pb-10">
                    "© 2023 " <a href="https://siamstr.com/" class="hover:underline">
                        "Siamtr"
                    </a>
                </span>
                <div class="flex flex-col space-y-2 flex md:order-2 space-x-3 md:space-x-1 rtl:space-x-reverse justify-items-center content-center items-center pb-10">
                    <button
                        type="button"
                        class="text-orange-700 border border-orange-700 hover:bg-orange-700 hover:text-white focus:ring-4 focus:outline-none focus:ring-orange-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center dark:border-orange-500 dark:text-orange-500 dark:hover:text-white dark:focus:ring-orange-800 dark:hover:bg-orange-500"
                        on:click=dark_mode_sw
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 384 512">
                            <path d="M144.7 98.7c-21 34.1-33.1 74.3-33.1 117.3c0 98 62.8 181.4 150.4 211.7c-12.4 2.8-25.3 4.3-38.6 4.3C126.6 432 48 353.3 48 256c0-68.9 39.4-128.4 96.8-157.3zm62.1-66C91.1 41.2 0 137.9 0 256C0 379.7 100 480 223.5 480c47.8 0 92-15 128.4-40.6c1.9-1.3 3.7-2.7 5.5-4c4.8-3.6 9.4-7.4 13.9-11.4c2.7-2.4 5.3-4.8 7.9-7.3c5-4.9 6.3-12.5 3.1-18.7s-10.1-9.7-17-8.5c-3.7 .6-7.4 1.2-11.1 1.6c-5 .5-10.1 .9-15.3 1c-1.2 0-2.5 0-3.7 0c-.1 0-.2 0-.3 0c-96.8-.2-175.2-78.9-175.2-176c0-54.8 24.9-103.7 64.1-136c1-.9 2.1-1.7 3.2-2.6c4-3.2 8.2-6.2 12.5-9c3.1-2 6.3-4 9.6-5.8c6.1-3.5 9.2-10.5 7.7-17.3s-7.3-11.9-14.3-12.5c-3.6-.3-7.1-.5-10.7-.6c-2.7-.1-5.5-.1-8.2-.1c-3.3 0-6.5 .1-9.8 .2c-2.3 .1-4.6 .2-6.9 .4z"></path>
                        </svg>
                        <span class="sr-only">Icon description</span>
                    </button>
                </div>
            </div>
        </footer>
    }
}   



















