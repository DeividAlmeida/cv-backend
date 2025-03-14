use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
			manager
				.create_table(
					Table::create()
						.table(Users::Table)
						.if_not_exists()
						.col(pk_auto(Users::Id))
						.col(string(Users::Name).not_null())
						.col(integer(Users::Age).not_null())                    
						.col(string(Users::Email).not_null().unique_key())
						.col(text(Users::Summery).not_null())
						.to_owned(),
				)
				.await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
			manager
				.drop_table(Table::drop().table(Users::Table).to_owned())
				.await
    }
}

#[derive(DeriveIden)]
pub enum Users {
	Table,
	Id,
	Name,
	Age,
	Email,
	Summery,
}