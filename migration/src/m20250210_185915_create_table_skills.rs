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
          .table(Skills::Table)
          .if_not_exists()
          .col(pk_auto(Skills::Id))
          .col(integer(Skills::UserId))
          .col(string(Skills::Name))
          .col(string(Skills::Experience))
          .foreign_key(
            ForeignKey::create()
              .name("fk-skills-user_id")
              .from(Skills::Table, Skills::UserId)
              .to(Users::Table, Users::Id)
        )
        .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Skills::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
pub enum Skills {
  Table,
  Id,
  UserId,
  Name,
  Experience,
}
