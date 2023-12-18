use lazy_static::lazy_static;
use pcre2::bytes::Regex;
use std::str::Lines;

pub fn run(lines: &Lines) -> u16 {
    // The test input contains numbers written as text: one, two, three, etc. up to nine.
    // We need to convert them to numbers.

    // The input contains numbers written as text: one, two, three, etc. up to nine.

    (*lines)
        .clone()
        .map(|line| {
            let (first, last) = extract(line);
            let first_digit = convert(first);
            let last_digit = convert(last);
            // concatenate first and last digit
            format!("{}{}", first_digit, last_digit)
                .parse::<u16>()
                .unwrap()
        })
        .sum()
}

fn extract(input: &str) -> (&str, &str) {
    const PATTERN: &str = "[0-9]|one|two|three|four|five|six|seven|eight|nine";

    lazy_static! {
        static ref TWO_MATCHES_R: Regex =
            Regex::new(format!(r"(?P<first>{p}).*(?P<last>{p})", p = PATTERN).as_str()).unwrap();
    }

    if let Some(caps) = TWO_MATCHES_R.captures(input.as_bytes()).unwrap() {
        let first = caps.name("first").unwrap().as_bytes();
        let last = caps.name("last").unwrap().as_bytes();
        return (
            std::str::from_utf8(first).unwrap(),
            std::str::from_utf8(last).unwrap(),
        );
    }

    lazy_static! {
        static ref ONE_MATCH_R: Regex =
            Regex::new(format!(r"(?P<first>{})", PATTERN).as_str()).unwrap();
    }

    if let Some(caps) = ONE_MATCH_R.captures(input.as_bytes()).unwrap() {
        let first = caps.name("first").unwrap().as_bytes();
        let first_str = std::str::from_utf8(first).unwrap();
        return (first_str, first_str);
    }

    panic!("No match for input: {}", input)
}

fn convert(input: &str) -> u16 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("Invalid conversion input: {}", input)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn minimal_input_test() {
        let result = run(&TEST_INPUT.lines());
        assert_eq!(result, 281);
    }
}
