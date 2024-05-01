use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Guess a number!");
	let secret_number = rand::thread_rng().gen_range(1..=100);
	// println!("the number {secret_number}");
	loop {
		println!("input your guess: ");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to readline");
		println!("your guessed {guess}");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("too small"),
			Ordering::Greater => println!("too large"),
			Ordering::Equal => {
				println!("you win");
				break;
			}
		}
	}
}
