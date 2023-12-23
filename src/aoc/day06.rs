// https://adventofcode.com/2023/day/6

use super::utils::get_lines;

#[derive(Debug, Default)]
struct Races {
    races: Vec<(u32, u32)>,
}

#[derive(Debug)]
struct Input {
    races: Races,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let input = Input {
        races: parse_races(lines),
    };

    input
}

fn parse_races(lines: Vec<String>) -> Races {
    let mut races = Races::default();
    let times_str = &lines[0].rsplit_once(':').unwrap().1.trim();
    let times: Vec<u32> = times_str
        .split_whitespace()
        .map(|time| {
            time.parse().unwrap()
        })
        .collect();
    let distances_str = &lines[1].rsplit_once(':').unwrap().1.trim();
    let distances: Vec<u32> = distances_str
    .split_whitespace()
        .map(|time| {
            time.parse().unwrap()
        })
        .collect();
    races.races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| (time, distance))
        .collect();
    races
}

fn get_beaten_records(input_file: &str) -> u32 {
    let input = parse_input(input_file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_destination_test01() {
        assert_eq!(0, get_beaten_records("input/day06_test01.txt"));
    }

    #[test]
    fn test_get_destination_test02() {
        assert_eq!(0, get_beaten_records("input/day06.txt"));
    }
}
