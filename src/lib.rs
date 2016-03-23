extern crate toml;

pub mod livetoml;
pub mod parseerror;

#[cfg(test)]
mod tests {
	use toml::Value;
	use toml;
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
		livetoml::execute_command(&mut table, "a = b")
			.expect("Could not execute command");
		let value = table.lookup("table.value");
	}
}
