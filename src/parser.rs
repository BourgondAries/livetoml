use std::str::CharIndices;

/// Skip all left whitespace
fn ws(it: &mut CharIndices) {
	for (_, ch) in it {
		if ch.is_whitespace() == false {
			return
		}
	}
}

/// Parse a literal string
fn literal_string(it: &mut CharIndices) -> Option<String> {
	if let Some((_, ch)) = it.next() {
		let mut sum = String::new();
		if ch == '\'' {
			while let Some((_, ch)) = it.next() {
				if ch == '\'' {
					return Some(sum);
				}
				sum.push(ch);
			}
		}
	}
	None
}

/// Parse an interpreted string
fn interpreted_string(it: &mut CharIndices) -> Option<String> {
	if let Some((_, ch)) = it.next() {
		let mut sum = String::new();
		if ch == '\'' {
			while let Some((_, ch)) = it.next() {
				if ch == '\'' {
					return Some(sum);
				}
				sum.push(ch);
			}
		}
	}
	None
}

/// Gather the path of the key
fn key(it: &mut CharIndices) -> Option<Vec<String>> {
	Some(vec!())
}
