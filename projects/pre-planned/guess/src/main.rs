use std::{io::{self, Write}};
use rand::Rng;

fn main() {
	let mut rng = rand::rng();
	let n: u32 = rng.random();
	let number = u64::from(n);

	let mut num: u64 = 0;

	let mut checking = true;
	while checking{
		print!("Guess a number or type 'exit'/^C to quit: ");
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");

		for ch in input.chars() {
			if let Some(digit) = ch.to_digit(10){
				num = num * 10 + digit as u64;
			}
		}

		if input.to_lowercase() == "exit" {checking = false}
		else if num > number {
			println!("Too high~~")
		}
		else if num < number {
			println!("Too low bro")
		}
		else if num == number {
			println!("Congratulations! The number was {}", number);
			checking = false;
		}
		num = 0;
	}
}
