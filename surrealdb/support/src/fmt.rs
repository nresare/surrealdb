//! Low-level SurrealQL formatting utilities.

mod escape;

use std::cell::Cell;
use std::fmt::Display;

pub use escape::{EscapeKwFreeIdent, EscapeKwIdent, EscapeObjectKey, EscapeRidKey, QuoteStr};
use surrealdb_types::{SqlFormat, ToSql, fmt_non_finite_f64};

/// Implements `ToSql` by calling a formatter on the wrapped contents.
pub struct Fmt<T, F> {
	contents: Cell<Option<T>>,
	formatter: F,
}

impl<T, F: Fn(T, &mut String, SqlFormat)> Fmt<T, F> {
	pub fn new(t: T, formatter: F) -> Self {
		Self {
			contents: Cell::new(Some(t)),
			formatter,
		}
	}
}

impl<T, F: Fn(T, &mut String, SqlFormat)> ToSql for Fmt<T, F> {
	fn fmt_sql(&self, f: &mut String, fmt: SqlFormat) {
		let contents = self.contents.replace(None).expect("only call Fmt::fmt once");
		(self.formatter)(contents, f, fmt)
	}
}

impl<I: IntoIterator<Item = T>, T: ToSql> Fmt<I, fn(I, &mut String, SqlFormat)> {
	pub fn comma_separated(into_iter: I) -> Self {
		Self::new(into_iter, fmt_comma_separated)
	}

	pub fn verbar_separated(into_iter: I) -> Self {
		Self::new(into_iter, fmt_verbar_separated)
	}

	pub fn pretty_comma_separated(into_iter: I) -> Self {
		Self::new(into_iter, fmt_pretty_comma_separated)
	}

	pub fn one_line_separated(into_iter: I) -> Self {
		Self::new(into_iter, fmt_one_line_separated)
	}
}

fn fmt_comma_separated<T: ToSql, I: IntoIterator<Item = T>>(
	into_iter: I,
	f: &mut String,
	fmt: SqlFormat,
) {
	for (i, v) in into_iter.into_iter().enumerate() {
		if i > 0 {
			f.push_str(", ");
		}
		v.fmt_sql(f, fmt);
	}
}

fn fmt_verbar_separated<T: ToSql, I: IntoIterator<Item = T>>(
	into_iter: I,
	f: &mut String,
	fmt: SqlFormat,
) {
	for (i, v) in into_iter.into_iter().enumerate() {
		if i > 0 {
			f.push_str(" | ");
		}
		v.fmt_sql(f, fmt);
	}
}

fn fmt_pretty_comma_separated<T: ToSql, I: IntoIterator<Item = T>>(
	into_iter: I,
	f: &mut String,
	fmt: SqlFormat,
) {
	for (i, v) in into_iter.into_iter().enumerate() {
		if i > 0 {
			if fmt.is_pretty() {
				f.push_str(",\n");
			} else {
				f.push_str(", ");
			}
		}
		v.fmt_sql(f, fmt);
	}
}

fn fmt_one_line_separated<T: ToSql, I: IntoIterator<Item = T>>(
	into_iter: I,
	f: &mut String,
	fmt: SqlFormat,
) {
	for (i, v) in into_iter.into_iter().enumerate() {
		if i > 0 {
			f.push('\n');
		}
		v.fmt_sql(f, fmt);
	}
}

/// Creates a formatting function that joins iterators with an arbitrary separator.
pub fn fmt_separated_by<T: ToSql, I: IntoIterator<Item = T>>(
	separator: impl Display,
) -> impl Fn(I, &mut String, SqlFormat) {
	move |into_iter: I, f: &mut String, fmt: SqlFormat| {
		let separator = separator.to_string();
		for (i, v) in into_iter.into_iter().enumerate() {
			if i > 0 {
				f.push_str(&separator);
			}
			v.fmt_sql(f, fmt);
		}
	}
}

pub struct Float(pub f64);

impl ToSql for Float {
	fn fmt_sql(&self, f: &mut String, fmt: SqlFormat) {
		match fmt_non_finite_f64(self.0) {
			Some(special) => f.push_str(special),
			None => {
				self.0.fmt_sql(f, fmt);
				f.push('f');
			}
		}
	}
}
