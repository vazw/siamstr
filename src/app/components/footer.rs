use leptos::*;

#[component]
pub fn Footer(dark_mode: RwSignal<bool>) -> impl IntoView {
    let dark_mode_sw = move |_| {
        let web_dark_mode = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap()
            .class_list();
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .expect("LocalStorage Not Found")
            .unwrap();
        let stored_mode = match storage.get_item("darkMode") {
            Ok(dark) => match dark {
                Some(expr) => expr.parse().unwrap_or(false),
                None => false,
            },
            Err(_e) => false,
        };
        if dark_mode.get() & stored_mode {
            let _ = web_dark_mode.remove_1("dark");
            let _ = storage.set_item("darkMode", "false");
            dark_mode.set(false);
        } else {
            web_dark_mode.set_value("dark");
            let _ = storage.set_item("darkMode", "true");
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
                        <span class="sr-only">Dark Mode</span>
                    </button>
                </div>
            </div>
        </footer>
    }
}

