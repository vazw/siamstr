use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Pool, Sqlite};
use wasm_bindgen::UnwrapThrowExt;

const RELAYS: [&str; 3] = [
    "wss://relay.siamstr.com",
    "wss://relay.notoshi.win",
    "wss://bostr.lecturify.net",
];
const SIGN_RELAYS: [&str; 1] = ["wss://sign.siamstr.com"];

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
        fn str_to_string(s: &str) -> String {
            s.to_string()
        }
        let mut names: HashMap<String, String> = HashMap::new();
        names.insert(user.name.clone(), user.pubkey.clone());
        let mut relays: HashMap<String, Vec<String>> = HashMap::new();
        relays.insert(user.pubkey.clone(), RELAYS.map(str_to_string).to_vec());
        let mut nip_46: HashMap<String, Vec<String>> = HashMap::new();
        nip_46.insert(user.pubkey.clone(), SIGN_RELAYS.map(str_to_string).to_vec());
        NostrUser {
            names,
            relays,
            nip46: nip_46,
        }
    }
}

pub async fn get_username(db: web::Data<Pool<Sqlite>>, name: &str) -> Option<UsersData> {
    let query = format!("SELECT * FROM users WHERE name='{name}'");
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

#[get("/lnurlp/{name}")]
pub async fn lnurl(db: web::Data<Pool<Sqlite>>, payload: web::Path<String>) -> impl Responder {
    let error_response = HttpResponse::NotFound().json(
        serde_json::from_str::<Value>("{\"status\":404,\"message\":\"Error\"}").unwrap_throw(),
    );
    match get_username(db, &payload).await {
        Some(user) => {
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
        }
        None => error_response,
    }
}
