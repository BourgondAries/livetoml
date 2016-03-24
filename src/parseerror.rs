#[derive(Debug)]
pub enum ParseErrorKind {
	MissingSpace,
	MissingOperator,
	MissingCommandAfterSpace,
	NoSuchPath,
	NoSuchOperation,
}

#[derive(Debug)]
pub struct ParseError {
	pub kind: ParseErrorKind
}

impl ParseError {
	pub fn new(kind: ParseErrorKind) -> ParseError {
		ParseError {
			kind: kind
		}
	}
}
