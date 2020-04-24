// Basic syntax examples for Group B presentation
// Katherine Riedling

fn demo_conditions() -> bool {	
	let immutable = 30;
	let mut mutable = 10;

	let equal: bool;
	if immutable > 0 {
		while mutable < 30 {
			mutable = mutable + 1;
		}

	}
	equal = immutable == mutable;
	println!("It is equal! True or false?\t{}", equal.to_string());
	return equal;
}

fn demo_looping() {

	let book: [String; 5] = ["Introduction".to_string(), 
		"Act I".to_string(), 
		"Act II".to_string(), 
		"Act III".to_string(), 
		"Conclusion".to_string()]; 

	let mut iter = book.iter();

	println!("Loop looping");
	loop{
		match iter.next(){
			Some(title) => {
				println!("{}", title);
			}
			None => { break }
		}
	}

	println!("For loop");
	for chapter in book.iter() {
		println!("{}", chapter);
	}
}

fn demo_shadowing() {
	let shadow = "example";
	let shadow = "I shadowed this variable! Hooray!";
	println!("{}", shadow);
}

//Main
fn main() {
	const MOST_MONEY: f32 = 999_999.99;
	let mut _args: Vec<String>;
	let equal: bool = demo_conditions();

	demo_looping();
	demo_shadowing();
}