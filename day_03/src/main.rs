use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

const TREE_STRING: &str = "#";
const SLOPE_RUN: usize = 3;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let reader = BufReader::new(File::open(filename).unwrap());

	let mut trees = 0;
	let mut pos = 0;
	for l in reader.lines() {
		let line = l.unwrap();
		if &line[pos..pos + 1] == TREE_STRING {
			trees += 1;
		}
		pos = (pos + SLOPE_RUN) % line.len();
	}

	println!("{}", trees);
}
