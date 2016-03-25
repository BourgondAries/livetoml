use toml;
use toml::Value;
use parseerror::{ParseError, ParseErrorKind};

pub trait Interpret {
	fn eval(&mut self, command: &str) -> Result<(), ParseError>;
}

impl Interpret for toml::Value {
	fn eval(&mut self, command: &str) -> Result<(), ParseError> {
		execute_command(self, command)
	}
}

pub fn execute_command(table: &mut toml::Value, command: &str) -> Result<(), ParseError> {
	let result = try!(parse_command(command));
	let (lookup, operation, value) = result;
	let mut atom = lookup_mut(table, lookup);
	if let Some(ref mut atom) = atom {
		 try!(operate_on_value(atom, operation, value));
	} else {
		return Err(ParseError::new(ParseErrorKind::NoSuchPath));
	}
	Ok(())
}

fn operate_on_value(atom: &mut Value, operation: &str, value: &str)
	-> Result<(), ParseError> {
	match operation {
		"=" => assign_to_atom(atom, value),
		_ => Err(ParseError::new(ParseErrorKind::NoSuchOperation)),
	}
}

fn assign_to_atom(atom: &mut Value, value: &str) -> Result<(), ParseError> {
	match *atom {
		Value::Integer(ref mut integer) => {
			*integer = match value.parse::<i64>() {
				Ok(v) => v,
				Err(_) => return Err(ParseError::new(ParseErrorKind::TypeMismatch)),
			};
		},
		Value::Float(ref mut float) => {
			*float = match value.parse::<f64>() {
				Ok(v) => v,
				Err(_) => return Err(ParseError::new(ParseErrorKind::TypeMismatch)),
			};
		},
		Value::String(ref mut string) => {
			*string = String::from(value);
		},
		Value::Boolean(ref mut boolean) => {
			*boolean = match value {
				"true" => true,
				"false" => false,
				_ => return Err(ParseError::new(ParseErrorKind::NotABooleanValue)),
			};
		},
		_ => return Err(ParseError::new(ParseErrorKind::NoAssignmentHandler)),
	}
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
	if index + 1 < slice.len() {
		Ok((&slice[..index], &slice[index+1..]))
	} else {
		Err(ParseError::new(ParseErrorKind::MissingCommandAfterSpace))
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

use std::str::Split;

fn lookup_mut_recurse<'a>(value: &'a mut Value, matches: &mut Split<'a, char>)
	-> Option<&'a mut Value> {
	if let Some(key) = matches.next() {
		match *value {
			Value::Table(ref mut hm) => {
				match hm.get_mut(key) {
					Some(v) => return lookup_mut_recurse(v, matches),
					None => return None,
				}
			},
			Value::Array(ref mut v) => {
				match key.parse::<usize>().ok() {
					Some(idx) if idx < v.len()
						=> return lookup_mut_recurse(&mut v[idx], matches),
					_ => return None,
				}
			},
		_ => return None
		}
	}

	Some(value)
}

fn lookup_mut<'a>(value: &'a mut Value, path: &'a str) -> Option<&'a mut Value> {
	if path.len() == 0 {
		return Some(value)
	}

	let mut matches = path.split('.');
	lookup_mut_recurse(value, &mut matches)
}
