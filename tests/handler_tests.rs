mod test_utils;

#[cfg(test)]
mod tests {
    use sqlx::Pool;
    use rstest::*;
    use actix_web::{web, test, App};
    use hanayoku::handlers::user_handlers::{greet, get_total_users};
    use hanayoku::handlers::responses::{GreetResponse, TotalUsersResponse};
    use crate::test_utils::{f_pool};

    #[rstest]
    #[actix_rt::test]
    async fn test_index_get() {
        let uri = "/";
        let mut app = test::init_service(
            App::new()
                .route(uri, web::get().to(greet))
        ).await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();

        let result: GreetResponse = test::call_and_read_body_json(&mut app, req).await;
        assert_eq!("Hello World", result.msg);
    }

    #[rstest]
    #[actix_rt::test]
    async fn test_get_total_users(#[future] f_pool: Pool<sqlx::Postgres>) {
        let uri = "/users";
        let pool = f_pool.await;
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route(uri, web::get().to(get_total_users))
        ).await;

        let req = test::TestRequest::get()
            .uri("/users")
            .to_request();

        let result: TotalUsersResponse = test::call_and_read_body_json(&mut app, req).await;
        assert_eq!(0, result.count);
    }
}
