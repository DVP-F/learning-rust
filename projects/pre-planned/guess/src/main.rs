use std::{io::{self, Write}};

fn main() {
	const NUMBER: u32 = 12;
	let mut num: u32 = 0;

	let mut checking = true;
	while checking{
		print!("Guess a number or type 'exit'/^C to quit: "); 
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read input");

		for ch in input.chars() {
			if let Some(digit) = ch.to_digit(10){
				num = num * 10 + digit as u32;
			}
		}

		if input.to_lowercase() == "exit" {checking = false}
		else if num > NUMBER {
			println!("Too high~~")
		}
		else if num < NUMBER {
			println!("Too low bro")
		}
		else if num == NUMBER {
			println!("Congratulations! The number was {}", NUMBER);
			checking = false;
		}
	}
}
