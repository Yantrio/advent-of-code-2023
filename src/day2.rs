#[derive(Debug, PartialEq)]
pub enum RGB {
    Red,
    Green,
    Blue,
}

pub struct BallCount {
    pub amount: u32,
    pub color: RGB,
}

impl From<&str> for BallCount {
    fn from(s: &str) -> Self {
        let (amount_string, colour) = s.split_once(' ').unwrap();
        let amount = amount_string.parse::<u32>().unwrap();
        let color = match colour {
            "red" => RGB::Red,
            "green" => RGB::Green,
            "blue" => RGB::Blue,
            _ => panic!("Invalid color"),
        };
        BallCount { amount, color }
    }
}

pub struct Game {
    pub game_number: u32,
    pub rounds: Vec<Vec<BallCount>>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (gnum, rounds_text) = s
            .trim()
            .trim_start_matches("Game ")
            .split_once(":")
            .unwrap();

        let game_number = gnum.parse::<u32>().unwrap();

        let rounds = rounds_text
            .split(';')
            .map(|round| {
                round
                    .split(',')
                    .map(|ball_count| BallCount::from(ball_count.trim()))
                    .collect()
            })
            .collect();

        Game {
            game_number,
            rounds,
        }
    }
}

impl Game {
    fn is_possible_within_bounds(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.rounds.iter().all(|round| {
            round.iter().all(|ball_count| match ball_count.color {
                RGB::Red => max_red >= ball_count.amount,
                RGB::Green => max_green >= ball_count.amount,
                RGB::Blue => max_blue >= ball_count.amount,
            })
        })
    }

    fn get_fewest_possible_cubes(&self) -> (u32, u32, u32) {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for round in &self.rounds {
            for ball_count in round {
                match ball_count.color {
                    RGB::Red => red_count = red_count.max(ball_count.amount),
                    RGB::Green => green_count = green_count.max(ball_count.amount),
                    RGB::Blue => blue_count = blue_count.max(ball_count.amount),
                }
            }
        }

        (red_count, green_count, blue_count)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .split('\n')
        .map(|line| Game::from(line))
        .collect::<Vec<Game>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .filter(|game| game.is_possible_within_bounds(12, 13, 14))
        .map(|game| game.game_number)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| game.get_fewest_possible_cubes())
        .map(|(r, g, b)| r * g * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_game_from_str() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.game_number, 1);
        assert_eq!(game.rounds.len(), 3);
        assert_eq!(game.rounds[0].len(), 2);
        assert_eq!(game.rounds[0][0].amount, 3);
        assert_eq!(game.rounds[0][0].color, RGB::Blue);
    }

    #[test]
    fn test_input_generator() {
        let input = input_generator(SAMPLE_INPUT);
        assert_eq!(input.len(), 5);

        assert_eq!(input[0].game_number, 1);
        assert_eq!(input[0].rounds.len(), 3);
        assert_eq!(input[0].rounds[0].len(), 2);
        assert_eq!(input[0].rounds[0][0].amount, 3);
        assert_eq!(input[0].rounds[0][0].color, RGB::Blue);
    }

    #[test]
    fn example_part1() {
        let input = input_generator(SAMPLE_INPUT);
        assert_eq!(solve_part1(&input), 8);
    }

    #[test]
    fn test_is_possible_within_bounds() {
        let input = input_generator(SAMPLE_INPUT);
        assert_eq!(input[0].is_possible_within_bounds(12, 13, 14), true);
        assert_eq!(input[1].is_possible_within_bounds(12, 13, 14), true);
        assert_eq!(input[2].is_possible_within_bounds(12, 13, 14), false);
        assert_eq!(input[3].is_possible_within_bounds(11, 13, 14), false);
        assert_eq!(input[4].is_possible_within_bounds(12, 13, 14), true);
    }

    #[test]
    fn test_get_fewest_possible_cubes() {
        let input = input_generator(SAMPLE_INPUT);
        assert_eq!(input[0].get_fewest_possible_cubes(), (4, 2, 6));
        assert_eq!(input[1].get_fewest_possible_cubes(), (1, 3, 4));
        assert_eq!(input[2].get_fewest_possible_cubes(), (20, 13, 6));
        assert_eq!(input[3].get_fewest_possible_cubes(), (14, 3, 15));
        assert_eq!(input[4].get_fewest_possible_cubes(), (6, 3, 2));
    }

    #[test]
    fn example_part2() {
        let input = input_generator(SAMPLE_INPUT);
        assert_eq!(solve_part2(&input), 2286);
    }
}
