use dotenv;
use rstest::*;
use sqlx::{PgPool, Pool};
use std::env;

#[fixture]
pub async fn f_pool() -> Pool<sqlx::Postgres> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL is not set in .env file");
    let f_pool = PgPool::connect(&db_url).await.expect("Could not create DB connection pool");

    sqlx::migrate!("./migrations")
        .run(&f_pool)
        .await
        .expect("Could not run migrations for testing");

    return f_pool;
}