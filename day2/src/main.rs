mod game;
mod part1;
mod part2;

fn main() {
    let lines = include_str!("../input.txt").lines();

    let result1 = part1::run(&lines);
    let result2 = part2::run(&lines);

    println!("Part 1 result: {}", result1);
    println!("Part 2 result: {}", result2);

    assert_eq!(result1, 1931);
    assert_eq!(result2, 83105);
}
