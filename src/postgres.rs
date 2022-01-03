//! Provides the connection pool for asynchronous PostgreSQL connections.

use sqlx::{postgres::PgPool, Error};

/// Creates a new connection pool
pub async fn connect() -> Result<PgPool, Error> {
    let uri = dotenv::var("DATABASE_URL").unwrap();

    // Create a new connection pool with a default pool configuration
    //
    // max_connections: 10
    // min_connections: 0
    // connect_timeout: 30sec
    PgPool::connect(&uri).await
}
