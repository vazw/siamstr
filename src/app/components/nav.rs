use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let onclick_menu = move |_| {
        let menu = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("mobile-menu")
            .expect("element found");
        let _ = menu.class_list().toggle("hidden");
    };
    view! {
		<nav class="bg-white dark:bg-gray-900 w-full top-0 start-0 border-b border-violet-500 dark:border-gray-600 scroll-pt-1 p-1">
			<div class="flex items-center justify-between">
				<div class="text-white font-bold text-xl pl-5">
					<a
						href="/"
						target="_self"
						class="flex items-center space-x-3 rtl:space-x-reverse"
					>
						<span class="text-2xl font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 lg:text-5xl md:text-4xl sm:text-3xl">
							"siamstr"
							<small class="text-xl font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 pt-10 lg:text-2xl md:text-xl sm:text-xs pl-1 sm:pl-1 md:pl-5">
								"FREE NOSTR ADDRESSES"
							</small>
						</span>
					</a>
				</div>
				<div class="hidden md:block pr-5">
					<ul class="flex items-center space-x-8">
						<li>
							<a
								href="https://nostree.me/npub1vaz88a5zhsqsrj220vh5vdnpjsu53msm34hzvcrh27x5d7zeav7qm45t60/vaz"
								target="_blank"
							>
								<small class="text-xl font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 pt-10 md:text-xl sm:text-xs pl-1 sm:pl-1 md:pl-5">
									"About Me"
								</small>
							</a>
						</li>
						<li>
							<a href="https://github.com/vazw/siamstr" target="_blank">
								<small class="text-xs font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 pt-10 md:text-xl sm:text-xs pl-1 sm:pl-1 md:pl-5">
									"Github"
								</small>

							</a>
						</li>
					</ul>
				</div>
				<div class="md:hidden">
					<button
						class="outline-none mobile-menu-button"
						on:click={onclick_menu}
					>
						<svg
							class="w-6 h-6 text-black dark:text-white fill-orange-600"
							x-show="!showMenu"
							fill="none"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path d="M4 6h16M4 12h16M4 18h16"></path>
						</svg>
					</button>
				</div>
			</div>
			<div class="mobile-menu hidden md:hidden" id="mobile-menu">
				<ul class="mt-4 space-y-4">
					<li>
						<a
							href="https://nostree.me/npub1vaz88a5zhsqsrj220vh5vdnpjsu53msm34hzvcrh27x5d7zeav7qm45t60/vaz"
							target="_blank"
							class="block px-4 py-2 text-white bg-white dark:bg-gray-900 rounded"
						>
							<small class="text-xs font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 pt-10 md:text-l sm:text-xs pl-1 sm:pl-1 md:pl-5">
								"About me"
							</small>
						</a>
					</li>
					<li>
						<a
							href="https://github.com/vazw/siamstr"
							target="_blank"
							class="block px-4 py-2 text-white  bg-white dark:bg-gray-900 rounded"
						>
							<small class="text-xs font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-violet-500 to-pink-600 md:text-l sm:text-xs pl-1 sm:pl-1 md:pl-5">
								"Github   "
							</small>
						</a>
					</li>
				</ul>
			</div>
		</nav>
	}
}
