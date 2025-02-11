use crate::service::user_service::UserService;
use actix_web::{ web, HttpResponse, Responder };
use crate::entity::user;
pub struct UserController;

impl UserController {
  pub async fn create_user(user: web::Json<user::Model>) -> impl Responder {
    match UserService::create_user(user).await {
      Ok(_) => HttpResponse::Ok().body("User created successfully"),
      Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
  }
  pub async fn get_user(id:  web::Path<u32>) -> impl Responder {
    match UserService::get_user(id.into_inner()).await {
      Ok(user) => {
        HttpResponse::Ok()
          .content_type("application/json")
          .json(user)
      },
      Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
  }
}