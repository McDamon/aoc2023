// https://adventofcode.com/2023/day/10

use grid::Grid;

use super::utils::get_lines;

#[derive(Debug, Default)]
#[repr(u8)]
enum Pipe {
    #[default]
    Ground = b'.',
    VerticalNS = b'|',
    HorizontalEW = b'-',
    NE90Deg = b'L',
    NW90Deg = b'J',
    SW90Deg = b'7',
    SE90Deg = b'F',
    StartPos = b'S'
}

impl TryFrom<u8> for Pipe {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Pipe::Ground as u8 => Ok(Pipe::Ground),
            x if x == Pipe::VerticalNS as u8 => Ok(Pipe::VerticalNS),
            x if x == Pipe::HorizontalEW as u8 => Ok(Pipe::HorizontalEW),
            x if x == Pipe::NE90Deg as u8 => Ok(Pipe::NE90Deg),
            x if x == Pipe::NW90Deg as u8 => Ok(Pipe::NW90Deg),
            x if x == Pipe::SW90Deg as u8 => Ok(Pipe::SW90Deg),
            x if x == Pipe::SE90Deg as u8 => Ok(Pipe::SE90Deg),
            x if x == Pipe::StartPos as u8 => Ok(Pipe::StartPos),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Input {
    tiles: Grid<Pipe>
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        tiles: parse_tiles(iter.next().unwrap().to_owned()),
    }
}

fn parse_tiles(
    tiles_lines: Vec<String>
) -> Grid<Pipe> {
    let mut tiles = Grid::new(0, 0);
    for tiles_line in tiles_lines.into_iter() {
        let mut tiles_entries: Vec<Pipe> = Vec::new();
        for tiles_entry in tiles_line.chars() {
            match Pipe::try_from(tiles_entry as u8) {
                Ok(pipe) => tiles_entries.push(pipe),
                Err(_) => panic!("Invalid pipe"),
            }
        }
        tiles.push_row(tiles_entries)
    }
    tiles
}

fn get_farthest_steps(input_file: &str) -> u32 {
    let input = parse_input(input_file);
    
    for ((row, col), entry) in input.tiles.indexed_iter() {
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_farthest_steps_test01() {
        assert_eq!(4, get_farthest_steps("input/day10_test01.txt"));
    }

    #[test]
    fn test_get_farthest_steps_test02() {
        assert_eq!(8, get_farthest_steps("input/day10_test02.txt"));
    }

    #[test]
    fn test_get_farthest_steps() {
        assert_eq!(0, get_farthest_steps("input/day10.txt"));
    }
}