use sqlx::PgPool;
use actix_web::{HttpRequest, web};
use super::responses::{GreetResponse, TotalUsersResponse};

pub async fn greet(_req: HttpRequest) -> web::Json<GreetResponse> {
    web::Json(GreetResponse {
      msg: String::from("Hello World"),  
    })
}

pub async fn get_total_users(data: web::Data<PgPool>, _req: HttpRequest) -> web::Json<TotalUsersResponse> {
    let result = sqlx::query!("select count(*) as count from users")
        .fetch_one(data.get_ref())
        .await
        .expect("No sql results")
        .count.unwrap();

    web::Json(TotalUsersResponse {
        count: result,
    })
}
