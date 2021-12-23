//! Provides the connection pool for asynchronous PostgreSQL connections.
use sqlx::{
    postgres::{PgConnectOptions, PgPool},
    Error,
};

/// Creates a new connection pool
pub async fn connect() -> Result<PgPool, Error> {
    // By default, this reads the following environment variables
    // and sets their equivalent options.
    // See docs: https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#impl
    let options = PgConnectOptions::new();

    // Creates a new connection pool with a default pool configuration
    // and the given connection options; and, immediately establishes one connection.
    //
    // max_connections: 10
    // min_connections: 0
    // connect_timeout: 30sec
    PgPool::connect_with(options).await
}
