fn solution(days: &Vec<(u64, u64)>) -> u64 {
	let mut product = 1;
	for &(race_time, record) in days {
		let mut races_won = 0;
		for i in 0..race_time {
			if race_distance(race_time, i) > record {
				races_won += 1;
			}
		}
		product *= races_won;
	}
	product
}

fn race_distance(time_race: u64, time_held: u64) -> u64 {
	(time_race - time_held) * time_held
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_race_distance() {
		assert_eq!(race_distance(7, 2), 10);
		assert_eq!(race_distance(7, 3), 12);
	}

	#[test]
	fn test_solution() {
		let input = vec![(7, 9), (15, 40), (30, 200)];
		let answer = solution(&input);
		assert_eq!(answer, 288);
	}
}

fn main() {
	let task1 = vec![(62, 644), (73, 1023), (75, 1240), (65, 1023)];
	let answer1 = solution(&task1);
	println!("Answer: {}", answer1);
	let task2 = vec![(62737565, 644102312401023)];
	let answer2 = solution(&task2);
	println!("Answer 2: {}", answer2);
}
