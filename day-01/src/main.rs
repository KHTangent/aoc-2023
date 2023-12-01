use std::fs;

use aho_corasick::{AhoCorasick, MatchKind};

fn solution(input: &String) -> u64 {
	let ac_forward = AhoCorasick::builder()
		.match_kind(MatchKind::LeftmostFirst)
		.build(&[
			"1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
			"seven", "8", "eight", "9", "nine",
		])
		.unwrap();
	let ac_backward = AhoCorasick::builder()
		.match_kind(MatchKind::LeftmostFirst)
		.build(&[
			"1", "eno", "2", "owt", "3", "eerht", "4", "ruof", "5", "evif", "6", "xis", "7",
			"neves", "8", "thgie", "9", "enin",
		])
		.unwrap();
	let mut sum = 0;
	for line in input.lines() {
		let first = (ac_forward.find(line).unwrap().pattern().as_u64()) / 2 + 1;
		let reversed_line = line.chars().rev().collect::<String>();
		let last = (ac_backward
			.find(reversed_line.as_str())
			.unwrap()
			.pattern()
			.as_u64() / 2)
			+ 1;
		sum += first * 10 + last;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
		);
		let answer = solution(&input);
		assert_eq!(answer, 142);
	}

	#[test]
	fn test_solution_p2() {
		let input = String::from(
			r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
		);
		let answer = solution(&input);
		assert_eq!(answer, 281);
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
