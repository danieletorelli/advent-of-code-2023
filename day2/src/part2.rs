use crate::game::Game;
use std::str::Lines;

pub fn run(lines: &Lines) -> u32 {
    (*lines)
        .clone()
        .map(|l| l.trim())
        .filter(|l| !(*l).is_empty())
        .flat_map(Game::parse)
        .map(|g| g.min_set_possible())
        .map(|set| set.blue_cubes as u32 * set.green_cubes as u32 * set.red_cubes as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn minimal_input_test() {
        let lines = TEST_INPUT.lines();
        assert_eq!(run(&lines), 2286);
    }
}
