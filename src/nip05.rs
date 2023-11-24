use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{FromRow, Pool, Sqlite};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Name {
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
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
}

pub async fn get_username(
    db: web::Data<Pool<Sqlite>>,
    name: &str,
) -> std::io::Result<Option<UsersData>> {
    // let result =
    //     sqlx::query("INSERT INTO users (id,name,pubkey,lightning_url,created) VALUES (?,?,?,?,?)")
    //         .bind("test_id")
    //         .bind("vaz")
    //         .bind("58f5a23008ba5a8730a435f68f18da0b10ce538c6e2aa5a1b7812076304d59f7")
    //         .bind("vazw@getalby.com")
    //         .bind("test")
    //         .execute(&**db)
    //         .await
    //         .unwrap();
    // println!("{:#?}", result);

    let query = format!("SELECT * FROM users WHERE name='{name}'");
    let exute: UsersData = sqlx::query_as::<_, UsersData>(&query)
        .fetch_one(&**db)
        .await
        .unwrap();
    Ok(Some(exute))
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        // at vercel link here as iframe
        "
            <head>
  <title>siamstr</title>
             <link rel=\"stylesheet\" href=\"assets/style.css\">
</head>
<iframe width=\"100%\" height=\"100%\" src=\"https://siamnostr-vazw.vercel.app\" title=\"siamstr.com\" style=\"border:none;\"></iframe>
",
    )
}

#[get("/nostr.json")]
pub async fn verify(db: web::Data<Pool<Sqlite>>, payload: web::Query<Name>) -> impl Responder {
    let user = get_username(db, &payload.name).await.unwrap();
    match user {
        Some(user) => {
            let user_respon = format!("{{\"{}\":\"{}\"}}", user.name, user.pubkey);
            let respon: NostrUser = NostrUser {
                names: serde_json::from_str(&user_respon).unwrap(),
            };
            HttpResponse::Ok().json(respon)
        }
        None => HttpResponse::NotFound()
            .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap()),
    }
}
// {"names":{"vazw":"58f5a23008ba5a8730a435f68f18da0b10ce538c6e2aa5a1b7812076304d59f7"}}

#[get("/lnurlp/{name}")]
pub async fn lnurl(db: web::Data<Pool<Sqlite>>, payload: web::Path<String>) -> impl Responder {
    let user = get_username(db, &payload).await.unwrap();
    match user {
        Some(user) => {
            if user.lightning_url.is_empty() {
                return HttpResponse::NotFound()
                    .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap());
            };
            let user_domain: Vec<&str> = user.lightning_url.split("@").collect();
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
                    return HttpResponse::Ok().json(expr);
                }
                Err(expr) => {
                    println!("{:#?}", expr);
                    return HttpResponse::NotFound().json(
                        serde_json::from_str::<Value>("{{\"status\":400,\"message\":\"Error\"}")
                            .unwrap(),
                    );
                }
            }
        }
        None => HttpResponse::NotFound()
            .json(serde_json::from_str::<Value>("{\"status\":404}").unwrap()),
    }
}




