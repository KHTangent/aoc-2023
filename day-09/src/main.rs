use std::fs;

fn next_number(numbers: &Vec<i64>) -> i64 {
	let mut stack: Vec<Vec<i64>> = vec![numbers.clone()];
	while !stack.last().unwrap().iter().all(|&n| n == 0) {
		let lowest = stack.last().unwrap();
		stack.push(
			lowest
				.iter()
				.enumerate()
				.skip(1)
				.map(|(i, _)| lowest[i] - lowest[i - 1])
				.collect(),
		);
	}
	for i in (0..stack.len() - 1).rev() {
		let a = *stack[i].last().unwrap();
		let b = *stack[i + 1].last().unwrap();
		stack[i].push(a + b);
	}
	*stack.first().unwrap().last().unwrap()
}

fn solution(input: &String) -> i64 {
	let mut sum = 0;
	for line in input.lines() {
		let nums: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
		sum += next_number(&nums);
	}
	sum
}

fn solution2(input: &String) -> i64 {
	let mut sum = 0;
	for line in input.lines() {
		let mut nums: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
		nums.reverse();
		sum += next_number(&nums);
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_next() {
		let input = vec![0, 3, 6, 9, 12, 15];
		let answer = next_number(&input);
		assert_eq!(answer, 18);
	}

	#[test]
	fn test_solution() {
		let input = String::from(
			r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
		);
		let answer = solution(&input);
		assert_eq!(answer, 114);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(r"10 13 16 21 30 45");
		let answer = solution2(&input);
		assert_eq!(answer, 5);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file());
	let answer2 = solution2(&get_entire_input_file());
	println!("Answer: {}", answer);
	println!("Answer 2: {}", answer2);
}
