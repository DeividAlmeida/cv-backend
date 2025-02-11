use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250210_190529_create_table_stories::Stories;
use crate::m20250210_185915_create_table_skills::Skills;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(StorySkills::Table)
          .if_not_exists()
          .col(pk_auto(StorySkills::Id))
          .col(integer(StorySkills::StroyId).not_null())
          .col(integer(StorySkills::SkillId).not_null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-story-skills-stroy_id")
              .from(StorySkills::Table, StorySkills::StroyId)
              .to(Stories::Table, Stories::Id)
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-story-skills-skill_id")
              .from(StorySkills::Table, StorySkills::SkillId)
              .to(Skills::Table, Skills::Id)
          )
          .to_owned(),
        )
        .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(StorySkills::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
pub enum StorySkills {
  Table,
  Id,
  StroyId,
  SkillId,
}
