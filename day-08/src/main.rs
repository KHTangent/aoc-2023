use std::{collections::HashMap, fs};

use regex::Regex;

fn solution(input: &String) -> i32 {
	let mut steps = 0;
	let line_regex = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();
	let directions: Vec<char> = input.lines().next().unwrap().chars().collect();
	let mut map: HashMap<String, (String, String)> = HashMap::new();
	for line in input.lines().skip(2) {
		let matches = line_regex.captures(line).unwrap();
		map.insert(
			matches[1].to_owned(),
			(matches[2].to_owned(), matches[3].to_owned()),
		);
	}
	let mut current_pos = "AAA";
	for c in directions.iter().cycle() {
		if current_pos == "ZZZ" {
			break;
		}
		steps += 1;
		let next_step = map.get(current_pos).unwrap();
		current_pos = match c {
			'L' => next_step.0.as_str(),
			'R' => next_step.1.as_str(),
			_ => panic!(),
		};
	}
	steps
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
		);
		let answer = solution(&input);
		assert_eq!(answer, 2);
	}
	#[test]
	fn test2_solution() {
		let input = String::from(
			r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
		);
		let answer = solution(&input);
		assert_eq!(answer, 6);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file());
	println!("Answer: {}", answer);
}
