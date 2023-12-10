// https://adventofcode.com/2023/day/2

use grid::Grid;

use super::utils::get_lines;

#[derive(Debug, Default)]
struct SchematicEntry {
    digit: Option<u32>,
    is_symbol: bool,
}

#[derive(Debug)]
struct Input {
    engine_schematic: Grid<SchematicEntry>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    let input = Input {
        engine_schematic: parse_engine_schematic(iter.next().unwrap().to_owned()),
    };

    input
}

fn parse_engine_schematic(engine_schematic_lines: Vec<String>) -> Grid<SchematicEntry> {
    let mut engine_schematic = Grid::new(0, 0);
    for engine_schematic_line in engine_schematic_lines.into_iter() {
        let mut engine_schematic_entries: Vec<SchematicEntry> = Vec::new();
        for engine_schematic_entry in engine_schematic_line.chars() {
            match engine_schematic_entry {
                '1'..='9' => engine_schematic_entries.push(SchematicEntry {
                    digit: engine_schematic_entry.to_digit(10),
                    is_symbol: false,
                }),
                '.' => engine_schematic_entries.push(SchematicEntry {
                    digit: None,
                    is_symbol: false,
                }),
                _ => engine_schematic_entries.push(SchematicEntry {
                    digit: None,
                    is_symbol: true,
                }),
            }
        }
        engine_schematic.push_row(engine_schematic_entries)
    }
    engine_schematic
}

fn get_sum_part_nums(input_file: &str) -> u32 {
    let input = parse_input(input_file);
    let sum_part_nums: u32 = 0;
    sum_part_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_part_nums_test01() {
        assert_eq!(0, get_sum_part_nums("input/day03_test01.txt"));
    }

    #[test]
    fn test_get_sum_part_nums() {
        assert_eq!(0, get_sum_part_nums("input/day03.txt"));
    }
}
