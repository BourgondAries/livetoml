# LiveToml

The livetoml library is a super tiny interpreter for toml-based data stores.

# Usage

First parse a toml table using the toml library. Then execute a command on it.

	let mut table: Value = "[table] value = true".parse().unwrap();
	livetoml::execute_command(&mut table, "table.value = false").expect("");
	println!("{:?}", table);

The command will be interpreted and the change will be made.

# Goal

The goal of livetoml is to be an in-program, simple interpreter. The main idea is to have two threads, one getting input, and the "logic" thread processing the input.

	let mut toml = "[hello] world = 1".parse().unwrap();
	let mut bindstore = BindStore::new();
	let mut world: i32;
	bindstore.bind("hello.world", &mut world);
	loop {
		let input = get_input_timeout(10);
		if let Some(input) = input {
			execute_command(&mut toml, &input);
			bindstore.process(&mut toml);
		}
	}

# Grammar

The grammar starts with a statement symbol:

	start ::= stmt;

## Assignment

	stmt ::= path ops val;
	path ::= LOOKUP
	ops ::= '=';
	val ::= string | number;

# Example Statement

	foo = hello world
	foo = this is a string
	foo = it starts after the space after '=', for simple parsing. Continues to the end of the string.

	bar = 1
	bar = 100
	bar = -30
	bar = 0

Note that you always need spaces around operations. This is to keep the parser super-simple.
All the parser does is split at the first two spaces. Nothing fancy as LL(1) or LALR.

