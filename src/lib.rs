use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
	pub case_sensitivity: bool,
}

impl Config {
    pub fn new(arguments: &[String]) -> Result<Config, &'static str> {
        if arguments.len() < 3 {
            return Err("Not enought arguments were passed");
        }

        let query = arguments[1].clone();
        let filename = arguments[2].clone(); 
		// clone makes it more straightforward, we don't have to worry about lifetime

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
			query: query,
			filename: filename,
			case_sensitivity: case_sensitive,
		 })
    }
}



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

	let res = if config.case_sensitivity {
		search(&config.query, &content)
	} else {
		search_case_insensitive(&config.query, &content)
	};

	for line in res {
		println!("{}", line);
	}
    
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut res = Vec::new();
	
	for line in content.lines() {
		if line.contains(query) {
			res.push(line);
		}
	}

	res
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut res = Vec::new();

	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
			res.push(line);
		}
	}

	res
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!( 
			vec!["safe, fast, productive."], 
			search(query, content)
		);
	}

	#[test]
	fn case_insensitive() {
		let query ="rUsT";
		let content = "\
Rust:
safe, fast, productive/
Pick three.
Trust me.";
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, content)
		);
	}
}

