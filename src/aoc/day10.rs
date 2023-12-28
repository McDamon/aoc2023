// https://adventofcode.com/2023/day/10

use grid::Grid;
use indextree::{Arena, NodeId};

use super::utils::get_lines;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
enum Pipe {
    #[default]
    Ground = b'.',
    VerticalNS = b'|',
    HorizontalEW = b'-',
    NE90DegLSym = b'L',
    NW90DegJSym = b'J',
    SW90Deg7Sym = b'7',
    SE90DegFSym = b'F',
    StartPos = b'S',
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl TryFrom<u8> for Pipe {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Pipe::Ground as u8 => Ok(Pipe::Ground),
            x if x == Pipe::VerticalNS as u8 => Ok(Pipe::VerticalNS),
            x if x == Pipe::HorizontalEW as u8 => Ok(Pipe::HorizontalEW),
            x if x == Pipe::NE90DegLSym as u8 => Ok(Pipe::NE90DegLSym),
            x if x == Pipe::NW90DegJSym as u8 => Ok(Pipe::NW90DegJSym),
            x if x == Pipe::SW90Deg7Sym as u8 => Ok(Pipe::SW90Deg7Sym),
            x if x == Pipe::SE90DegFSym as u8 => Ok(Pipe::SE90DegFSym),
            x if x == Pipe::StartPos as u8 => Ok(Pipe::StartPos),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Input {
    tiles: Grid<Pipe>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        tiles: parse_tiles(iter.next().unwrap().to_owned()),
    }
}

fn parse_tiles(tiles_lines: Vec<String>) -> Grid<Pipe> {
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

#[derive(Debug, Copy, Clone)]
struct Entry {
    pipe: Pipe,
    pos: (usize, usize),
}

fn get_farthest_steps(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for ((row, col), pipe) in input.tiles.indexed_iter() {
        if *pipe == Pipe::StartPos {
            start_pos = Some((row, col));
        }
    }

    if let Some((row, col)) = start_pos {
        let mut arena: Arena<Entry> = Arena::new();
        let root_node = arena.new_node(Entry {
            pipe: Pipe::StartPos,
            pos: (row, col),
        });

        build_tree(&input.tiles, &mut arena, None, root_node);

        let traverser = root_node.traverse(&arena);
        let mut node_ids: Vec<NodeId> = vec![];
        for ev in traverser {
            match ev {
                indextree::NodeEdge::Start(id) => node_ids.push(id),
                _ => break
            };
        };
        let res = node_ids.len() / 2;
        println!("{}", res);
        res
    } else {
        panic!("Invalid start node");
    }
}

fn build_tree(
    tiles: &Grid<Pipe>,
    arena: &mut Arena<Entry>,
    maybe_prev_index: Option<NodeId>,
    current_index: NodeId,
) {
    let maybe_node = arena.get_mut(current_index);
    if let Some(current_node) = maybe_node {
        let current_path = current_node.get().pipe;
        let current_row: i64 = current_node.get().pos.0 as i64;
        let current_col: i64 = current_node.get().pos.1 as i64;
        // Loop through all the potential new nodes to check next direction
        for test_row in (current_row - 1)..(current_row + 2) {
            for test_col in (current_col - 1)..(current_col + 2) {
                let maybe_direction = get_direction(
                    (current_row as i32, current_col as i32),
                    (test_row as i32, test_col as i32),
                );
                if let Some(next_direction) = maybe_direction {
                    // Don't consider our current node, and only consider NSEW
                    if !(current_row == test_row && current_col == test_col)
                        && (next_direction == Direction::N
                            || next_direction == Direction::S
                            || next_direction == Direction::E
                            || next_direction == Direction::W)
                    {
                        if let Some(next_pipe) = tiles.get(test_row, test_col) {
                            if let Some(prev_index) = maybe_prev_index {
                                let maybe_prev_node = arena.get(prev_index);
                                if let Some(prev_node) = maybe_prev_node {
                                    if prev_node.get().pos != (test_row as usize, test_col as usize)
                                    {
                                        if is_pipe_connected(
                                            current_path,
                                            *next_pipe,
                                            next_direction,
                                        ) {
                                            let new_node = arena.new_node(Entry {
                                                pipe: *next_pipe,
                                                pos: (test_row as usize, test_col as usize),
                                            });
                                            current_index.append(new_node, arena);
                                            build_tree(tiles, arena, Some(current_index), new_node)
                                        }
                                    }
                                }
                            } else {
                                if is_pipe_connected(current_path, *next_pipe, next_direction) {
                                    let new_node = arena.new_node(Entry {
                                        pipe: *next_pipe,
                                        pos: (test_row as usize, test_col as usize),
                                    });
                                    current_index.append(new_node, arena);
                                    build_tree(tiles, arena, Some(current_index), new_node)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_pipe_connected(current_pipe: Pipe, next_pipe: Pipe, next_direction: Direction) -> bool {
    fn is_north_pipe(next_pipe: Pipe) -> bool {
        match next_pipe {
            Pipe::VerticalNS | Pipe::SW90Deg7Sym | Pipe::SE90DegFSym => true,
            _ => false,
        }
    }
    fn is_south_pipe(next_pipe: Pipe) -> bool {
        match next_pipe {
            Pipe::VerticalNS | Pipe::NE90DegLSym | Pipe::NW90DegJSym => true,
            _ => false,
        }
    }
    fn is_east_pipe(next_pipe: Pipe) -> bool {
        match next_pipe {
            Pipe::HorizontalEW | Pipe::NW90DegJSym | Pipe::SW90Deg7Sym => true,
            _ => false,
        }
    }
    fn is_west_pipe(next_pipe: Pipe) -> bool {
        match next_pipe {
            Pipe::HorizontalEW | Pipe::NE90DegLSym | Pipe::SE90DegFSym => true,
            _ => false,
        }
    }
    match current_pipe {
        Pipe::Ground => false,
        Pipe::VerticalNS => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::S => is_south_pipe(next_pipe),
            _ => false,
        },
        Pipe::HorizontalEW => match next_direction {
            Direction::E => is_east_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
            _ => false,
        },
        Pipe::NE90DegLSym => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::E => is_east_pipe(next_pipe),
            _ => false,
        },
        Pipe::NW90DegJSym => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
            _ => false,
        },
        Pipe::SW90Deg7Sym => match next_direction {
            Direction::S => is_south_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
            _ => false,
        },
        Pipe::SE90DegFSym => match next_direction {
            Direction::E => is_east_pipe(next_pipe),
            Direction::S => is_south_pipe(next_pipe),
            _ => false,
        },
        Pipe::StartPos => match next_direction {
            Direction::N => is_north_pipe(next_pipe),
            Direction::E => is_east_pipe(next_pipe),
            Direction::S => is_south_pipe(next_pipe),
            Direction::W => is_west_pipe(next_pipe),
        },
    }
}

fn get_direction(
    (current_row, current_col): (i32, i32),
    (next_row, next_col): (i32, i32),
) -> Option<Direction> {
    if current_row - 1 == next_row && current_col == next_col {
        return Some(Direction::N);
    } else if current_row == next_row && current_col + 1 == next_col {
        return Some(Direction::E);
    } else if current_row + 1 == next_row && current_col == next_col {
        return Some(Direction::S);
    } else if current_row == next_row && current_col - 1 == next_col {
        return Some(Direction::W);
    }
    None
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
        assert_eq!(6823, get_farthest_steps("input/day10.txt"));
    }
}
