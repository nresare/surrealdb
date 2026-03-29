pub use surrealdb_sdk_core::dbs::{QueryResult, QueryResultBuilder, QueryType, Status};

use crate::expr::TopLevelExpr;

pub(crate) fn query_type_for_toplevel_expr(expr: &TopLevelExpr) -> QueryType {
	match expr {
		TopLevelExpr::Live(_) => QueryType::Live,
		TopLevelExpr::Kill(_) => QueryType::Kill,
		_ => QueryType::Other,
	}
}
