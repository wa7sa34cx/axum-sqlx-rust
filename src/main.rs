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

pub struct InsertTest {
    pub id: i32,
    pub text: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = postgres::connect().await.unwrap();

    let app = Router::new()
        .route("/one", get(test_handler_one))
        .route("/two", get(test_handler_two))
        .route("/three", get(test_handler_three))
        .route("/four", get(test_handler_four))
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

async fn test_handler_three(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let insert = InsertTest { id: 5, text: None };

    let sql = "INSERT INTO wa7_test (text) VALUES ($1::TEXT)";

    let _ = sqlx::query(&sql)
        .bind(&insert.text.unwrap_or("".to_string()))
        .execute(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json("test"))
}

async fn test_handler_four(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let insert = InsertTest { id: 5, text: None };

    let sql = "INSERT INTO wa7_test_null (text) VALUES ($1::TEXT)";

    let _ = sqlx::query(&sql)
        .bind(&insert.text)
        .execute(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json("test"))
}
