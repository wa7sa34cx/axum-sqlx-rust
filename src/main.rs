use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, AddExtensionLayer,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};

mod postgres;

#[derive(FromRow, Deserialize, Serialize)]
struct Test {
    id: i32,
    text: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = postgres::connect().await.unwrap();

    let app = Router::new()
        .route("/one", get(test_handler_one))
        .route("/two", get(test_handler_two))
        .layer(AddExtensionLayer::new(pool));

    let host = dotenv::var("HOST").expect("Couldn't read HOST from .env file!");

    println!("Listening on: {}", &host);

    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_handler_one(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM wa7_test_sqlx where id=1".to_string();

    let test: Test = sqlx::query_as(&sql).fetch_one(&pool).await.unwrap();

    (StatusCode::OK, Json(test))
}

async fn test_handler_two(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM wa7_test_sqlx where id=2".to_string();

    let test: Test = sqlx::query_as(&sql).fetch_one(&pool).await.unwrap();

    (StatusCode::OK, Json(test))
}
