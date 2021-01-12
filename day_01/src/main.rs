use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn recurse(input: &[i32], target: i32, sum: i32, index: usize, numbers: usize) -> Option<Vec<i32>> {
	if numbers > 0 {
		for (i, v) in input[index..input.len() - (numbers - 1)].iter().enumerate() {
			if let Some(mut x) = recurse(input, target, sum + v, i + 1, numbers - 1) {
				x.push(*v);
				return Some(x)
			}
		}
		None
	} else if sum == target {
		Some(Vec::new())
	} else {
		None
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let numbers = args[2].parse::<usize>().unwrap();
	let reader = BufReader::new(File::open(filename).unwrap());
	let mut input = Vec::new();
	for line in reader.lines() {
		input.push(line.unwrap().parse::<i32>().unwrap());
	}

	if let Some(res) = recurse(&input, 2020, 0, 0, numbers) {
		for v in res {
			println!("{}", v);
		}
	} else {
		println!("No result found");
	}

}
