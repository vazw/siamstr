#[cfg(feature = "ssr")]
pub mod api;
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use futures::join;
    use std::time::Duration;

    use actix_files::Files;
    use actix_web::*;
    use api::{lnurl, verify};
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use siamstr::app::core_api::api::DB_URL;
    use siamstr::app::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::{migrate::MigrateDatabase, Sqlite};
    use sqlx::{Executor, Pool};

    pub async fn create_data_table(db: Pool<Sqlite>) {
        let _con = db
            .clone()
            .acquire()
            .await
            .unwrap()
            .execute(
                r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                pubkey TEXT NOT NULL,
                lightning_url TEXT NOT NULL,
                created TEXT NOT NULL
            );"#,
            )
            .await;
    }

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(30))
        .connect_lazy(DB_URL)
        .unwrap();
    join!(create_data_table(db.clone()));
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            //serve Nostr api
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/.well-known").service(verify).service(lnurl))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use siamstr::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
