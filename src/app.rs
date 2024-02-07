pub mod components;
pub mod core_api;
pub mod nostr;
use crate::app::core_api::api::count_users;
use components::donate::*;
use components::footer::*;
use components::nav::*;
use components::nofusers::*;
use components::notfound::*;
use components::signin::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/siamstr.css"/>

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

#[component]
fn HomePage() -> impl IntoView {
    let dark_mode = create_rw_signal(false);
    let count = create_rw_signal(0);
    let users_count = create_local_resource(move || count.get(), count_users);
    create_effect(move |_| match users_count.clone().get() {
        Some(respon) => match respon {
            Ok(user) => count.set(user.count),
            Err(_e) => count.set(0),
        },
        _ => count.set(0),
    });
    create_effect(move |_| {
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
        if stored_mode {
            let _ = web_dark_mode.set_value("dark");
            dark_mode.set(true);
        } else {
            let _ = web_dark_mode.remove_1("dark");
            dark_mode.set(false);
        };
    });
    view! {
        <NavBar/>
        <div class="bg-white dark:bg-zinc-950 max-w-full max-h-full min-w-screen min-h-screen bg-cover grid grid-cols-1 justify-items-center py-20 sm:py-20 md:py-20 lg:py-20">
            <SignInPage/>
            <UsersCount count=count/>
            <Donate/>
        </div>
        <Footer dark_mode=dark_mode/>
    }
}
