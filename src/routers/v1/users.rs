use actix_web::{post, web};
use crate::AppData;

#[post("/login")]
pub async fn login_user(data: web::Data<AppData>) -> String {
    data.database.ping().await.unwrap();
    String::from("hi")
}