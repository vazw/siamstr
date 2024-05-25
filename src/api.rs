use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Pool, Sqlite};

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

#[derive(Debug, Serialize)]
pub struct NostrUser {
    pub names: Value,
    pub relays: Value,
    pub nip46: Value,
}

pub async fn get_username(
    db: web::Data<Pool<Sqlite>>,
    name: &str,
) -> std::io::Result<Option<UsersData>> {
    let lowercase_name = name.to_lowercase();
    let query = format!("SELECT * FROM users WHERE name='{lowercase_name}'");
    match sqlx::query_as::<_, UsersData>(&query)
        .fetch_one(&**db)
        .await
    {
        Ok(user) => Ok(Some(user)),
        Err(_) => Ok(None),
    }
}

#[get("/nostr.json")]
pub async fn verify(db: web::Data<Pool<Sqlite>>, payload: web::Query<Name>) -> impl Responder {
    let user = get_username(db, &payload.name).await.unwrap();
    match user {
        Some(user) => {
            let user_respon = format!("{{\"{}\":\"{}\"}}", user.name, user.pubkey);
            let relay_respon = format!("{{\"{}\":{}}}", user.pubkey, "[\"wss://relay.siamstr.com\", \"wss://relay.notoshi.win\", \"wss://bostr.lecturify.net\"]");
            let nip46 = format!("{{\"{}\":{}}}", user.pubkey, "[\"wss://sign.siamstr.com\"]");
            let respon: NostrUser = NostrUser {
                names: serde_json::from_str(&user_respon).unwrap(),
                relays: serde_json::from_str(&relay_respon).unwrap(),
                nip46: serde_json::from_str(&nip46).unwrap(),
            };
            HttpResponse::Ok().json(respon)
        }
        None => HttpResponse::NotFound()
            .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap()),
    }
}

#[get("/lnurlp/{name}")]
pub async fn lnurl(db: web::Data<Pool<Sqlite>>, payload: web::Path<String>) -> impl Responder {
    let user = get_username(db, &payload).await.unwrap();
    match user {
        Some(user) => {
            if user.lightning_url.is_empty() {
                return HttpResponse::NotFound()
                    .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap());
            };
            let user_domain: Vec<&str> = user.lightning_url.split('@').collect();
            if user_domain.len() > 1 {
                let respon = reqwest::get(format!(
                    "https://{}/.well-known/lnurlp/{}",
                    user_domain[1], user_domain[0]
                ))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
                let json_respon = serde_json::from_str::<Value>(&respon);
                match json_respon {
                    Ok(expr) => {
                        HttpResponse::Ok().json(expr)
                    }
                    Err(expr) => {
                        println!("{:#?}", expr);
                        HttpResponse::NotFound().json(
                            serde_json::from_str::<Value>(
                                "{{\"status\":400,\"message\":\"Error\"}",
                            )
                            .unwrap(),
                        )
                    }
                }
            } else {
                HttpResponse::NotFound().json(
                    serde_json::from_str::<Value>("{{\"status\":400,\"message\":\"Error\"}")
                        .unwrap(),
                )
            }
        }
        None => HttpResponse::NotFound()
            .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap()),
    }
}
