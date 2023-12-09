use std::fs;

struct GameLine {
	pub cards: [u8; 5],
	pub bid: u64,
}

fn solution(input: &String) -> u64 {
	let mut games: Vec<(u64, GameLine)> = vec![];
	for line in input.lines() {
		games.push((0, parse_game_line(&line)));
	}
	for i in 0..games.len() {
		games[i].0 = face_scores(&games[i].1) + pattern_score(&games[i].1);
	}
	games.sort_by_key(|g| g.0);
	let mut sum = 0;
	for i in 0..games.len() {
		sum += (i as u64 + 1) * games[i].1.bid;
	}
	sum
}

fn parse_game_line(line: &str) -> GameLine {
	let (cards, bid) = line.split_once(" ").unwrap();
	GameLine {
		cards: cards
			.chars()
			.map(|c| match c {
				'A' => 14,
				'K' => 13,
				'Q' => 12,
				'J' => 11,
				'T' => 10,
				'0'..='9' => (c as u8) - ('0' as u8),
				_ => panic!(),
			})
			.collect::<Vec<u8>>()
			.try_into()
			.unwrap(),
		bid: bid.parse().unwrap(),
	}
}

fn pattern_score(game: &GameLine) -> u64 {
	let mut game_clone = game.cards.clone();
	game_clone.sort();
	let (a, b, c, d, e) = game_clone.into();
	if a == b && b == c && c == d && d == e {
		return 6 << 20;
	} else if (a == b && b == c && c == d) || (b == c && c == d && d == e) {
		return 5 << 20;
	} else if (a == b && b == c && d == e) || (a == b && c == d && d == e) {
		return 4 << 20;
	} else if (a == b && b == c) || (b == c && c == d) || (c == d && d == e) {
		return 3 << 20;
	}
	let mut pairs = 0;
	if a == b {
		pairs += 1;
	}
	if b == c {
		pairs += 1;
	}
	if c == d {
		pairs += 1;
	}
	if d == e {
		pairs += 1;
	}
	match pairs {
		2 => 2 << 20,
		1 => 1 << 20,
		_ => 0,
	}
}

fn face_scores(game: &GameLine) -> u64 {
	let mut sum: u64 = 0;
	for i in 0..5u64 {
		sum += (game.cards[4 - (i as usize)] as u64) << (i * 4);
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
		);
		let answer = solution(&input);
		assert_eq!(answer, 6440);
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
