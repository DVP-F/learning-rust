use std::{io::{self, Write}};
// use expr::Parser;
// there are two main ways to do this. the better, easier version uses the expr::Parser; crate
//? I will not get into pest, nom, or combine for this.
// All ways: for loop on char of string input, rlua, expr::Parser;, pest etc..

fn main() {
	let mut number1: i64 = 0;
	let mut number2: i64 = 0;
	let mut op: u8 = 1; let mut is_float = false;
	let mut output: i64 = 0; let mut outputf: f64 = 0.0;

	print!("Type in an expression (+-*/): "); 
	io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	// *py*: input_expression = input("Type in an expression (+-*/): ")

	for ch in input.chars() {
		if let Some(digit) = ch.to_digit(10){
			if op == 1 {
				number1 = number1 * 10 + digit as i64;
			}
			else if op == 2 {
				number2 = number2 * 10 + digit as i64;
			}
		} else {op += 1; continue}
	}
	
	for ch in input.chars() {
		match ch {
			c if c == '+' => {output  = number1        + number2; break;},
			c if c == '-' => {output  = number1        - number2; break;},
			c if c == '*' => {output  = number1        * number2; break;},
			c if c == '/' => {outputf = number1 as f64 / number2 as f64; is_float = true; break;},
			c if c == '%' => {outputf = number1 as f64 % number2 as f64; is_float = true; break;},
			_ => output = 0
		}
	}
	if !is_float {
		println!("Your input: {} computes to: {}", input, output);
	} else {
		println!("Your input: {} computes to: {}", input, outputf);
	}
}
