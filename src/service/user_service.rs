use crate::entity::user;
use crate::repository::Queries;
use sea_orm::{sea_query::{Alias, SimpleExpr}, DbErr, ExecResult, Iden, Iterable};
use actix_web::web;

pub(crate) trait UserService {
  async fn create_user(body: web::Json<user::Model>) -> Result<ExecResult, DbErr>;
  fn set_columns() -> Vec<Alias>;
}

impl UserService for Queries {

  fn set_columns() -> Vec<Alias> {
    user::Column::iter()
      .filter(|col| col.to_string() != "id".to_string())
      .map(|col| Alias::new(col.to_string().as_str()))
      .collect()
  }

  async fn create_user(body: web::Json<user::Model>) -> Result<ExecResult, DbErr> {
    let body = body.into_inner();
    let values: Vec<SimpleExpr> = vec![
      body.name.into(),
      body.age.into(),
      body.email.into(),
      body.summery.into()
    ];
    let columns = Self::set_columns();

    Queries::create(user::Entity, columns, values).await
  }
}