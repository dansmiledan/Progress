use std::env;
use std::process;
use minigrep::Config;

fn main() {
	// let config = parse_config(&args);
	let config = Config::build(env::args()).unwrap_or_else(|err| {
		println!("parse cmd failed! {err}");
		process::exit(1);
	});
	
	if let Err(e) = minigrep::run(config) {
		println!("Application error: {e}");
        process::exit(1);
	}

}
