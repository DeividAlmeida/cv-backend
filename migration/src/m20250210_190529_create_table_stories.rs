use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250210_185802_create_table_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Stories::Table)
          .if_not_exists()
          .col(pk_auto(Stories::Id))
          .col(integer(Stories::UserId).not_null())
          .col(string(Stories::Position))
          .col(string(Stories::PositionLocation))
          .col(string(Stories::Period))
          .col(string(Stories::Company))
          .col(string(Stories::Summary))
          .foreign_key(
            ForeignKey::create()
              .name("fk-stories-user_id")
              .from(Stories::Table, Stories::UserId)
              .to(Users::Table, Users::Id)
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Stories::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
pub enum Stories {
  Table,
  Id,
  UserId,
  Position,
  PositionLocation,
  Period,
  Company,
  Summary,
}