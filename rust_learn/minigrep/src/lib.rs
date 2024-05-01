use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	println!("search {} in {}", config.query, config.file_path);
	let contents = fs::read_to_string(config.file_path)?;
	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};
	for line in results {
		println!("{line}");
	}
	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&str> = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&str> = Vec::new();
	let query = query.to_lowercase();
	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
    results
}
pub struct Config {
	query: String,
	file_path: String,
	ignore_case: bool
}

impl Config {
	pub fn build(args: &[String])-> Result<Config, & str> {
		if args.len() < 3 {
			return Err("have no enough params");
		}
		let query = args[1].clone();
		let file_path = args[2].clone();
		let ignore_case = env::var("IGNORE_CASE").is_ok();
		return Ok(Config {query, file_path, ignore_case});
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";
		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Trust me.";
assert_eq!(
	vec!["Rust:", "Trust me."],
	search_case_insensitive(query, contents)
);
	}
}