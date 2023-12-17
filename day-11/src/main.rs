use std::fs;

fn solution(input: &String, expand_factor: usize) -> i64 {
	let mut galaxy_number = 0;
	let mut grid: Vec<Vec<i64>> = vec![];
	for line in input.lines().map(|l| l.chars()) {
		grid.push(
			line.map(|s| {
				if s == '.' {
					0
				} else {
					galaxy_number += 1;
					galaxy_number
				}
			})
			.collect(),
		);
	}
	expand_grid(&mut grid, expand_factor);
	let galaxies = find_galaxies(&grid);
	let mut sum = 0;
	for i in 0..galaxies.len() {
		for j in i..galaxies.len() {
			sum += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
		}
	}
	sum
}

fn find_galaxies(grid: &Vec<Vec<i64>>) -> Vec<(i64, i64)> {
	let mut galaxies: Vec<(i64, i64)> = vec![];
	for i in 0..grid.len() {
		for j in 0..grid[i].len() {
			if grid[i][j] != 0 {
				galaxies.push((i as i64, j as i64));
			}
		}
	}
	galaxies
}

fn expand_grid(grid: &mut Vec<Vec<i64>>, factor: usize) {
	let mut line = 0;
	let line_length = grid[0].len();
	while line < grid.len() {
		if grid[line].iter().all(|&e| e == 0) {
			for _ in 0..factor {
				grid.insert(line, vec![0; line_length]);
			}
			line += factor;
		}
		line += 1;
	}
	let mut col = 0;
	let grid_height = grid.len();
	while col < grid[0].len() {
		if (0..grid_height).all(|i| grid[i][col] == 0) {
			for _ in 0..factor {
			for i in 0..grid_height {
				grid[i].insert(col, 0);
			}
			}
			col += factor;
		}
		col += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
		);
		let answer = solution(&input, 1);
		assert_eq!(answer, 374);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file(), 1);
	println!("Answer: {}", answer);
	let answer2 = solution(&get_entire_input_file(), 1000000);
	println!("Answer 2: {}", answer2);
}
