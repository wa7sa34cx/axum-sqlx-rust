mod postgres;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let pool = postgres::connect().await.unwrap();

    let sql = "INSERT INTO wa7_test_sqlx (text) VALUES ('test')".to_string();

    sqlx::query(&sql).execute(&pool).await?;

    Ok(())
}
