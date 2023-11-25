use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
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

