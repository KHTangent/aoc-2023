use std::fs;

fn solution(input: &String, set: &ColorSet) -> u32 {
	let mut sum = 0;
	for line in input.lines() {
		let game = parse_game_line(line);
		if set_possible_in_game(&game, &set) {
			sum += game.game_number;
		}
	}
	sum
}

fn solution2(input: &String) -> u32 {
	let mut sum = 0;
	for line in input.lines() {
		let mut smallest_color_set = ColorSet {
			reds: 0,
			greens: 0,
			blues: 0,
		};
		let game = parse_game_line(line);
		for set in game.color_sets {
			if set.reds > smallest_color_set.reds {
				smallest_color_set.reds = set.reds;
			}
			if set.greens > smallest_color_set.greens {
				smallest_color_set.greens = set.greens;
			}
			if set.blues > smallest_color_set.blues {
				smallest_color_set.blues = set.blues;
			}
		}
		sum += smallest_color_set.reds * smallest_color_set.greens * smallest_color_set.blues;
	}
	sum
}

struct ColorSet {
	pub reds: u32,
	pub greens: u32,
	pub blues: u32,
}

struct GameLine {
	pub game_number: u32,
	pub color_sets: Vec<ColorSet>,
}

fn parse_color_set(input: &str) -> ColorSet {
	let mut out = ColorSet {
		reds: 0,
		greens: 0,
		blues: 0,
	};
	for tuple in input.split(", ") {
		let (num, color) = tuple.split_once(" ").unwrap();
		let num = num.parse::<u32>().unwrap();
		match color {
			"red" => out.reds += num,
			"green" => out.greens += num,
			"blue" => out.blues += num,
			&_ => (),
		}
	}
	out
}

fn parse_game_line(line: &str) -> GameLine {
	let (title, sets) = line.split_once(": ").unwrap();
	let (_, game_number) = title.split_once(" ").unwrap();
	GameLine {
		game_number: game_number.parse::<u32>().unwrap(),
		color_sets: sets.split("; ").map(parse_color_set).collect(),
	}
}

fn set_possible_in_game(game: &GameLine, set: &ColorSet) -> bool {
	for s in &game.color_sets {
		if s.reds > set.reds || s.greens > set.greens || s.blues > set.blues {
			return false;
		}
	}
	true
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
		);
		let answer = solution(
			&input,
			&ColorSet {
				reds: 12,
				greens: 13,
				blues: 14,
			},
		);
		assert_eq!(answer, 8);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 2286);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(
		&get_entire_input_file(),
		&ColorSet {
			reds: 12,
			greens: 13,
			blues: 14,
		},
	);
	println!("Answer task 1: {}", answer);
	let answer2 = solution2(&get_entire_input_file());
	println!("Answer task 2: {}", answer2);
}
