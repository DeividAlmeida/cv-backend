use crate::entity::user;
use sea_orm::*;
use std::env;
use dotenv::dotenv;
use actix_web::{web};
pub struct UserService;

impl UserService {
  pub async fn create_user( body: web::Json<user::Model>) -> Result<user::ActiveModel, DbErr> {
    let database_url = "sqlite://C:/Users/Deivid/Desktop/back-cv/database.db".to_string();
    let db = Database::connect(&database_url).await.unwrap();
    dbg!(&body);
      user::ActiveModel {       
        name: Set(body.name.to_string()),
        age: Set(body.age),
        email: Set(body.email.to_string()),
        summery: Set(body.summery.to_string()),
        ..Default::default()
      }
      .save(&db)
      .await
  }
}