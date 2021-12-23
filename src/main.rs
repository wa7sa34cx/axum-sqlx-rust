mod postgres;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let pool = postgres::connect().await.unwrap();

    

    Ok(())
}
