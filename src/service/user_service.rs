use crate::entity::user;
use crate::db::DatabaseConnector;
use sea_orm::*;
use actix_web::web;
pub struct UserService;

impl UserService {
  pub async fn create_user( body: web::Json<user::Model>) -> Result<user::ActiveModel, DbErr> {

    let db = DatabaseConnector::connect().await?;
    let user::Model { name, age, email, summery, .. } = body.into_inner();
    user::ActiveModel {       
      name: Set(name),
      age: Set(age),
      email: Set(email),
      summery: Set(summery),
      ..Default::default()
    }
    .save(&db)
    .await
  }

  pub async fn get_user(id: u32) -> Result<JsonValue, DbErr> {
    let db = DatabaseConnector::connect().await?;
    let id_i32: i32 = id.try_into().map_err(|_| DbErr::Custom("ID conversion error".into()))?;
    match user::Entity::find_by_id(id_i32).into_json().one(&db).await {
      Ok(Some(user)) => Ok(user),
      Ok(None) => Err(DbErr::RecordNotFound("User not found".into())),
      Err(err) => Err(err),
    }
  }
}