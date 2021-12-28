use sqlx::{PgPool, Error};
use serde::{Serialize, Deserialize};
use bcrypt::{DEFAULT_COST, hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub active: Option<bool>,
}

impl User {
    pub async fn get(uid: i32, pool: &PgPool) -> Result<Self, Error> {
        Ok(sqlx::query_as!(User, "
            SELECT
                *
            FROM users WHERE id = $1
        ", uid).fetch_one(pool).await?)
    }

    pub async fn add(email: &String, password: &String, pool: &PgPool) -> User {
        let hashed = hash(password, DEFAULT_COST).expect("Could not hash password");
        sqlx::query_as!(
                User,
                "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
                email,
                hashed,
            )
            .fetch_one(pool)
            .await.expect("Could not add user")
    }

    pub async fn update(user: &User, pool: &PgPool) -> User {
        sqlx::query_as!(
                    User,
                    "UPDATE users SET email = $1, first_name = $2, last_name = $3, active = $4
                    WHERE id = $5
                    RETURNING *
                ",
                user.email,
                user.first_name,
                user.last_name,
                user.active,
                user.id
            )
            .fetch_one(pool)
            .await.expect("Could not update user")
    }

    pub async fn delete(uid: i32, pool: &PgPool) -> bool {
        sqlx::query!("
                DELETE FROM users WHERE id = $1
            ", uid)
        .execute(pool)
        .await
        .expect("Could not delete user")
        .rows_affected() == 1
    }
}
