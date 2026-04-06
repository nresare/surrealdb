use revision::revisioned;
use surrealdb_types::{SqlFormat, ToSql};

pub use surrealdb_schema::NamespaceId;

use crate::expr::statements::info::InfoStructure;
use crate::kvs::impl_kv_value_revisioned;
use crate::sql::statements::DefineNamespaceStatement;
use crate::sql::{Expr, Literal};
use crate::val::Value;

impl_kv_value_revisioned!(NamespaceId);

#[revisioned(revision = 1)]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct NamespaceDefinition {
	pub namespace_id: NamespaceId,
	pub name: String,
	pub comment: Option<String>,
}
impl_kv_value_revisioned!(NamespaceDefinition);

impl NamespaceDefinition {
	fn to_sql_definition(&self) -> DefineNamespaceStatement {
		DefineNamespaceStatement {
			name: crate::sql::Expr::Idiom(crate::sql::Idiom::field(self.name.clone())),
			comment: self
				.comment
				.clone()
				.map(|v| Expr::Literal(Literal::String(v)))
				.unwrap_or(Expr::Literal(Literal::None)),
			..Default::default()
		}
	}
}

impl ToSql for NamespaceDefinition {
	fn fmt_sql(&self, f: &mut String, fmt: SqlFormat) {
		self.to_sql_definition().fmt_sql(f, fmt)
	}
}

impl InfoStructure for NamespaceDefinition {
	fn structure(self) -> Value {
		Value::from(map! {
			"name".to_string() => self.name.into(),
			"comment".to_string(), if let Some(v) = self.comment => v.into(),
			"id".to_string() => self.namespace_id.0.into(),
		})
	}
}
