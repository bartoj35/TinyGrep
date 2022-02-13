extern crate tiny_grep;

use std::env;
use std::process;
use tiny_grep::Config;
use tiny_grep::run;

fn main() {
	let arguments: Vec<String> = env::args().collect();
	let config = Config::new(&arguments)
		.unwrap_or_else(|error| {
			println!("Problem parsing arguments: {}", error);
			process::exit(1);
		});

	if let Err(error) = run(config) {
		eprintln!(
			"Application error: {}", 
			error
		);
		process::exit(1);
	}
}

