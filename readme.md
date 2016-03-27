# LiveToml

The livetoml library is a super tiny interpreter for toml-based data stores.
All it does is take in toml-compatible input. Then it merges that tree with the
existing tree.

# Usage

First parse a toml table using the toml library. Then execute a command on it.

	let mut table: Value = "[table] value = true".parse().unwrap();
	table.update("[table] other = false").unwrap();
	println!("{:?}", table);

# Goal

The goal of livetoml is to easily update data in a running program.
