// Word Count Functionality
// Katherine Riedling
use std::io::*;
use std::fs::*;
use std::env;

//Read the contents of a file as a string
fn read_file(filename:String) -> String {
	let mut f_file = File::open(filename).expect("Failed to open file");
	let mut f_content = String::new();
	f_file.read_to_string(&mut f_content).expect("Failed to read to string");
	return f_content;
}

//Count the characters in a line
fn count_line_char(line_contents:String) -> i32 {
	let num_char = line_contents.chars().count() as i32;
	return num_char;
}

//Count the words in a line
fn count_line_words(line_contents:String) -> i32 {
	let mut words_iter = line_contents.split_whitespace();
	let mut word_count = 0;
	loop{
		match words_iter.next(){
			Some(_x) => {
				word_count = word_count + 1;
			},
			None => { break }
		}
	}
	return word_count;
}

//Count the lines in a file and call per-line counting functions
fn count_lines(file_contents:String) -> (i32, i32, i32) {
	let mut lines = file_contents.lines();
	let mut line_count = 0; 
	let mut word_count = 0;
	let mut char_count = 0;

	loop{
		match lines.next(){
			Some(l) => {
				line_count = line_count + 1;
				let c = count_line_char(l.to_string());
				let w = count_line_words(l.to_string());
				char_count = char_count + c;
				word_count = word_count + w;
			}
			None => { break }
		}
	}
	return (line_count, word_count, char_count);
}

//Call all the file counting functions
fn wcf_in_full(filename:String) -> (i32, i32, i32) {
	let contents = read_file(filename);
	let file_counts = count_lines(contents);
	
	println!("Number of lines:{0}\nNumber of words:{1}\nNumber of chars:{2}\n", 
		file_counts.0, 
		file_counts.1, 
		file_counts.2);
	return file_counts;
}

//Main - loop over the files
fn main() {
	let mut filename : String;
	let mut counts : (i32,i32,i32);
	let mut tot_lines = 0;
	let mut tot_words = 0;
	let mut tot_chars = 0;
	
	let args: Vec<String> = env::args().collect();
	let mut iter = args.iter().skip(1);

	loop{
		match iter.next(){
			Some(i) => {
				filename = i.to_string();
				println!("{}",filename.clone());
				counts = wcf_in_full(filename); 
				tot_lines = tot_lines + counts.0;
				tot_words = tot_words + counts.1;
				tot_chars = tot_chars + counts.2;
			}
			None => { break }
		}
	}

	//Print total values here
	println!("Total values\nNumber of lines:{0}\nNumber of words:{1}\nNumber of chars:{2}\n", 
		tot_lines,
		tot_words,
		tot_chars);
}