use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type Db = Pool<Postgres>;

/// Setup DB connection
/// ref: https://github.com/skerkour/bloom-legacy/blob/v3/bloom/kernel/src/db.rs
pub async fn setup(url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(15)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(url)
        .await
}
