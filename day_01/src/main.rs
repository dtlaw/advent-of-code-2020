use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let reader = BufReader::new(File::open(filename).unwrap());
	let mut input = Vec::new();
	for line in reader.lines() {
		input.push(line.unwrap().parse::<i32>().unwrap());
	}

	let front = &input[0..input.len() / 2];
	let back = &input[input.len() / 2..input.len()];

	for i in front {
		for j in back {
			if i + j == 2020 {
				println!("{}", i * j);
			}
		}
	}
}
