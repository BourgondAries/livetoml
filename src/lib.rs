extern crate toml;

pub mod livetoml;
pub mod parseerror;

#[cfg(test)]
mod tests {
	use toml::Value;
	use livetoml;

	#[test]
	fn integer_assignment() {
		let table: Value = "[table] value = 1".parse().unwrap();
		let value = table.lookup("table.value");
		assert_eq!(value, Some(&Value::Integer(1)));
	}

	#[test]
	fn integer_reassignment() {
		let mut table: Value = "[table] value = 1".parse().unwrap();
		livetoml::execute_command(&mut table, "table.value = 2")
			.expect("Could not execute command");
		let value = table.lookup("table.value");
		assert_eq!(value, Some(&Value::Integer(2)));
	}

	#[test]
	fn float_reassignment() {
		let mut table: Value = "[table] value = 1.0".parse().unwrap();
		livetoml::execute_command(&mut table, "table.value = 2.0")
			.expect("Could not execute command");
		let value = table.lookup("table.value");
		assert_eq!(value, Some(&Value::Float(2.0)));
	}

	#[test]
	fn string_reassignment() {
		let mut table: Value = "[table] value = \"Hello\"".parse().unwrap();
		livetoml::execute_command(&mut table, "table.value = World")
			.expect("Could not execute command");
		let value = table.lookup("table.value");
		assert_eq!(value, Some(&Value::String(String::from("World"))));
	}

	#[test]
	fn bool_reassignment() {
		let mut table: Value = "[table] value = true".parse().unwrap();
		livetoml::execute_command(&mut table, "table.value = false")
			.expect("Could not execute command");
		let value = table.lookup("table.value");
		assert_eq!(value, Some(&Value::Boolean(false)));
	}

	#[test]
	fn manual() {
		let mut table: Value = "[table] value = true\nheight = 0.0\nip = \"0.1\"".parse().unwrap();
		let mut line = String::new();
		use std::io;
		use std::io::Write;
		while io::stdin().read_line(&mut line).expect("It's ok") > 0 {
			{
				let trimmed = line.trim();
				match livetoml::execute_command(&mut table, trimmed) {
					Ok(()) => {}
					Err(err) => { writeln!(&mut io::stderr(), "{:?}", err); }
				}
				writeln!(&mut io::stderr(), "{:?}", table);
			}
			line = String::new();
		}
	}

}
