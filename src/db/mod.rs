use std::time::Duration;
use log::LevelFilter;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub struct DatabaseConnector ;

impl DatabaseConnector {
  pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .acquire_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .max_lifetime(Duration::from_secs(8))
      .sqlx_logging(true)
      .sqlx_logging_level(LevelFilter::Info);

    match Database::connect(opt).await {
      Ok(conn) => Ok(conn),
      Err(err) => Err(err),
    }
  }
}