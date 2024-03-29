use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use chrono::Local;
#[cfg(feature = "ssr")]
use uuid::Uuid;
#[cfg(feature = "ssr")]
pub const DB_URL: &str = "sqlite://database.db";
#[cfg(feature = "ssr")]
use sqlx::{Connection, FromRow, SqliteConnection, Row};
#[cfg(feature = "ssr")]
pub async fn db() -> Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect(DB_URL).await?)
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

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersCount {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersDataResult {
    pub id: String,
    pub name: String,
    pub pubkey: String,
    pub lightning_url: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRespons {
    pub user: Option<UsersDataResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CountsRespon {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoolRespons {
    pub status: i8,
}

#[server]
pub async fn count_users(_count: i32) -> Result<CountsRespon, ServerFnError> {
    let mut con = db().await.unwrap();
    let query = format!("SELECT COUNT(*) FROM users");
    let result = sqlx::query(&query)
        .fetch_one(&mut con)
        .await;
    match result {
        Ok(user) => {
            let num :i32=user.get(0);
            Ok(CountsRespon { count: num })
        }
        Err(_) => Ok(CountsRespon { count: 0 }),
    }
}

#[server]
pub async fn check_npub(hex_npub: String) -> Result<UserRespons, ServerFnError> {
    let mut con = db().await.unwrap();
    let query = format!("SELECT * FROM users WHERE pubkey='{hex_npub}'");
    let result = sqlx::query_as::<_, UsersData>(&query)
        .fetch_one(&mut con)
        .await;
    match result {
        Ok(user) => Ok(UserRespons {
            user: Some(UsersDataResult {
                id: user.id,
                name: user.name,
                pubkey: user.pubkey,
                lightning_url: user.lightning_url,
                created: user.created,
            }),
        }),
        Err(_) => Ok(UserRespons { user: None }),
    }
}

#[server]
pub async fn check_username(username: String) -> Result<BoolRespons, ServerFnError> {
    let mut con = db().await.unwrap();
    if username.is_empty() {
        Ok(BoolRespons { status: 0 })
    } else {
        let query = format!("SELECT * FROM users WHERE name='{username}'");
        let result = sqlx::query_as::<_, UsersData>(&query)
            .fetch_one(&mut con)
            .await;
        match result {
            Ok(_user) => Ok(BoolRespons { status: 1 }),
            Err(_) => Ok(BoolRespons { status: 0 }),
        }
    }
}

#[server]
pub async fn add_user(
    username: String,
    pubkey: String,
    lnurl: String,
) -> Result<BoolRespons, ServerFnError> {
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
        Ok(_user) => Ok(BoolRespons { status: 1 }),
        Err(_) => Ok(BoolRespons { status: 0 }),
    }
}

#[server]
pub async fn edit_user(
    username: String,
    pubkey: String,
    lnurl: String,
) -> Result<BoolRespons, ServerFnError> {
    let mut con = db().await.unwrap();
    let query = format!(
        "UPDATE users SET name='{username}', lightning_url='{lnurl}' WHERE pubkey='{pubkey}'"
    );
    match sqlx::query(&query).execute(&mut con).await {
        Ok(_user) => Ok(BoolRespons { status: 1 }),
        Err(_) => Ok(BoolRespons { status: 0 }),
    }
}

#[server]
pub async fn delete_user(pubkey: String) -> Result<BoolRespons, ServerFnError> {
    let mut con = db().await.unwrap();
    match sqlx::query("DELETE FROM users WHERE pubkey=(?)")
        .bind(pubkey)
        .execute(&mut con)
        .await
    {
        Ok(_user) => Ok(BoolRespons { status: 1 }),
        Err(_) => Ok(BoolRespons { status: 0 }),
    }
}
