use sqlx::{PgPool, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
}

impl User {
    pub async fn get(uid: i32, pool: &PgPool) -> Result<Self, Error> {
        Ok(sqlx::query_as_unchecked!(User, "
            SELECT
                *
            FROM users WHERE id = $1
        ", uid).fetch_one(pool).await?)
    }
}