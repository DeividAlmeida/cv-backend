use sea_orm::sea_query::{Alias, IntoTableRef, SimpleExpr};
use sea_query::Query;
use sea_orm::*;
use crate::db::DatabaseConnector;

pub struct Queries;

impl Queries {
	pub async fn create<T, V>(tbl_ref: T, columns: Vec<Alias>, values: V) -> Result<ExecResult, DbErr>  where T: IntoTableRef, V: IntoIterator<Item = SimpleExpr>, {
		let db = DatabaseConnector::connect().await?;

		let mut stmt = Query::insert();
		stmt.into_table(tbl_ref).columns(columns);
		stmt.values_panic(values);
		let builder = db.get_database_backend();
		match db.execute(builder.build(&stmt)).await {
			Ok(res) => {
				Ok(res)
			},
			Err(err) => {
				Err(err)
			},
		}
	}
}