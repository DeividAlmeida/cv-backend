mod controller;
mod service;
mod entity;

use actix_web::{error, post, get, web, App, Error, HttpServer, HttpResponse };
use controller::user_controller::UserController;
use actix_web::Responder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  HttpServer::new(|| {
    App::new()
      .service(create_user)
      .service(get_user)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

#[post("/")]
async fn create_user(body: web::Json<entity::user::Model>) -> impl Responder {
  UserController::create_user(body).await
}

#[get("/")]
async fn get_user() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}