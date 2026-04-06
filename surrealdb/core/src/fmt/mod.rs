//! SurrealQL formatting utilities.

#[cfg(test)]
mod test;

use surrealdb_support::fmt as support_fmt;
use surrealdb_types::{SqlFormat, ToSql};

use crate::sql;

pub use support_fmt::{
	EscapeKwFreeIdent, EscapeKwIdent, EscapeObjectKey, EscapeRidKey, Float, Fmt, QuoteStr,
	fmt_separated_by,
};

fn push_escaped_backtick_string(into: &mut String, value: &str) {
	for c in value.chars() {
		match c {
			'\0' => into.push_str("\\0"),
			'\r' => into.push_str("\\r"),
			'\t' => into.push_str("\\t"),
			'\n' => into.push_str("\\n"),
			'\x08' => into.push_str("\\u{8}"),
			'\x0C' => into.push_str("\\f"),
			'\\' => into.push_str("\\\\"),
			'`' => into.push_str("\\`"),
			_ => into.push(c),
		}
	}
}

/// Escapes identifiers which might be used in the same place as a keyword.
pub struct EscapeIdent<T>(pub T);

impl<T: AsRef<str>> ToSql for EscapeIdent<T> {
	fn fmt_sql(&self, f: &mut String, fmt: SqlFormat) {
		let s = self.0.as_ref();
		if crate::syn::could_be_reserved_keyword(s) {
			f.push('`');
			push_escaped_backtick_string(f, s);
			f.push('`');
		} else {
			EscapeKwFreeIdent(s).fmt_sql(f, fmt);
		}
	}
}

pub struct CoverStmts<'a>(pub &'a sql::Expr);

impl ToSql for CoverStmts<'_> {
	fn fmt_sql(&self, f: &mut String, fmt: SqlFormat) {
		match self.0 {
			sql::Expr::Literal(_)
			| sql::Expr::Param(_)
			| sql::Expr::Idiom(_)
			| sql::Expr::Table(_)
			| sql::Expr::Mock(_)
			| sql::Expr::Block(_)
			| sql::Expr::Constant(_)
			| sql::Expr::Prefix {
				..
			}
			| sql::Expr::Postfix {
				..
			}
			| sql::Expr::Binary {
				..
			}
			| sql::Expr::FunctionCall(_)
			| sql::Expr::Closure(_)
			| sql::Expr::Break
			| sql::Expr::Continue
			| sql::Expr::Throw(_) => self.0.fmt_sql(f, fmt),
			sql::Expr::Return(x) => {
				if x.fetch.is_some() {
					f.push('(');
					self.0.fmt_sql(f, fmt);
					f.push(')');
				} else {
					self.0.fmt_sql(f, fmt);
				}
			}
			sql::Expr::IfElse(_)
			| sql::Expr::Select(_)
			| sql::Expr::Create(_)
			| sql::Expr::Update(_)
			| sql::Expr::Upsert(_)
			| sql::Expr::Delete(_)
			| sql::Expr::Relate(_)
			| sql::Expr::Insert(_)
			| sql::Expr::Define(_)
			| sql::Expr::Remove(_)
			| sql::Expr::Rebuild(_)
			| sql::Expr::Alter(_)
			| sql::Expr::Info(_)
			| sql::Expr::Foreach(_)
			| sql::Expr::Let(_)
			| sql::Expr::Sleep(_)
			| sql::Expr::Explain {
				..
			} => {
				f.push('(');
				self.0.fmt_sql(f, fmt);
				f.push(')');
			}
		}
	}
}
