use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub birth_place: String,
    pub birth_date: String,
    pub gender: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub user_id: i32,
    pub fullname: String,
    pub email: String,
    pub exp: u64,
}

#[derive(Serialize)]
pub struct ResponseAuthorization {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseSignup {
    pub status: i32,
    pub message: String,
}