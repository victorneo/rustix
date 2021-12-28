mod handlers;
mod models;
mod state;

use std::env;
use actix_web::{web, App, HttpServer};
use handlers::user_handlers::{greet, get_total_users};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPool::connect(&db_url).await.expect("Could not create DB connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(greet))
            .route("/users", web::get().to(get_total_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
