extern crate toml;

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
				debug!("Overwrote String!");
			} else {
				debug!("You don goofd");
			}
		}
		_ => {
			// debug!("{:?}", val);
		}
	}
}

impl Live for toml::Value {

	fn update(&mut self, tree: &str) {
		use toml::Value;

		debug!("{:?}", self);

		let mut tree: toml::Value = tree.parse().unwrap();
		match tree {
			Value::Table(ref mut table) => {
				if let Value::Table(ref mut onto) = *self {
					for (key, ref value) in table {
						recursive_update(onto.get_mut(key).unwrap(), value);
					}
				} else {
					panic!("should not happen");
				}
			}
			_ => {}
		};
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
		tree.update(r#"
			value = 100
			name = "whatever"
		"#);

		debug!("{:?}", tree);
	}

}
