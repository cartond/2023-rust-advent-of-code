/*
	1. put the example data in the test input (`let result = part1(....`)
	2. put the example answer in the tests assert
	3. do your work in fn part1
	4. run `cargo test` to test
	5. when satisfied, put the puzzle input data into input1.txt
	6. run `cargo run --bin part1` to get the answer, submit.
	7. copy all the code into part2.rs
	8. ...

	run this by running day02 part1
	`cargo run --bin part1`

*/

use std::cmp::max;

fn main() {
	let input = include_str!("./input1.txt");
	let output = part1(input);
	dbg!(output);
}

fn part1(input: &str) -> i32{
	println!("hello '{}'", input);
	let mut rolling_sum = 0;

	// cut "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
	for input_line in input.lines() {
		let valid_game = check_game(input_line);
		if valid_game > 0 {
			rolling_sum += valid_game;
		}
	}

	rolling_sum
}

// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
fn check_game(game: &str) -> i32 {
	let mut red_max = 0;
	let mut blue_max = 0;
	let mut green_max = 0;

	let mut game_itr = game.split(":");
	// 1. split on ";" and get number after "Game "
	let game_tag = game_itr.next().unwrap().trim();
	// game_tag = "Game 1"
	println!("game tag '{}'", game_tag);
	let game_id: i32 = game_tag.split(" ").last().unwrap().parse().expect("Not a valid number");
	println!("game id '{}'", game_id);

	// split on ":" and get the second half
	let bag = game_itr.last().unwrap().trim();
	// bag = 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
	println!("bag '{}'", bag);
	
	for round in bag.split(';'){
		// round = 1 red, 2 green, 6 blue
		println!("round '{}'", round);

		for count_color in round.trim().split(", ") {
			// count_color = '3 blue' then '_ _3 blue'
			println!("count_color '{}'", count_color.trim());

			let mut iter = count_color.trim().split(' ');
			let count: i32 = iter.next().unwrap_or("9999999").parse().expect("Not a valid number");
			println!("count '{}'", count);
			
			let color = iter.next().unwrap_or("NO COLOR");
			println!("color '{}'", color);

			// could early break on these if any count surpasseses its max
			if color == "red" {
				red_max = max(red_max, count);
			}
			if color == "blue" {
				blue_max = max(blue_max, count);
			}
			if color == "green" {
				green_max = max(green_max, count);
			}
			println!("red_max '{}', blue_max '{}', green_max '{}'", red_max, blue_max, green_max);
		}
		// if max hit, break
		// if no max passed, add id into sum
		// else continue
	}
	// if no max passed, add id into sum
	// 2. check if each section is valid

	// 3. return true if all sections are valid
	
	//  only 12 red cubes, 13 green cubes, and 14 blue cubes?
	if red_max <= 12 && green_max <= 13 && blue_max <= 14{
		// return game_id;
		return game_id;
	}
	else {
		return -1;	
	}
}

#[cfg(test)]
mod tests {
	// pull functions above in so we can use it in our tests
	use super::*;

    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}