// https://adventofcode.com/2023/day/4

use std::collections::HashMap;

use super::utils::get_lines;

use regex::Regex;

#[derive(Debug, Default)]
struct Game {
    winning_nums: Vec<u32>,
    nums: Vec<u32>,
}

#[derive(Debug)]
struct Input {
    games: HashMap<u32, Game>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let input = Input {
        games: parse_games(lines),
    };

    input
}

fn parse_games(game_lines: Vec<String>) -> HashMap<u32, Game> {
    lazy_static! {
        static ref RE_GAME: Regex = Regex::new(r"\d+").unwrap();
    }

    let mut games: HashMap<u32, Game> = HashMap::new();

    for game_line in game_lines {
        let game_line_parts: Vec<&str> = game_line.split([':', '|']).collect();
        assert!(game_line_parts.len() == 3);

        let card_num: Vec<u32> = RE_GAME
            .find_iter(&game_line_parts[0])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let winning_nums: Vec<u32> = RE_GAME
            .find_iter(&game_line_parts[1])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let nums: Vec<u32> = RE_GAME
            .find_iter(&game_line_parts[2])
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        let game = Game {
            winning_nums: winning_nums,
            nums: nums,
        };

        games.insert(*card_num.first().unwrap(), game);
    }

    games
}

fn get_scratchcard_score(input_file: &str) -> u32 {
    let mut sum_scratchcard_points: u32 = 0;

    let input = parse_input(input_file);

    for (_, game) in input.games.into_iter() {
        let mut game_score: u32 = 0;
        for num in game.nums {
            if game.winning_nums.contains(&num) {
                if game_score == 0 {
                    game_score = 1;
                } else {
                    game_score *= 2;
                }
            }
        }
        sum_scratchcard_points += game_score;
    }

    sum_scratchcard_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scratchcard_score_test01() {
        assert_eq!(13, get_scratchcard_score("input/day04_test01.txt"));
    }

    #[test]
    fn test_get_scratchcard_score() {
        assert_eq!(20667, get_scratchcard_score("input/day04.txt"));
    }
}
