mod reserved_keyword;

pub fn could_be_reserved_keyword(s: &str) -> bool {
	reserved_keyword::could_be_reserved_keyword(s)
}

#[cfg(test)]
mod tests {
	use super::could_be_reserved_keyword;

	#[test]
	fn reserved_keywords_are_case_insensitive() {
		assert!(could_be_reserved_keyword("select"));
		assert!(could_be_reserved_keyword("LET"));
		assert!(!could_be_reserved_keyword("not_a_keyword"));
	}
}
