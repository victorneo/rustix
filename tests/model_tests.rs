mod test_utils;

#[cfg(test)]
mod tests {
    use std::panic;
    use sqlx::Pool;
    use rstest::*;
    use rustix::models::user::User;
    use crate::test_utils::{f_pool};

    #[rstest]
    #[actix_rt::test]
    async fn test_user_get(#[future] f_pool: Pool<sqlx::Postgres>) {
        let pool = f_pool.await;
        let email = "email@email.com";
        let password = "123456";

        let id = sqlx::query!(
                "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id",
                &email,
                &password)
            .fetch_one(&pool)
            .await
            .expect("Did not insert test user").id;
        
        let user = User::get(id, &pool).await.unwrap();

        let test_result = panic::catch_unwind(|| {
            assert_eq!(id, user.id);
        });
        
        // Perform cleanup before raising errors
        sqlx::query!("DELETE FROM users WHERE id = $1", id).execute(&pool).await.expect("Did not delete user");

        // Raise assertion error for tests
        assert!(test_result.is_ok());
    }

    #[rstest]
    #[actix_rt::test]
    async fn test_user_add(#[future] f_pool: Pool<sqlx::Postgres>) {
        let pool = f_pool.await;
        let email = String::from("user1");
        let password = String::from("1234");
        let user = User::add(&email, &password, &pool).await;

        let test_result = panic::catch_unwind(|| {
            assert_eq!(email, user.email);
            assert_ne!(password, user.password);
        });
        
        // Perform cleanup before raising errors
        sqlx::query!("DELETE FROM users WHERE id = $1", user.id).execute(&pool).await.expect("Did not delete user");

        // Raise assertion error for tests
        assert!(test_result.is_ok());
    }

    #[rstest]
    #[actix_rt::test]
    async fn test_user_delete(#[future] f_pool: Pool<sqlx::Postgres>) {
        let pool = f_pool.await;
        let email = String::from("user1");
        let password = String::from("1234");

        let id = sqlx::query!(
                "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id",
                &email,
                &password)
            .fetch_one(&pool)
            .await
            .expect("Did not insert test user").id;
        
        let deleted = User::delete(id, &pool).await;

        let test_result = panic::catch_unwind(|| {
            assert!(deleted);
        });
        if !test_result.is_ok() {
            // Perform cleanup before raising errors
            sqlx::query!("DELETE FROM users WHERE id = $1", id).execute(&pool).await.expect("Did not delete user");
            assert!(test_result.is_ok());
        }
    }
}

