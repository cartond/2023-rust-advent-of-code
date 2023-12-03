use std::cmp::max;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    println!("hello '{}'", input);
    let mut rolling_sum = 0;

    // cut "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    for input_line in input.lines() {
        let cube_amount = get_cube_amount_from_game(input_line);
        if cube_amount > 0 {
            rolling_sum += cube_amount;
        }
    }

    rolling_sum
}

// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
fn get_cube_amount_from_game(game: &str) -> i32 {
    let mut red_min = 0;
    let mut blue_min = 0;
    let mut green_min = 0;

    let mut game_itr = game.split(":");
    // 1. split on ";" and get number after "Game "
    let game_tag = game_itr.next().unwrap().trim();
    // game_tag = "Game 1"
    println!("game tag '{}'", game_tag);
    let game_id: i32 = game_tag
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .expect("Not a valid number");
    println!("game id '{}'", game_id);

    // split on ":" and get the second half
    let bag = game_itr.last().unwrap().trim();
    // bag = 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    println!("bag '{}'", bag);

    for round in bag.split(';') {
        // round = 1 red, 2 green, 6 blue
        println!("round '{}'", round);

        for count_color in round.trim().split(", ") {
            // count_color = '3 blue' then '_ _3 blue'
            println!("count_color '{}'", count_color.trim());

            let mut iter = count_color.trim().split(' ');
            let count: i32 = iter
                .next()
                .unwrap_or("9999999")
                .parse()
                .expect("Not a valid number");
            println!("count '{}'", count);

            let color = iter.next().unwrap_or("NO COLOR");
            println!("color '{}'", color);

            // could early break on these if any count surpasseses its min
            if color == "red" {
                red_min = max(red_min, count);
            }
            if color == "blue" {
                blue_min = max(blue_min, count);
            }
            if color == "green" {
                green_min = max(green_min, count);
            }
            println!(
                "red_min '{}', blue_min '{}', green_min '{}'",
                red_min, blue_min, green_min
            );
        }
        // if min hit, break
        // if no min passed, add id into sum
        // else continue
    }
    // if no min passed, add id into sum
    // 2. check if each section is valid

    // 3. return true if all sections are valid
	println!(
		"final: 	ed_min '{}', blue_min '{}', green_min '{}'",
		red_min, blue_min, green_min
	);
    return red_min * green_min * blue_min;
}

#[cfg(test)]
mod tests {
    // pull functions above in so we can use it in our tests
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
