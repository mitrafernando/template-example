use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode, Uri,
    },
    middleware as AxumMiddleware,
    response::Json,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use handler::post::{delete_post, get_post, published_post, show_post, write_post};
use handler::auth::{login, signup};
use middleware::auth::auth;
use serde_json::{json, Value};
use std::env;
use tower_http::cors::{Any, CorsLayer};

pub mod constant;
pub mod handler;
pub mod helper;
pub mod interface;
pub mod middleware;
pub mod models;
pub mod repository;
pub mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = env::var("BE_URL").expect("BE_URL must be set");
    let app = Router::new()
        .route("/posts", get(show_post))
        .route("/post/:post_id", get(get_post))
        .route("/post/new", post(write_post))
        .route("/post/publish", post(published_post))
        .route("/post/delete", post(delete_post))
        .route_layer(AxumMiddleware::from_fn(auth))
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/plain_text", get(plain_text))
        .route("/json", get(json))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST])
                .allow_origin(Any)
                .allow_headers([CONTENT_TYPE, AUTHORIZATION]),
        )
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    println!("Running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

async fn plain_text() -> &'static str {
    "Foo"
}

async fn json() -> Json<Value> {
    Json(json!([{ "data": 42 }]))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
