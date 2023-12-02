fn main() {
	let input = include_str!("./input1.txt");
	let output = part2(input);
	dbg!(output);
}

fn part2(input: &str) -> u32{
	let mut sum = 0;
	for input_line in input.lines() {
		println!("line:	{}", input_line);
		let num = process_input_line(input_line);
		println!("num:	{}", num);
		sum += num;
	}
	sum
}

fn process_input_line(input_line: &str) -> u32 {
    let mut it = (0..input_line.len()).filter_map(|index| {
        // example, "hione22...", index 2 is 'o'
        // so reduced_line would be "one22...."
        // so we chck if that starts with the text version of a number
        let reduced_line = &input_line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}


#[cfg(test)]
mod tests {
	// pull functions above in so we can use it in our tests
	use super::*;

    #[test]
    fn it_works() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
}