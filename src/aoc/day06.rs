// https://adventofcode.com/2023/day/6

use super::utils::get_lines;

#[derive(Debug, Default)]
struct Races {
    races: Vec<(u64, u64)>,
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
    let times: Vec<u64> = times_str
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    let distances_str = &lines[1].rsplit_once(':').unwrap().1.trim();
    let distances: Vec<u64> = distances_str
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();
    races.races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| (time, distance))
        .collect();
    races
}

fn get_multiple_beaten_records(input_file: &str) -> u64 {
    let mut beaten_records: u64 = 1;
    let input = parse_input(input_file);
    for race in input.races.races {
        beaten_records *= get_beaten_records(race);
    }
    beaten_records
}

fn get_beaten_records((time, distance): (u64, u64)) -> u64 {
    let mut beaten_records: u64 = 0;
    for t in 0..time {
        let dist = t * (time - t);
        if dist > distance {
            beaten_records += 1;
        }
    }
    beaten_records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_multiple_beaten_records_test01() {
        assert_eq!(288, get_multiple_beaten_records("input/day06_test01.txt"));
    }

    #[test]
    fn test_get_multiple_beaten_records_test02() {
        assert_eq!(4, get_multiple_beaten_records("input/day06_test02.txt"));
    }
    
    #[test]
    fn test_get_multiple_beaten_records_test03() {
        assert_eq!(8, get_multiple_beaten_records("input/day06_test03.txt"));
    }
    
    #[test]
    fn test_get_multiple_beaten_records_test04() {
        assert_eq!(9, get_multiple_beaten_records("input/day06_test04.txt"));
    }

    #[test]
    fn test_get_multiple_beaten_records_part01() {
        assert_eq!(140220, get_multiple_beaten_records("input/day06_part01.txt"));
    }

    #[test]
    fn test_get_multiple_beaten_records_part02() {
        assert_eq!(39570185, get_multiple_beaten_records("input/day06_part02.txt"));
    }
}
