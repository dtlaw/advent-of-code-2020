use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

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
	let mut passport: HashMap<String, String> = HashMap::new();

	let re_year = Regex::new(r"^\d{4}$").unwrap();
	let re_hcl = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
	let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
	let re_pid = Regex::new(r"^\d{9}$").unwrap();

	// Chain a final blank line to replace the one ignored by the lines() iterator
	for l in reader.lines().chain(std::iter::once(Ok(String::from("")))) {
		let line = l.unwrap();
		if line.is_empty() {
			let mut valid = true;
			for &field in REQUIRED_FIELDS.iter() {
				if let Some(value) = passport.get(field) {
					valid = match field {
						"byr" => {
							if re_year.is_match(value) {
								let v = value.parse::<i32>().unwrap();
								v >= 1920 && v <= 2002
							} else {
								false
							}
						}
						"iyr" => {
							if re_year.is_match(value) {
								let v = value.parse::<i32>().unwrap();
								v >= 2010 && v <= 2020
							} else {
								false
							}
						}
						"eyr" => {
							if re_year.is_match(value) {
								let v = value.parse::<i32>().unwrap();
								v >= 2020 && v <= 2030
							} else {
								false
							}
						}
						"hgt" => {
							if let Some(c) = value.rfind("cm") {
								if let Ok(v) = value[0..c].parse::<i32>() {
									v >= 150 && v <= 193
								} else {
									false
								}
							} else if let Some(i) = value.rfind("in") {
								if let Ok(v) = value[0..i].parse::<i32>() {
									v >= 59 && v <= 76
								} else {
									false
								}
							} else {
								false
							}
						}
						"hcl" => {
							re_hcl.is_match(value)
						}
						"ecl" => {
							re_ecl.is_match(value)
						}
						"pid" => {
							re_pid.is_match(value)
						}
						_ => {
							true
						}
					};

					if !valid {
						break;
					}
				} else {
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
				let (key, value) = pair.split_at(pair.find(':').unwrap());
				passport.insert(key.to_string(), value.to_string());
			}
		}
	}

	println!("{}", valid_passports);
}
