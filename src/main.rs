use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, AddExtensionLayer,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use std::sync::Arc;
use std::ops::Deref;

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

// #[derive(Clone)]
// pub struct State {
//     pub text: String,
// }

pub struct State(Arc<SharedState>);

impl Clone for State {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

// impl<T: ?Sized> AsRef<T> for State<T> {
//     fn as_ref(&self) -> &T {
//         &**self
//     }
// }

impl Deref for State {
    type Target = SharedState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SharedState {
    pub text: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = postgres::connect().await.unwrap();

    // Run mmigrations
    sqlx::migrate!().run(&pool).await.unwrap();

    // let state = Arc::new(State {
    //     text: "Hello".to_string(),
    // });
    let state = State(Arc::new(SharedState { text: "Hello".to_string() }));

    let app = Router::new()
        .route("/one", get(test_handler_one))
        .route("/two", get(test_handler_two))
        .route("/three", get(test_handler_three))
        .route("/four", get(test_handler_four))
        .route("/five", get(test_handler_five))
        .layer(AddExtensionLayer::new(pool))
        .layer(AddExtensionLayer::new(state));

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

    (StatusCode::OK, Json("four"))
}

async fn test_handler_five(
    Extension(state): Extension<State>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let sql = "INSERT INTO wa7_test (text) VALUES ($1::TEXT)";

    let _ = sqlx::query(&sql)
        .bind(&state.text)
        .execute(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json("five"))
}
