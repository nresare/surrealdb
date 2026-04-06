use revision::revisioned;
use surrealdb_types::{SqlFormat, ToSql};

use crate::catalog::NamespaceId;
use crate::expr::ChangeFeed;
use crate::expr::statements::info::InfoStructure;
use crate::kvs::impl_kv_value_revisioned;
use crate::sql::statements::define::DefineDatabaseStatement;
use crate::sql::{Expr, Idiom, Literal};
use crate::val::Value;
pub use surrealdb_schema::DatabaseId;

impl_kv_value_revisioned!(DatabaseId);

#[revisioned(revision = 1)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DatabaseDefinition {
	pub(crate) namespace_id: NamespaceId,
	pub(crate) database_id: DatabaseId,
	pub(crate) name: String,
	pub(crate) comment: Option<String>,
	pub(crate) changefeed: Option<ChangeFeed>,
	pub(crate) strict: bool,
}
impl_kv_value_revisioned!(DatabaseDefinition);

impl DatabaseDefinition {
	fn to_sql_definition(&self) -> DefineDatabaseStatement {
		DefineDatabaseStatement {
			name: Expr::Idiom(Idiom::field(self.name.clone())),
			comment: self
				.comment
				.clone()
				.map(|v| Expr::Literal(Literal::String(v)))
				.unwrap_or(Expr::Literal(Literal::None)),
			changefeed: self.changefeed.map(|v| v.into()),
			..Default::default()
		}
	}
}

impl ToSql for DatabaseDefinition {
	fn fmt_sql(&self, f: &mut String, fmt: SqlFormat) {
		self.to_sql_definition().fmt_sql(f, fmt)
	}
}

impl InfoStructure for DatabaseDefinition {
	fn structure(self) -> Value {
		Value::from(map! {
			"name".to_string() => self.name.into(),
			"comment".to_string(), if let Some(v) = self.comment => v.into(),
			"id".to_string() => self.database_id.0.into(),
		})
	}
}
