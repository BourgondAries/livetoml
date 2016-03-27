extern crate toml;
use std::str::CharIndices;
mod parser;

use std::io::Write;

macro_rules! debug {
	($($arg:tt)*) => {{
		writeln!(&mut ::std::io::stderr(), $($arg)*)
			.expect("failed printing to sterr");
	}}
}

trait Live {
	fn update(&mut self, tree: &str);
}

/*
pub enum Value {
	String(String),
	Integer(i64),
	Float(f64),
	Boolean(bool),
	Datetime(String),
	Array(Array),
	Table(Table),
}
*/

fn recursive_update(on: &mut toml::Value, by: &toml::Value) {
	use toml::Value;
	// Try updating a single string first
	debug!("on: ==== {:?}", on);
	debug!("by: ==== {:?}", by);
	match by {
		&Value::String(ref string) => {
			if let Value::String(ref mut to_string) = *on {
				*to_string = string.clone();
				debug!("Overwrote String!");
			} else {
				debug!("You don goofd");
			}
		}
		&Value::Integer(ref integer) => {
			if let Value::Integer(ref mut to_integer) = *on {
				*to_integer = *integer;
				debug!("Overwrote Int!");
			} else {
				debug!("You don goofd");
			}
		}
		&Value::Float(ref integer) => {
			if let Value::Float(ref mut to_integer) = *on {
				*to_integer = *integer;
				debug!("Overwrote Float!");
			} else {
				debug!("You don goofd");
			}
		}
		&Value::Boolean(ref integer) => {
			if let Value::Boolean(ref mut to_integer) = *on {
				*to_integer = *integer;
				debug!("Overwrote bool!");
			} else {
				debug!("You don goofd");
			}
		}
		&Value::Datetime(ref integer) => {
			if let Value::Datetime(ref mut to_integer) = *on {
				*to_integer = integer.clone();
				debug!("Overwrote bool!");
			} else {
				debug!("You don goofd");
			}
		}
		&Value::Array(ref integer) => {
			if let Value::Array(ref mut to_integer) = *on {

			} else {
				debug!("You don goofd");
			}
		}
		_ => {
			// debug!("{:?}", val);
		}
	}
}

fn expect(ch: char) -> bool {
	false
}

fn string_literal<'a>(index: &mut CharIndices<'a>) -> Option<String> {
	Some(String::from("ok"))
}

fn key(command: &str) -> &str {
	command
}

impl Live for toml::Value {

	fn update(&mut self, command: &str) {
		use toml::Value;
	}
}

#[cfg(test)]
mod tests {
	use toml;
	use Live;
	use std::io::Write;

	#[test]
	fn write() {
		let mut tree: toml::Value = r#"
			value = 0
			name = "ok dude"
		"#.parse().unwrap();

		debug!("{:?}", tree);
	}

}
