mod test_utils;

#[cfg(test)]
mod tests {
    use std::panic;
    use rustix::models::user::User;
    use super::test_utils::{get_pool, get_test_user};

    #[actix_rt::test]
    async fn test_user_get() {
        let pool = get_pool().await;
        let user = get_test_user("getuser@get.com", &pool).await;
        
        let got_user = User::get(user.id, &pool).await.unwrap();

        let test_result = panic::catch_unwind(|| {
            assert_eq!(user.id, got_user.id);
        });
        
        // Perform cleanup before raising errors
        sqlx::query!("DELETE FROM users WHERE id = $1", user.id).execute(&pool).await.expect("Did not delete user");

        // Raise assertion error for tests
        assert!(test_result.is_ok());
    }

    #[actix_rt::test]
    async fn test_user_add() {
        let pool = get_pool().await;
        let email = String::from("adduser@get.com");
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

    #[actix_rt::test]
    async fn test_user_update() {
        let pool = get_pool().await;
        let mut user = get_test_user("updateuser@get.com", &pool).await;
        
        user.first_name = Some(String::from("Firsty"));
        user.last_name = Some(String::from("Lasty"));
        user.active = Some(false);

        let updated_user = User::update(&user, &pool).await;
        
        let test_result = panic::catch_unwind(|| {
            assert_eq!(user.first_name, updated_user.first_name);
            assert_eq!(user.last_name, updated_user.last_name);
            assert_eq!(user.active, updated_user.active);
        });

        sqlx::query!("DELETE FROM users WHERE id = $1", user.id).execute(&pool).await.expect("Did not delete user");
        assert!(test_result.is_ok());
    }

    #[actix_rt::test]
    async fn test_user_delete() {
        let pool = get_pool().await;
        let user = get_test_user("deleteuser@get.com", &pool).await;
        
        let deleted = User::delete(user.id, &pool).await;

        let test_result = panic::catch_unwind(|| {
            assert!(deleted);
        });
        if !test_result.is_ok() {
            // Perform cleanup before raising errors
            sqlx::query!("DELETE FROM users WHERE id = $1", user.id).execute(&pool).await.expect("Did not delete user");
            assert!(test_result.is_ok());
        }
    }
}

