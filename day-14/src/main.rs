use std::fs;

#[derive(Clone, PartialEq, Eq)]
enum TileType {
	Space,
	Roll,
	Square,
}

fn parse_grid(input: &String) -> Vec<Vec<TileType>> {
	let mut grid: Vec<Vec<TileType>> = vec![];
	for line in input.lines() {
		let mut l: Vec<TileType> = Vec::with_capacity(line.len());
		for char in line.chars() {
			l.push(match char {
				'O' => TileType::Roll,
				'#' => TileType::Square,
				_ => TileType::Space,
			});
		}
		grid.push(l);
	}
	grid
}

fn count_grid(grid: &Vec<Vec<TileType>>) -> i64 {
	let mut sum = 0;
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] == TileType::Roll {
				sum += grid.len() - y;
			}
		}
	}
	sum as i64
}

fn solution(input: &String) -> i64 {
	let mut grid = parse_grid(input);
	let mut changed = true;
	while changed {
		changed = false;
		for y in 1..grid.len() {
			for x in 0..grid[y].len() {
				if grid[y][x] == TileType::Roll && grid[y - 1][x] == TileType::Space {
					grid[y][x] = TileType::Space;
					grid[y - 1][x] = TileType::Roll;
					changed = true;
				}
			}
		}
	}
	count_grid(&grid)
}

fn solution2(input: &String, cycles: i64) -> i64 {
	let mut grid = parse_grid(input);
	let line_length = grid.first().unwrap().len();
	for _ in 0..cycles {
		let mut changed = true;
		while changed {
			changed = false;
			for y in 1..grid.len() {
				for x in 0..line_length {
					if grid[y][x] == TileType::Roll && grid[y - 1][x] == TileType::Space {
						grid[y][x] = TileType::Space;
						grid[y - 1][x] = TileType::Roll;
						changed = true;
					}
				}
			}
		}
		changed = true;
		while changed {
			changed = false;
			for y in 0..grid.len() {
				for x in 1..line_length {
					if grid[y][x] == TileType::Roll && grid[y][x - 1] == TileType::Space {
						grid[y][x] = TileType::Space;
						grid[y][x - 1] = TileType::Roll;
						changed = true;
					}
				}
			}
		}
		changed = true;
		while changed {
			changed = false;
			for y in 0..grid.len() - 1 {
				for x in 0..line_length {
					if grid[y][x] == TileType::Roll && grid[y + 1][x] == TileType::Space {
						grid[y][x] = TileType::Space;
						grid[y + 1][x] = TileType::Roll;
						changed = true;
					}
				}
			}
		}
		changed = true;
		while changed {
			changed = false;
			for y in 0..grid.len() {
				for x in 0..line_length - 1 {
					if grid[y][x] == TileType::Roll && grid[y][x + 1] == TileType::Space {
						grid[y][x] = TileType::Space;
						grid[y][x + 1] = TileType::Roll;
						changed = true;
					}
				}
			}
		}
	}
	count_grid(&grid)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
		);
		let answer = solution(&input);
		assert_eq!(answer, 136);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
		);
		let answer = solution2(&input, 3);
		assert_eq!(answer, 69);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file());
	println!("Answer 1: {}", answer);
	let answer2 = solution2(&get_entire_input_file(), 1000000000);
	println!("Answer 2: {}", answer2);
}
