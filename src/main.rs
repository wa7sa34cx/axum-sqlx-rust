use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, AddExtensionLayer,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
    
mod postgres;

#[derive(Serialize)]
struct Test {
    id: u64,
    text: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = postgres::connect().await.unwrap();

    let app = Router::new()
        .route("/", get(test_handler))
        .layer(AddExtensionLayer::new(pool));;

    let host = dotenv::var("HOST").expect("Couldn't read HOST from .env file!");

    println!("Listening on: {}", &host);

    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_handler(Extension(pool): Extension<PgPool>,) -> impl IntoResponse {
    // insert your application logic here
    let test = Test {
        id: 2,
        text: "abc".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(test))
}

// mod postgres;

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     dotenv::dotenv().ok();

//     let pool = postgres::connect().await.unwrap();

//     let sql = "INSERT INTO wa7_test_sqlx (text) VALUES ('test')".to_string();

//     sqlx::query(&sql).execute(&pool).await?;

//     Ok(())
// }
