use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

const REQUIRED_FIELDS: [&str; 7] = [
	"byr",
	"iyr",
	"eyr",
	"hgt",
	"hcl",
	"ecl",
	"pid",
];

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let reader = BufReader::new(File::open(filename).unwrap());

	let mut valid_passports = 0;
	let mut passport = HashMap::new();

	// Chain a final blank line to replace the one ignored by the lines() iterator
	for l in reader.lines().chain(std::iter::once(Ok(String::from("")))) {
		let line = l.unwrap();
		if line.is_empty() {
			let mut valid = true;
			for &field in REQUIRED_FIELDS.iter() {
				if !passport.contains_key(field) {
					valid = false;
					break;
				}
			}

			if valid {
				valid_passports += 1;
			}
			passport.clear();
		} else {
			for pair in line.split_whitespace() {
				let mut p = pair.split(':');
				let key = p.next().unwrap();
				let value = p.next().unwrap();
				passport.insert(key.to_string(), value.to_string());
			}
		}
	}

	println!("{}", valid_passports);
}
