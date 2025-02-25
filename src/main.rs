mod controller;
mod service;
mod entity;
mod db;
mod repository;

use actix_web::{ post, get, web::{ Path, Json }, App, HttpServer, HttpResponse };
use controller::user_controller::UserController;
use actix_web::Responder;
use dotenv::var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let port = var("PORT").expect("PORT must be set").parse().unwrap();
  HttpServer::new(|| {
    App::new()
      .service(create_user)
      // .service(get_user)
  })
  .bind(("127.0.0.1", port))?
  .run()
  .await
}

#[post("/")]
async fn create_user(body: Json<entity::user::Model>) -> impl Responder {
  UserController::create_user(body).await
}

// #[get("/{id}")]
// async fn get_user(id: Path<u32>) -> impl Responder {
//   UserController::get_user(id).await
// }