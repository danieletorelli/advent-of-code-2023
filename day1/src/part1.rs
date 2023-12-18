use std::str::Lines;

pub fn run(lines: &Lines) -> u16 {
    (*lines)
        .clone()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            // concatenate first and last digit
            format!("{}{}", first_digit, last_digit)
                .parse::<u16>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    #[test]
    fn minimal_input_test() {
        let result = run(&TEST_INPUT.lines());
        assert_eq!(result, 142);
    }
}
