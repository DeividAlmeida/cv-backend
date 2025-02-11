pub use sea_orm_migration::prelude::*;

mod m20250210_185802_create_table_users;
mod m20250210_185915_create_table_skills;
mod m20250210_190529_create_table_stories;
mod m20250210_192643_create_table_story_skills;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
            Box::new(m20250210_185802_create_table_users::Migration),
            Box::new(m20250210_185915_create_table_skills::Migration),
            Box::new(m20250210_190529_create_table_stories::Migration),
            Box::new(m20250210_192643_create_table_story_skills::Migration),
        ]
	}
}
