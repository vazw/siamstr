use actix_web::{get, web, HttpResponse, Responder};
use futures::join;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Pool, Sqlite};
use std::collections::HashMap;
use wasm_bindgen::UnwrapThrowExt;

// TODO make user choose relay of your choice
lazy_static! {
    static ref RELAYS: Vec<String> = vec![
        "wss://relay.siamstr.com".to_string(),
        "wss://wot.siamstr.com".to_string(),
        "wss://relay.notoshi.win".to_string(),
        "wss://nos.lol".to_string(),
        "wss://relay.damus.io".to_string(),
        "wss://relay.nostr.band".to_string(),
    ];
    static ref SIGN_RELAYS: Vec<String> = vec!["wss://sign.siamstr.com".to_string()];
}

#[derive(Debug, Deserialize)]
pub struct Name {
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UsersData {
    pub id: String,
    pub name: String,
    pub pubkey: String,
    pub lightning_url: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NostrUser {
    pub names: HashMap<String, String>,
    pub relays: HashMap<String, Vec<String>>,
    pub nip46: HashMap<String, Vec<String>>,
}

impl From<UsersData> for NostrUser {
    fn from(user: UsersData) -> NostrUser {
        let mut names: HashMap<String, String> = HashMap::new();
        names.insert(user.name.clone(), user.pubkey.clone());
        let mut relays: HashMap<String, Vec<String>> = HashMap::new();
        relays.insert(user.pubkey.clone(), RELAYS.clone());
        let mut nip_46: HashMap<String, Vec<String>> = HashMap::new();
        nip_46.insert(user.pubkey.clone(), SIGN_RELAYS.clone());
        NostrUser {
            names,
            relays,
            nip46: nip_46,
        }
    }
}

pub async fn get_username(db: web::Data<Pool<Sqlite>>, name: &str) -> Option<UsersData> {
    let l_name = name.to_lowercase();
    let query = format!("SELECT * FROM users WHERE name='{l_name}'");
    match sqlx::query_as::<_, UsersData>(&query)
        .fetch_one(&**db.clone())
        .await
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

#[get("/nostr.json")]
pub async fn verify(db: web::Data<Pool<Sqlite>>, payload: web::Query<Name>) -> impl Responder {
    match get_username(db, &payload.name).await {
        Some(user) => {
            let respon: NostrUser = NostrUser::from(user);
            HttpResponse::Ok().json(respon)
        }
        None => HttpResponse::NotFound()
            .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap_throw()),
    }
}

async fn make_error_response() -> HttpResponse {
    HttpResponse::NotFound().json(
        serde_json::from_str::<Value>("{\"status\":404,\"message\":\"Error\"}").unwrap_throw(),
    )
}

#[get("/lnurlp/{name}")]
pub async fn lnurl(db: web::Data<Pool<Sqlite>>, payload: web::Path<String>) -> impl Responder {
    let (user_info, error_response) = join!(get_username(db, &payload), make_error_response());
    if let Some(user) = user_info {
        if user.lightning_url.is_empty() {
            return error_response;
        };
        let user_domain: Vec<&str> = user.lightning_url.split('@').collect();
        if user_domain.len() > 1 {
            if let Ok(json_respon) = reqwest::get(format!(
                "https://{}/.well-known/lnurlp/{}",
                user_domain[1], user_domain[0]
            ))
            .await
            {
                match json_respon.json::<Value>().await {
                    Ok(lnurl_json) => HttpResponse::Ok().json(lnurl_json),
                    Err(_) => error_response,
                }
            } else {
                error_response
            }
        } else {
            error_response
        }
    } else {
        error_response
    }
}
