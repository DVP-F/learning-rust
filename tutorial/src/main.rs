// Import HashMap
use std::collections::HashMap;

fn main() {
	println!("Hello, world! ");
	print!("wsg gangggg ");
	print!("ts firee\n");
	print!("newlining stuffs :3\n\t'ello 'arry - yer a wizarddd!!!!");
	// man rust could be pr cool tbhhhh
	/* gotta comment on syntax being very similar to python though frfr, excluding these comments uwu */ 
	//// also love that whitespace is FULLY optional here bc in py i could not do everything as a one-liner :o
	let name = "Johnny";
	println!("Name == {}", name); // {} is basically like %s as used primarily in earlier versions of py before like 3.0 or smn? not used to that
	let age = 4582;
	println!("{} is {}y old", name, age); // yep. %s but easier?? fun
	let x = 6; // constant 
	let mut y = 5; // variable 
	/* technically not but easier to consider them as such tbh. mut just adds 'mutable' letting it be changed later by reassignment */
	println!("{}",x); println!("{}",y); y=20; println!("{}",y); 
	/* excerpt from w3: 
Variable Values Cannot be Changed by Default

By default, variables in Rust cannot be changed after they are created:
let x = 5;
x = 10; // Error
println!("{}", x);
Change Variable Values

If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable):
Example
let mut x = 5;
println!("Before: {}", x);
x = 10;
println!("After: {}", x); 
	*/

	// dynamic casting ig 

	// '_' is to stop rust code analyzer from complaining about unused vars 
	/* im avoiding the ': i32', ': &'static str' inlay hints with 
	https://stackoverflow.com/questions/69909997/how-can-i-remove-type-annotation-help-when-using-rust-analyzer 
	https://rust-analyzer.github.io/book/features.html#hover */ 

	let _my_num = 3;					// integer 
	let _my_double = 5.87;				// floating point number 
	let _my_letter = 'D';				// character 
	let _my_bool = true;				// boolean 	[js style?] 
	let _my_text = "hewwo >w<";			// string 

	// compared to explicit cast 
	let _my_num: i32 = 3;				// integer (32-bit?)
	let _my_double: f64 = 5.87;			// floating point number (double 32-bit for float adds up yeah)
	let _my_letter: char = 'D';			// character (reminds me of cpp/cs/c tbh)
	let _my_bool: bool = true;			// boolean 	[js style?] 
	let _my_text: &str = "hewwo >w<";	// string 


	let name = "John";
	let age = 28;
	let is_admin = false;
	println!("Name: {}", name);
	println!("Age: {}", age);
	println!("Admin: {}", is_admin); 

	// annoyingly this is where constants come into play 
	// constants have to be initalized with a type: 
	const _BIRTHYEAR: i32 = 1980;
	const _MINUTES_PER_HOUR: i32 = 60;

	/* comparison:
			can change?		type required?
	const:		No				Yes
	var:	Yes, with mut		No
	 */


	// arithmetic is standard

	let add = 5 + 3;
	let sub = 10 - 4;
  	let mul = 6 * 2;
	let div = 12 / 3;
	let rem = 10 % 3;

	println!("Add: {}", add);
	println!("Sub: {}", sub);
	println!("Mul: {}", mul);
	println!("Div: {}", div);
	println!("Rem: {}", rem);


	// assigment uses 'normal' syntax 

	let mut z = 10;
	println!("Start: {}", z);

	z += 5;
	println!("After += 5: {}", z);

	z -= 2;
	println!("After -= 2: {}", z);

	z *= 2;
	println!("After *= 2: {}", z);

	z /= 3;
	println!("After /= 3: {}", z);

	z %= 4;
	println!("After %= 4: {}", z);


	// comparison operations too 

	let a = 5;
	let b = 10;

	println!("5 == 10: {}", a == b);
	println!("5 != 10: {}", a != b);
	println!("5 < 10: {}",  a <  b);
	println!("5 >= 10: {}", a >= b);


	// Logic operations 

	let logged_in = true;
	let admin = false;

	println!("Is regular user: {}",  logged_in && !admin);
	println!("Has any access: {}",   logged_in || admin);
	println!("Not logged in: {}",   !logged_in );


// Here '/*? ' removes vs code's colour coding based on operator like '?' or '!' within the comment bracket when on a newline :3 
/*? overview of operators from https://www.w3schools.com/rust/rust_operators.php 

Operator 	Meaning 					Example use 		Result 
+			Addition					5 + 3				8 
-			Subtraction					5 - 3				2 
*			Multiplication				5 * 3				15 
/			Division					10 / 2				5 
%			Modulo (Remainder)			11 % 2				1 

=			Equals						x = 5				Assign 5 to x 
+=			Equals plus					x += 4				x = x + 4 
-=			Equals minus				x -= 3				x = x - 3 
*=			Equals times				x *= 2				x = x * 2 
/=			Equals divided by			x /= 7				x = x / 7 
%=			Equals by modulo 			x %= 13				x = x % 13 

==			Is equal to 				6 == 6				true 
!=			Is not equal to				7 != 7				false 
>			Greater than				8 > 4				true 
<			Less than					2 < 34				true 
>=			Greater than or equal to	5 >= 5				true 
<=			Less than or equal to		4 <= 3				false 

&&			AND	[true if both are]		
||			OR	[true if either is]		 ̌̌̌ ̌̌̌  see truth tables below ̌̌̌ ̌̌̌ 
!			NOT	[invert boolean truth]	

​		AND							OR						NOT 
true  | true  | true		true  | true  | true		true  | false 
true  | false | false		true  | false | true		false | true 
false | true  | false		false | true  | true 
false | false | false		false | false | false 

*/

	// bool from comparison
	let age = 20;
	let can_vote = age >= 18; 

	println!("Can vote? {}", can_vote);

	// also if else blocks :3 
	if can_vote {
		println!("Sup bbg ùwúh~");
	} else {
		println!("Suh dude?")
	}

	let x = 4; let y = 2;

	if 7 > 5 {
		println!("Thats math :o");
	} else if x % y == 0 {
		println!("woag im here?!?!?!?!?!");
	}

	let tim = 2000;
	let greeting = if tim > 1800 { "Good day my good sir." } else { "Good evenin me laddd!!!!" };
/* used ctrl + shift + p > 'rust-analyzer join lines' to make this from:
	let greeting = if tim > 1800 {
		"Good day my good sir."
	} else {
		"Good evenin me laddd!!!!"
	};
*/
	println!("{}", greeting);

	// match <3 my beloved 
	let day = 4;

	match day { // so weird to not use colon cannot lie 
		1 => println!("Garfeld"),
		2 => println!("Chewsdayy"),
		3 => println!("wenzdɑ̄e"),
		4 => println!("Thursday"),
		5 => println!("Freaky"),
		6 => println!("Jupiter"),
		7 => println!("AHHHHHH MY EYESSSSSSSSSSSS!!!!!!!!!!"),
		_ => println!("Invalid bastard"),
	}

	let result = match day {
		1|2|3|4|5 => "~~Weekday~~", // okay this is a nice feature. 
		6 | 7 => "Bloody weekend",
		_ => "Invalidity" 
	};

	println!("{}", result);

	// LOOPING δδδ 
	let mut count = 1;
	let result2 = loop {
		println!("HI! :3c");

		if count == 3 { break count; } 

		count += 1;
	};

	println!("Loop stopped at {} rounds", result2);

	let mut count = 1;

	while count <= 10 {
		if count == 3 { count += 1; continue; }

		if count == 6 {
			break;
		}

		println!("Count: {}", count);
		count += 1;
	}

	for i in 1..6 { // HOLY BASED RANGE DEFINITION I *NEED* THIS SO BAD --- equals [1, 2, 3, 4, 5] or range(1,6,1) in py 
		println!("{}", i);
	}
	for i in 2..=7 { // and inclusive for that badonka donk range(2, 8) but BETTER!!!! 
		if i == 3 { continue; }
		if i >= 5 { break; }
		print!("{}", i); 
	}
	print!("\n");


	// stringsssss
	let gretathunberg: &str = "How dare you.";
	println!("{}",gretathunberg);
	let text001 = gretathunberg.to_string();
	let mut text002 = String::from(gretathunberg);
	text002.push_str(" How dare you!");
	let mut stringgreet = String::from("Hi");
	stringgreet.push('i');

	println!("{}\n{}", stringgreet, text001);

	let s1 = String::from("Hewwo");
	let s2 = String::from("Gweetings");
	let s3 = String::from("HI! >W<");
	let resstr = format!("{} {} {}", s1, s2, s3); // combining strings
	let restr = s1 + " " + &s2 + " " + &s3; //       and in a very tedious fashion
/* Note: You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here. --- w3 */

	println!("{}\n{}", resstr, restr);

	println!("Length of resstr: {}", resstr.len());


	// OWNERSHIP - dump at ../ownership.md 
	let a1 = String::from("Hello");
	let b1 = a1;

	// println!("{}", a1); Error: a no longer owns the value 
	println!("{}", b1); // Ok: b now owns the value 

	// Effectively, variables are by default 'pointers' -- and act more like c/cs/cpp than py. 

	// Except simple types like numbers, chars, bools: 
	let a = 5;
	let b = a;
	println!("a = {}", a); 
	println!("b = {}", b); 

	// For python default behaviour you can use clone or borrowing/references:
	let a2 = String::from("Hey");
	let b2 = a2.clone();

	println!("a2 = {}", a2); 
	println!("b2 = {}", b2); 

	// references (which i internally relate to pointers to variables in cpp) and borrowing 
	let c = String::from("H");
	let d = &c;
	println!("c = {}", c); 
	println!("d = {}", d); 

	let mut name = String::from("John");
	let name_ref = &mut name;
	name_ref.push_str(" Doe");

	println!("{}", name_ref); // John Doe  \/\/\/  and this reinforces that relation. 


	//? DATA STRUCTURES ?// 
	// The humble array [fixed-size list] 
	let fruit = ["appel", "banana", "pineappel"];
	println!("Last fruit: {}", fruit[fruit.len()-1]);

	// Vector - normal list. 
	let mut fruits = vec!["apple", "banana"];
	fruits.push("cherry");

	println!("Third fruit: {}", fruits[2]); 

	// Tuple holds several types of data but is more like a class in value calling 
	let person = ("John", 30, true);
	println!("Name: {}", person.0);
	println!("Age: {}", person.1);
	println!("Is active: {}", person.2); 

	// Hashmaps are dictionaries but weird to make it looks like? key-value pairs anyways :3 
	let mut capital_cities = HashMap::new();
	capital_cities.insert("France", "Paris");
	capital_cities.insert("Japan", "Tokyo");

	println!("Capital of Japan is {}", capital_cities["Japan"]);

//? OVERVIEW		(yoinked from w3) 
/*
Data Structure		Use Case								Can Grow?
---------------------------------------------------------------------
Array				Fixed-size list of same-type values 	No
Vector (Vec) 		Growable list of same-type values		Yes
Tuple				Group different types together			No
HashMap 			Key-value lookup						Yes
*/ 

	// more fonctions of the structures:
	// arrays
	let mut numbers = [1, 2, 3, 4, 5];
	numbers[0] = 10;
	println!("The new first number is: {}", numbers[0]);

	println!("This array has {} elements.", numbers.len());

	for fruity in &fruit { // rember to loop with borrowed arrays/vecs etc. otherwise theyll be 'used up' 
		println!("I like {}.", fruity);
	}

	println!("{:?}", numbers); // print the entire array 

/* A good rule is to use {} for basic types like strings, numbers, and booleans, 
and {:?} for data structures like arrays and vectors - when printing the entire structure. */ // quote from our overlords 

	// vectors 
	fruits.pop();
	println!("{:?}", fruits); // ["apple", "banana"] 
	fruits.insert(0, "orange");
	println!("{:?}", fruits); // ["orange", "apple", "banana"] 
	fruits.remove(0);
	println!("{:?}", fruits); // ["apple", "banana"] 

	for fruitatious in &fruits { 
		println!("I like {}.", fruitatious);
	}

	// tuples
	// unpacking as an opposite to creating a tuple, called packing it. 
	let (name, age, active) = person;

	println!("Name: {}", name);
	println!("Age: {}", age);
	println!("Active: {}", active);

	// hashmaps
	capital_cities.insert("England", "London");
	capital_cities.insert("Germany", "Berlin");
	capital_cities.insert("Norway", "Oslo");

	if let Some(city) = capital_cities.get("England") {
		println!("The capital of England is {}.", city);
	} else {
		println!("England is not in the map.");
	} 

	capital_cities.insert("England", "London");
	capital_cities.insert("England", "Berlin");

	println!("{:?}", capital_cities); 

	// Remove the key "England"
	capital_cities.remove("England");

	// Loop through the HashMap
	for (country, city) in &capital_cities {
		println!("The capital of {} is {}.", country, city);
	} 


	// STRUCTS (I love custom data structures) 
	// Create a Struct called Person
	struct Person {
		name: String,
		age: u32,
		can_vote: bool,
	}

	// Create a Person object
	let mut user = Person {
		name: String::from("John"),
		age: 35,
		can_vote: true,
	};

	// change age first
	user.age = 27; 

	// Access and print the values
	println!("Name: {}", user.name);
	println!("Age: {}", user.age);
	println!("Can vote? {}", user.can_vote); 

	// enumsssss (is that a cpp reference??) 
	enum Direction {
		Up,
		Down,
		Left,
		Right
	}
	let my_direction = Direction::Left;
	match my_direction {
		Direction::Up => println!("Going up"),
		Direction::Down => println!("Going down"),
		Direction::Left => println!("Going left"),
		Direction::Right => println!("Going right"),
	}

	enum LoginStatus {
		Success(String),
		Error(String),
	}
	let result1 = LoginStatus::Success(String::from("Welcome, John!"));
	let result2 = LoginStatus::Error(String::from("Incorrect password"));

	match result1 {
		LoginStatus::Success(message) => println!("Success: {}", message),
		LoginStatus::Error(message) => println!("Error: {}", message),
	} 
	match result2 {
		LoginStatus::Success(message) => println!("Success: {}", message),
		LoginStatus::Error(message) => println!("Error: {}", message),
	} 


// FUCNTION CALLS // 
	hewwo(); 
	
	let sum = adder(3, 4);
	println!("Sum is: {}", sum);
}

// Functions 
fn hewwo() {
	println!("Hewwo fwom a funwction!! >w<"); 
}

fn adder(a: i32, b: i32) -> i32 {
	// return a + b;
	a + b // returning without return keyword 
}

// SCOPE: https://www.w3schools.com/rust/rust_scope.php - dumped to ../scope.md 
// extremely similar to cpp/py/js though. 

