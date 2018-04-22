use actix::*;
use actix_web::*;
use chrono::NaiveDateTime;

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct GithubConnection {
    pub id: i32,
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub token: String,
}

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct StoredApp {
    pub uuid: String,
    pub base60url: String,
    pub created_at: NaiveDateTime,
    pub base_app_uuid: Option<String>,
    pub base_app_base60url: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct NewStoredApp<'a> {
    pub uuid: String,
    pub base_app_uuid: &'a str,
    pub created_at: NaiveDateTime,
}
