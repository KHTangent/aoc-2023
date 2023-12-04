use std::fs;

fn winning_numbers_on_line(line: &str) -> u32 {
	let (_, numbers) = line.split_once(": ").unwrap();
	let (numbers_won, numbers_have) = numbers.split_once(" | ").unwrap();
	let numbers_winning: Vec<u32> = numbers_won
		.split(" ")
		.filter(|s| !s.is_empty())
		.map(|n| n.trim().parse::<u32>().unwrap())
		.collect();
	numbers_have
		.split(" ")
		.filter(|s| !s.is_empty())
		.map(|n| n.trim().parse::<u32>().unwrap())
		.filter(|n| numbers_winning.contains(n))
		.count() as u32
}

fn solution(input: &String) -> u32 {
	let mut sum = 0;
	for line in input.lines() {
		let winning_numbers_on_ticket = winning_numbers_on_line(line);
		if winning_numbers_on_ticket > 0 {
			sum += (2u32).pow(winning_numbers_on_ticket - 1);
		}
	}
	sum
}

fn solution2(input: &String) -> u32 {
	let mut sum = 0;
	let mut copies_to_add: Vec<u32> = vec![];
	for line in input.lines() {
		let won_this_line = winning_numbers_on_line(line);
		let multiplier = if copies_to_add.len() == 0 {
			1
		} else {
			copies_to_add.remove(0) + 1
		};
		for _ in 0..multiplier {
			for i in 0..(won_this_line as usize) {
				if copies_to_add.len() == i {
					copies_to_add.push(1);
				} else {
					copies_to_add[i] += 1;
				}
			}
			sum += 1;
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
		);
		let answer = solution(&input);
		assert_eq!(answer, 13);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 30);
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
	let answer = solution2(&get_entire_input_file());
	println!("Answer 2: {}", answer);
}
