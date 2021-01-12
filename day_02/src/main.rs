use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let part_two = args[2].parse::<bool>().unwrap();
	let reader = BufReader::new(File::open(filename).unwrap());

	let mut valid_passwords = 0;
	'outer: for l in reader.lines() {
		let line = l.unwrap();
		let range_split = line.find('-').unwrap();
		let character_index = line.find(' ').unwrap() + 1;
		let min = line[0..range_split].parse().unwrap();
		let max = line[range_split + 1..character_index - 1].parse().unwrap();
		let letter = &line[character_index..character_index + 1];
		let password = &line[character_index + 3..];

		if part_two {

			// Positions are indexed starting at 1
			let first = password[min - 1..min] == *letter;
			let second = password[max - 1..max] == *letter;
			if first != second {
				valid_passwords += 1;
			}
		} else {
			let character = letter.chars().next().unwrap();
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
	}

	println!("{}", valid_passwords);
}
