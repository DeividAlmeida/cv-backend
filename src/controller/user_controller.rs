use crate::service::user_service::{UserService};
use actix_web::{web, HttpResponse, Responder};
use crate::entity::user;

pub struct UserController;

impl UserController {
  pub async fn create_user(user: web::Json<user::Model>) -> impl Responder {
    match UserService::create_user(user).await {
      Ok(user) => HttpResponse::Ok(),
      Err(err) => {
        dbg!(err);
        HttpResponse::InternalServerError()
        },
    }
  }
}