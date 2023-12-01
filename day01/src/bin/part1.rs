/*
	1. put the example data in the test input (`let result = part1(....`)
	2. put the example answer in the tests assert
	3. do your work in fn part1
	4. run `cargo test` to test
	5. when satisfied, put the puzzle input data into input1.txt
	6. run `cargo run --bin part1` to get the answer, submit.
	7. copy all the code into part2.rs
	8. ...
*/

fn main() {
	let input = include_str!("./input1.txt");
	let output = part1(input);
	dbg!(output);
}

fn part1(input: &str) -> i32{
	let mut rolling_sum = 0;
	// for each line
	// get left most number (early break)
	// get right most number (early break)
	// combine to make XY
	// print and add to running sum
	for input_line in input.lines() {
		println!("{}", input_line);
		let left_dig = get_digit(input_line, false);
		let right_dig = get_digit(input_line, true);
		let ans = format!("{}{}", left_dig, right_dig);
		let number: i32 = ans.to_string().parse().expect("Not a valid number");
		println!("made {}{} should == {} should == {}", left_dig, right_dig, ans, number);
		rolling_sum = rolling_sum +  number
	}
	rolling_sum
}

fn get_digit(input: &str, reverse: bool) -> char {
	if reverse {
		for c in input.chars().rev() {
			// c now iterates over the characters in reverse order
			if c.is_numeric() {
				// print!("c: {}", c);
				// print!("c...: {}", c.to_string().parse::<i32>().unwrap());
				return c
			}
		}
	}else {
		for c in input.chars() {
			if c.is_numeric() {
				// print!("c: {}", c);
				// print!("c...: {}", c.to_string().parse::<i32>().unwrap());
				return c
			}
		}
	}
	panic!("[err]~~~:  No int found? reverse: {} with input: {}", reverse, input)
}

#[cfg(test)]
mod tests {
	// pull functions above in so we can use it in our tests
	use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}