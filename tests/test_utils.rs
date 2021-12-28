use sqlx::{PgPool, Pool};
use rustix::models::user::User;
use std::env;

#[cfg(test)]
pub async fn get_pool() -> Pool<sqlx::Postgres> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL is not set in .env file");
    let f_pool = PgPool::connect(&db_url).await.expect("Could not create DB connection pool");

    sqlx::migrate!("./migrations")
        .run(&f_pool)
        .await
        .expect("Could not run migrations for testing");

    f_pool
}

#[allow(dead_code)]
pub async fn get_test_user(email: &str, pool: &PgPool) -> User {
    let email = String::from(email);
    let password = String::from("1234");
    sqlx::query_as!(
            User,
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
            &email,
            &password)
        .fetch_one(pool)
        .await
        .expect("Did not insert test user")
}
