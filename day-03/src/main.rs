use std::fs;

type Grid = Vec<Vec<char>>;

fn solution(input: &String) -> i32 {
	let mut sum = 0;
	let grid = process_input(input);
	let row_length = grid[0].len();
	for y in 0..grid.len() {
		let mut x = 0;
		while x < row_length {
			if grid[y][x].is_digit(10) && has_symbol_around(&grid, x as i32, y as i32) {
				sum += get_number(&grid[y], x);
				while x < row_length - 1 && grid[y][x + 1].is_digit(10) {
					x += 1;
				}
			}
			x += 1;
		}
	}
	sum
}

fn solution2(input: &String) -> i32 {
	let mut sum = 0;
	let grid = process_input(input);
	let row_length = grid[0].len() as i32;
	for y in 0..(grid.len() as i32) {
		for x in 0..(grid[y as usize].len() as i32) {
			if grid[y as usize][x as usize] == '*' {
				let mut last_found_number = 0;
				let mut product = 1;
				let mut found_numbers = 0;
				for current_y in [y - 1, y, y + 1] {
					if current_y < 0 || current_y == grid.len() as i32 {
						continue;
					}
					for current_x in [x - 1, x, x + 1] {
						if current_x == x && current_y == y {
							continue;
						}
						if current_x < 0 || current_x == row_length {
							continue;
						}
						let found = get_number(&grid[current_y as usize], current_x as usize);
						if found > 0 && found != last_found_number {
							product *= found;
							last_found_number = found;
							found_numbers += 1;
						}
					}
				}
				if found_numbers == 2 {
					sum += product;
				}
			}
		}
	}
	sum
}

fn has_symbol_around(grid: &Grid, x: i32, y: i32) -> bool {
	let max_x = (grid.first().unwrap().len()) as i32;
	for current_y in [y - 1, y, y + 1] {
		if current_y < 0 || current_y == grid.len() as i32 {
			continue;
		}
		for current_x in [x - 1, x, x + 1] {
			if current_x < 0 || current_x == max_x {
				continue;
			}
			if is_symbol(grid[current_y as usize][current_x as usize]) {
				return true;
			}
		}
	}
	false
}

fn get_number(line: &[char], mut start_at: usize) -> i32 {
	if !line[start_at].is_digit(10) {
		return 0;
	}
	let number_end = start_at
		+ line
			.iter()
			.skip(start_at)
			.position(|c| c < &'0' || c > &'9')
			.unwrap_or(line.len() - start_at);
	while start_at > 0 && line[start_at - 1].is_digit(10) {
		start_at -= 1;
	}
	let mut sum = 0;
	for (pow, index) in (start_at..number_end).rev().enumerate() {
		sum += (line[index].to_digit(10).unwrap() as i32) * (10 as i32).pow(pow as u32);
	}
	sum
}

fn is_symbol(c: char) -> bool {
	c != '.' && !c.is_digit(10)
}

fn process_input(input: &String) -> Grid {
	input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_has_symbol_around() {
		let grid: Grid = vec![
			vec!['.', '.', '@', '.'],
			vec!['.', '.', '.', '.'],
			vec!['.', '.', '.', '.'],
			vec!['!', '.', '.', '.'],
			vec!['.', '.', '.', '.'],
		];
		assert_eq!(has_symbol_around(&grid, 1, 0), true);
		assert_eq!(has_symbol_around(&grid, 0, 0), false);
		assert_eq!(has_symbol_around(&grid, 1, 3), true);
	}

	#[test]
	fn test_get_number() {
		let input = ['.', '.', '4', '2', '1', '.'];
		let input2 = ['.', '.', '4', '2', '1'];
		let input3 = ['4', '2', '1', '.'];
		assert_eq!(get_number(&input, 2), 421);
		assert_eq!(get_number(&input, 3), 421);
		assert_eq!(get_number(&input2, 2), 421);
		assert_eq!(get_number(&input3, 0), 421);
	}

	#[test]
	fn test_solution() {
		let input = String::from(
			r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
		);
		let answer = solution(&input);
		assert_eq!(answer, 4361);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 467835);
	}

	#[test]
	fn test_solution3() {
		let input = String::from(
			r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$1*....
.664.598..",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 467 * 35);
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
	let answer2 = solution2(&get_entire_input_file());
	println!("Answer 2: {}", answer2);
}
