use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let reader = BufReader::new(File::open(filename).unwrap());

	let mut valid_passwords = 0;
	'outer: for l in reader.lines() {
		let line = l.unwrap();
		let range_split = line.find("-").unwrap();
		let character_index = line.find(" ").unwrap() + 1;
		let min = line[0..range_split].parse::<i32>().unwrap();
		let max = line[range_split + 1..character_index - 1].parse::<i32>().unwrap();
		let character = line[character_index..character_index + 1].chars().next().unwrap();
		let password = &line[character_index + 3..];

		let mut occurances = 0;
		for c in password.chars() {
			if c == character {
				occurances += 1;
			}

			if occurances > max {
				continue 'outer;
			}
		}

		if occurances >= min {
			valid_passwords += 1;
		}
	}

	println!("{}", valid_passwords);
}
