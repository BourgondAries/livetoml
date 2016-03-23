use toml;
use std::error::Error;
use parseerror::{ParseError, ParseErrorKind};


pub fn execute_command(table: &mut toml::Value, command: &str) -> Result<(), ParseError> {
	let result = try!(parse_command(command));
	Ok(())
}

fn parse_command<'a>(command: &'a str) -> Result<(&'a str, &'a str, &'a str), ParseError>  {
	let first = try!(find_space_from(command));
	let (lookup, rest) = try!(split_at(command, first));
	let second = try!(find_space_from(rest));
	let (operation, rest) = try!(split_at(rest, second));
	Ok((lookup, operation, rest))
}

fn split_at(slice: &str, index: usize) -> Result<(&str, &str), ParseError> {
	if index < slice.len() {
		Ok((&slice[..index], &slice[index+1..]))
	} else {
		Err(ParseError::new(ParseErrorKind::InternalIndexError))
	}
}

fn find_space_from(slice: &str) -> Result<usize, ParseError> {
	let mut accumulator = 0usize;
	for i in slice.chars() {
		if i == ' ' {
			return Ok(accumulator);
		}
		accumulator += 1;
	}
	Err(ParseError::new(ParseErrorKind::MissingSpace))
}
