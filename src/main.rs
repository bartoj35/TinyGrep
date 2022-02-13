use std::env;
use std::process;



fn main() {
	let arguments: Vec<String> = env::args().collect();
	let config = Config::new(&arguments)
		.unwrap_or_else(|error| {
			println!("Problem parsing arguments: {}", error);
			process::exit(1);
		});

	println!("Searching for: {}", config.query);
	println!("In file: {}", config.filename);
	
	if let Err(error) = run(config) {
		println!("Application error: {}", error);
		process::exit(1);
	}
}

