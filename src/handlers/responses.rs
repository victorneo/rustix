use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GreetResponse {
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct TotalUsersResponse {
    pub count: i64,
}