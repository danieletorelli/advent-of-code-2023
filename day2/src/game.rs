use lazy_static::lazy_static;
use pcre2::bytes::Regex;
use utils::extract_group;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GameId(pub u16);
#[derive(Debug)]
pub struct GameSet {
    pub red_cubes: u16,
    pub blue_cubes: u16,
    pub green_cubes: u16,
}
impl GameSet {
    fn new(red_cubes: u16, blue_cubes: u16, green_cubes: u16) -> Self {
        Self {
            red_cubes,
            blue_cubes,
            green_cubes,
        }
    }

    fn empty() -> Self {
        Self::new(0, 0, 0)
    }
}
#[derive(Debug)]
pub struct Game {
    pub id: GameId,
    sets: Vec<GameSet>,
}
impl Game {
    pub fn parse(input: &str) -> Result<Self, String> {
        fn parse_cube(i: &str) -> GameSet {
            i.split(',').fold(GameSet::empty(), |acc: GameSet, s| {
                let (amount, color) = s.trim().split_once(' ').unwrap();
                let amount = amount.parse::<u16>().unwrap();
                match color {
                    "red" => GameSet::new(acc.red_cubes + amount, acc.blue_cubes, acc.green_cubes),
                    "blue" => GameSet::new(acc.red_cubes, acc.blue_cubes + amount, acc.green_cubes),
                    "green" => {
                        GameSet::new(acc.red_cubes, acc.blue_cubes, acc.green_cubes + amount)
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            })
        }
        fn parse_set(input: &str) -> Vec<GameSet> {
            input.split(';').map(parse_cube).collect()
        }

        lazy_static! {
            static ref GAME_PATTERN: Regex =
                Regex::new(r#"^Game (?P<game_id>\d+): (?P<sets>.+)$"#).unwrap();
        }
        let game_id: Option<GameId> = extract_group(input, &GAME_PATTERN, "game_id")
            .map(|s| GameId(s.parse::<u16>().unwrap()));
        let sets: Vec<GameSet> = extract_group(input, &GAME_PATTERN, "sets")
            .map(parse_set)
            .unwrap_or_default();
        if let Some(game_id) = game_id {
            if !sets.is_empty() {
                Ok(Game { id: game_id, sets })
            } else {
                Err(format!("No game sets found in input: {}", input))
            }
        } else {
            Err(format!("No game id found in input: {}", input))
        }
    }

    // games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
    pub fn is_possible(&self) -> bool {
        self.sets
            .iter()
            .all(|set| set.blue_cubes <= 14 && set.green_cubes <= 13 && set.red_cubes <= 12)
    }

    pub fn min_set_possible(&self) -> GameSet {
        self.sets.iter().fold(GameSet::empty(), |acc, set| GameSet {
            blue_cubes: acc.blue_cubes.max(set.blue_cubes),
            green_cubes: acc.green_cubes.max(set.green_cubes),
            red_cubes: acc.red_cubes.max(set.red_cubes),
        })
    }
}
