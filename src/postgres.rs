use sqlx::{Error, postgres::{PgConnectOptions, PgPool}};

pub async fn connect() -> Result<PgPool, Error> {
    // PgConnectOptions::new().connect().await?

    // let options = PgConnectOptions::new()
    // .host("secret-host")
    // .port(2525)
    // .username("secret-user")
    // .password("secret-password")
    // .ssl_mode(PgSslMode::Require);

    let options = PgConnectOptions::new();

    PgPool::connect_with(options).await

}
