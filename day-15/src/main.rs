use std::fs;

fn solution(input: &String) -> i64 {
	input.split(',').fold(0, |i, s| {
		i + s
			.bytes()
			.filter(|&b| b >= 32 && b <= 126)
			.fold(0u8, |i: u8, c| i.wrapping_add(c).wrapping_mul(17)) as i64
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash() {
		let input = String::from(r"HASH");
		let answer = solution(&input);
		assert_eq!(answer, 52);
	}

	#[test]
	fn test_solution() {
		let input = String::from(r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
		let answer = solution(&input);
		assert_eq!(answer, 1320);
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
