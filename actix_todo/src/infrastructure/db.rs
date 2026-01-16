// use sqlx::mysql::MySqlPoolOptions;
// use sqlx::MySqlPool;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL.")
}

pub async fn run_migrations(pool: &Pool<Postgres>) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations.");
}
