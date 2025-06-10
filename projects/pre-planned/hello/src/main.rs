use chrono::Local;
use regex::Regex;
// #[macro_use] extern crate text_io; // wouldve been nice but oh well cant bother to fixies :3
use std::io::{self, Write}; //::{self, BufRead};


fn string_to_uint32(time_str: &str) -> Result<u32, String> {
    if time_str.len() != 6 || !time_str.chars().all(|c| c.is_digit(10)) {
        return Err(format!("Invalid time string: {}", time_str));
    }

    match time_str.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(format!("Failed to parse time string: {}", e))
    }
}

fn time_category(time: u32) -> u16 {
	if time > 235959 {
		return 0;
	}

	;let morning:   u32 = 010000 * 06 // + 01 * 30 // 0630 - 06:30
	;let afternoon: u32 = 010000 * 12
	;let evening:   u32 = 010000 * 18
	;let next_day:  u32 = 010000 * 24
	;

	match time {
		d if d >= morning   && d < afternoon => 1, // morning
		d if d >= afternoon && d < evening   => 2, // afternoon
		d if d >= evening   && d < next_day  => 3, // eve
		d if                   d < morning   => 4, // night
		_ => 0
	}
}

fn main() {
	// let name = input("Ur nem >w< > ");
	print!(" ur nem plz >ω< >> ");
	io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
	// let stdin = io::stdin();
    // let in_1 = stdin.lock().lines().next().unwrap().unwrap();
	println!("");

	let instant_now = Local::now().to_string(); let mut time: u32 = 0;

	let time_re = Regex::new(r"(\d+\:){2}\d+").unwrap();
	if let Some(time_match) = time_re.find(&instant_now) {
		let time_str = time_match.as_str().replace(':', "");
		match string_to_uint32(&time_str) {
			Ok(num) => time = num,
			Err(e) => eprintln!("{}", e)
		}

	} else {
		eprintln!("Could not parse time from: {}", instant_now);
	}

	println!("Current time as uint32: {}", time);

	match time_category(time) {
		1 => println!("Good morning, {}", input),
		2 => println!("Good day to you, {}", input),
		3 => println!("Good evening, {}", input),
		4 => println!("Enjoy you next 12 hours, {}", input),
		0 | _ => eprintln!("Fault in parsing time.")
	}



	// let time_of_day_str_list = instant_now.split(' ').collect::<Vec<&str>>();
	// let temp = String::from(time_of_day_str_list);
	// let time_of_day_str = String::new();
	// String::from("{:?}", time_of_day_str_list);
	// for element in time_of_day_str_list {
	// 	time_of_day_str += element;
	// }
	// println!("{:?}", time_of_day_str);

	// let time_re = Regex::new(r"(\d+\:){2}\d+").unwrap();
	// let mut time_str_lst = Vec::<&str>::new();
	// for m in time_re.find_iter(&instant_now) {
	// 	let time = m.as_str();
	// 	time_str_lst.push(time);
	// }
	// let mut time = String::new();
	// for element in time_str_lst {
	// 	time.push_str(element);
	// }

	// for (_, [path, lineno, line]) in time_re.captures_iter(&instant_now).map(|c| c.extract()).collect::<Vec<_>>() { // idk how tf this works.
	// 	let parsed_lineno = lineno.parse::<u64>().unwrap_or(0);
	// 	time_str.push((path, parsed_lineno, line));
	// }

	// println!("{}", time);



	// let current_time = instant_now.time();
	// let numeric_time: u64; let numeric_time_str: &str;

	// let time = format!("{:?}", current_time);
	// let str_time: &str = time.as_str();
	// if let Some(end) = str_time.find('.'){
	// 	numeric_time_str = &str_time[..end];
	// 	match numeric_time_str.parse::<u64>() {
	// 		Ok(num) => numeric_time = num,
	// 		Err(_e) => numeric_time = 0
	// 	}
	// } else { numeric_time = 0 };

	// println!("Time since unix epoch: {}", numeric_time);

	// let since_utc_midnight = numeric_time % 86400;
	// let hours_in = since_utc_midnight / 3600;
	// let minutes_since_last_hour = since_utc_midnight - hours_in / 60;
	// let time_of_day = hours_in * 100 + minutes_since_last_hour;
	// println!("Hours into this day (UTC/GMT+00): {}", time_of_day);

	// println!("{} {}!!!", greeting, name);
}

/*
use std::io;

fn main() {
    print!(" ur nem plz >ω< >> "); // Note: uses print! (no newline)
    io::stdout().flush().unwrap(); // **critical step to force output immediately**
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("\nYou entered: {}", input.trim()); // Added debug output
}

! Ignore this shit use the above tech if possible

In How to read user input in Rust?
you can see how to iterate over all lines:

use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}

You can also manually iterate without a for-loop:

use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
}

You cannot write a one-liner to do what you want. But the following reads a single line (and is exactly the same answer as in How do I read a single String from standard input?):

use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
}

You can also use the text_io crate for super simple input:

#[macro_use] extern crate text_io;

fn main() {
    // reads until a \n is encountered
    let line: String = read!("{}\n");
}
--oli_obk
*/
