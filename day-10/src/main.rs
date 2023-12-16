use std::fs;

#[derive(Clone, PartialEq, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
	None,
}

type Pipe = (Direction, Direction);

fn map_char(c: char) -> Pipe {
	match c {
		'|' => (Direction::Up, Direction::Down),
		'-' => (Direction::Left, Direction::Right),
		'7' => (Direction::Left, Direction::Down),
		'F' => (Direction::Right, Direction::Down),
		'J' => (Direction::Left, Direction::Up),
		'L' => (Direction::Right, Direction::Up),
		_ => (Direction::None, Direction::None),
	}
}

fn opposite(d: &Direction) -> Direction {
	match &d {
		Direction::Up => Direction::Down,
		Direction::Down => Direction::Up,
		Direction::Left => Direction::Right,
		Direction::Right => Direction::Left,
		Direction::None => panic!(),
	}
}

fn solution(input: &String, initial: Direction) -> i64 {
	let mut grid: Vec<Vec<Pipe>> = vec![];
	let mut start_pos = (0, 0);
	for line in input.lines() {
		if let Some(x) = line.find("S") {
			start_pos = (grid.len(), x);
		}
		grid.push(line.chars().map(map_char).collect());
	}
	let mut current_pos = start_pos.clone();
	let mut current_direction = initial;
	let mut steps = 0;
	loop {
		match current_direction {
			Direction::Up => current_pos.0 -= 1,
			Direction::Down => current_pos.0 += 1,
			Direction::Left => current_pos.1 -= 1,
			Direction::Right => current_pos.1 += 1,
			Direction::None => panic!(),
		}
		if current_pos == start_pos {
			break;
		}
		let pipe = grid[current_pos.0][current_pos.1].clone();
		current_direction = if current_direction == opposite(&pipe.0) {
			pipe.1
		} else {
			pipe.0
		};
		steps += 1;
	}
	(steps+1) / 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
		);
		let answer = solution(&input, Direction::Right);
		assert_eq!(answer, 8);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file(), Direction::Down);
	println!("Answer: {}", answer);
}
