#[cfg(feature = "ssr")]
pub mod api;
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::fs;
    use futures::join;
    use std::time::Duration;

    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::{migrate::MigrateDatabase, Sqlite};
    use sqlx::{Executor, Pool};
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use siamstr::app::*;
    use siamstr::app::core_api::api::DB_URL;
    use api::{verify, lnurl};

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


    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    pub struct Oid {
        #[serde(rename="$oid")]
        pub id: String
    }
    #[derive(Debug, Clone, Deserialize)]
    pub struct UsersData {
        #[serde(rename="_id")]
        pub id: Oid,
        pub username: String,
        pub pubkey: String,
        #[serde(rename="lightningAddress")]
        pub lightningaddress: String,
        #[serde(rename="registeredAt")]
        pub registeredat: String,
    }

    pub async fn import_json(db: Pool<Sqlite>) {
        if let Ok(json_file) = fs::File::open("users.json") {
            let json_data = std::io::BufReader::new(json_file);
            let json: Vec<UsersData> = serde_json::from_reader(json_data)
                .expect("file should be proper JSON");
            // let 
            let mut updated :i32 = 0;
            for user in json.iter() {
                match sqlx::query("INSERT INTO users (id,name,pubkey,lightning_url,created) VALUES (?,?,?,?,?)")
                        .bind(&user.id.id)
                        .bind(&user.username)
                        .bind(&user.pubkey)
                        .bind(&user.lightningaddress)
                        .bind(&user.registeredat)
                        .execute(&db)
                        .await
                         {
                    Ok(_result) => updated+=1,
                    Err(_) => println!("Failed to add : {:#?}", user.username),
                }
            println!("Updated User : {updated}")
            }
        } else {
            println!("Not Found json Back-UP file, skip back up process");
        }
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
        .connect_lazy(DB_URL).unwrap();
    join!(create_data_table(db.clone()), import_json(db.clone()));
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

