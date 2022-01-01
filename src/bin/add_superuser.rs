use std::env;
use sqlx::PgPool;
use rustix::models::user::User;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Username needed. Rerun with ./add_superuser <username>");
    }
    else {
        let username = String::from(args.get(1).unwrap());

        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::connect(&db_url).await.expect("Could not create DB connection pool");

        User::add_superuser(&username, &String::from("newpassword"), &pool).await;
        println!("Added superuser {}", username);
    }
}