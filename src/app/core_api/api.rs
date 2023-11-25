use leptos::*;
use chrono::{DateTime,Utc, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub const DB_URL: &str = "sqlite://database.db";
#[cfg(feature = "ssr")]
use sqlx::{Connection, SqliteConnection, FromRow};
#[cfg(feature = "ssr")]
pub async fn db() -> Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect(DB_URL).await?)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoolRespons {
    pub status: i8,
}

#[server]
pub async fn check_npub(hex_npub: String) -> Result<BoolRespons, ServerFnError> {
    let mut con = db().await.unwrap();
    let query = format!("SELECT * FROM users WHERE pubkey={hex_npub}");
    match sqlx::query_as::<_, UsersData>(&query).fetch_one(&mut con).await {
        Ok(_user) => Ok(BoolRespons{
            status:1
        }),
        Err(_) => Ok(BoolRespons{
            status:0
        }),
    }
}

#[server]
pub async fn check_username(username: String) -> Result<BoolRespons, ServerFnError> {
    let mut con = db().await.unwrap();
    let query = format!("SELECT * FROM users WHERE name={username}");
    match sqlx::query_as::<_,UsersData>(&query).fetch_one(&mut con).await {
        Ok(_user) => Ok(BoolRespons{
            status:1
        }),
        Err(_) => Ok(BoolRespons{
            status:0
        }),
    }
}

#[server]
pub async fn add_user(username: String, pubkey: String, lnurl: String) -> Result<(), ServerFnError> {
    let id = Uuid::new_v4().to_string();
    let time_now = Local::now().to_rfc3339();
    let mut con = db().await.unwrap();
    match sqlx::query("INSERT INTO users (id,name,pubkey,lightning_url,created) VALUES (?,?,?,?,?)")
            .bind(id)
            .bind(username)
            .bind(pubkey)
            .bind(lnurl)
            .bind(time_now)
            .execute(&mut con)
            .await
             {
        Ok(_user) => Ok(()),
        Err(_) => Ok(()),
    }
}


#[cfg(feature = "ssr")]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersData {
    pub id: String,
    pub name: String,
    pub pubkey: String,
    pub lightning_url: String,
    pub created: String,
}













