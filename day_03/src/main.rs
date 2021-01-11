use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

const TREE_STRING: &str = "#";

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let slope_x = args[2].parse::<usize>().unwrap();
	let slope_y = args[3].parse().unwrap();
	let reader = BufReader::new(File::open(filename).unwrap());

	let mut trees = 0;
	let mut pos = 0;
	let mut y = slope_y;
	for l in reader.lines() {
		if y == slope_y {
			let line = l.unwrap();
			if &line[pos..pos + 1] == TREE_STRING {
				trees += 1;
			}
			pos = (pos + slope_x) % line.len();
			y = 1;
		} else {
			y += 1;
		}

	}

	println!("{}", trees);
}
