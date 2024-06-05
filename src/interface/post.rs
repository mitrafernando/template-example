use template_example::*;
use models::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct ResponsePost {
    pub status: i32,
    pub data: Option<Post>,
}

#[derive(Validate, Deserialize)]
pub struct CreatePost {
    #[validate(length(min = 1, max=100))]
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct PostId {
    pub id: i32,
}

#[derive(Serialize)]
pub struct ResponseDeletePost {
    pub status: i32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct DeletePost {
    pub title: String,
}
