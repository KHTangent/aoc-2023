use std::fs;

fn solution(input: &String) -> i64 {
	let mut line_it = input.lines();
	let (_, seeds_line) = line_it.next().unwrap().split_once(": ").unwrap();
	let mut range_sets: Vec<Vec<Range>> = vec![];
	let mut temp: Vec<Range> = vec![];
	for line in line_it {
		if line.is_empty() {
			continue;
		} else if !line.chars().nth(0).unwrap().is_digit(10) {
			if temp.is_empty() {
				continue;
			}
			range_sets.push(temp);
			temp = vec![];
			continue;
		}
		temp.push(Range::from(line));
	}
	seeds_line
		.split(" ")
		.map(|s| s.parse::<i64>().unwrap())
		.map(|mut seed| {
			range_sets.iter().for_each(|ranges| {
				if let Some(range) = ranges.iter().find(|range| range.applies_to(seed)) {
					seed = range.apply(seed);
				}
			});
			seed
		})
		.fold(i64::MAX, |acc, n| acc.min(n))
}

#[derive(Clone)]
struct Range {
	pub from: i64,
	pub to: i64,
	pub length: i64,
}

impl Range {
	pub fn from(line: &str) -> Range {
		let mut line_parts = line.split(" ");
		Range {
			to: line_parts.next().unwrap().parse().unwrap(),
			from: line_parts.next().unwrap().parse().unwrap(),
			length: line_parts.next().unwrap().parse().unwrap(),
		}
	}

	pub fn applies_to(&self, num: i64) -> bool {
		num >= self.from && num < self.from + self.length
	}

	pub fn apply(&self, num: i64) -> i64 {
		num + (self.to - self.from)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
		);
		let answer = solution(&input);
		assert_eq!(answer, 35);
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
