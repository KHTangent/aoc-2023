use std::fs;

fn solution(input: &String) -> i32 {
	let mut line_it = input.lines();
	let (_, seeds_line) = line_it.next().unwrap().split_once(": ").unwrap();
	let mut current_ranges: Vec<Range> = seeds_line
		.split(" ")
		.map(|s| Range {
			from: s.parse().unwrap(),
			numbers: 1,
		})
		.collect();
	let mut maps: Vec<Vec<(u32,Range)>> = vec![];
	let mut temp: Vec<(u32, Range)> = vec![];
	for line in line_it {
		if line.is_empty() {
			continue;
		} else if !line.chars().nth(0).unwrap().is_digit(10) {
			if temp.is_empty() {
				continue;
			}
			maps.push(temp);
			temp = vec![];
		}
		let (i, range) = line.split_once(" ").unwrap();
		temp.push((i.parse().unwrap(), Range::from(range)));
	}
	for map in maps {
		for i in 0..current_ranges.len() {
			for (num, range) in &map {
				if current_ranges[i].has(*num) {
					current_ranges[i] = Range{
						from: range.from,
						numbers: range.numbers,
					};
					break;
				}
			}
		}
	}
	0
}

struct Range {
	pub from: u32,
	pub numbers: u32,
}

impl Range {
	pub fn from(line: &str) -> Range {
		let (from, numbers) = line.split_once(" ").unwrap();
		Range {
			from: from.parse::<u32>().unwrap(),
			numbers: numbers.parse::<u32>().unwrap(),
		}
	}

	pub fn has(&self, num: u32) -> bool {
		num >= self.from && num < self.from + self.numbers
	}

	pub fn overlaps(&self, other: &Range) -> bool {
		let end1 = self.from + self.numbers;
		let end2 = other.from + other.numbers;
		(self.from < end2 && end1 > other.from) ||
		(other.from < end1 && end2 > self.from)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_range_overlap() {
		let r1 = Range {
			from: 10,
			numbers: 5,
		};
		let r2 = Range {
			from: 5,
			numbers: 10,
		};
		let r3 = Range {
			from: 25,
			numbers: 10,
		};
		assert_eq!(r1.overlaps(&r2), true);
		assert_eq!(r2.overlaps(&r1), true);
		assert_eq!(r1.overlaps(&r3), false);
		assert_eq!(r3.overlaps(&r1), false);
	}

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
