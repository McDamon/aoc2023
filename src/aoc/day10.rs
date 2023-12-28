// https://adventofcode.com/2023/day/10

use std::{cell::RefCell, rc::Rc};

use grid::Grid;

use super::utils::get_lines;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
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
    StartPos = b'S',
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pipe: Pipe,
    pos: (usize, usize),
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(pipe: Pipe, pos: (usize, usize)) -> Self {
        TreeNode {
            pipe,
            pos,
            left: None,
            right: None,
        }
    }
}

fn get_farthest_steps(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for ((row, col), pipe) in input.tiles.indexed_iter() {
        if *pipe == Pipe::StartPos {
            start_pos = Some((row, col));
        }
    }

    if let Some((row, col)) = start_pos {
        let root_node = Rc::new(RefCell::new(TreeNode::new(Pipe::StartPos, (row, col))));
        build_tree(row, col, &input.tiles, &root_node);

        traverse_tree(&Some(root_node));
    } else {
        panic!("Invalid start node");
    }

    0
}

fn build_tree(row: usize, col: usize, tiles: &Grid<Pipe>, parent: &Rc<RefCell<TreeNode>>) {
    let current_pipe = parent.borrow().pipe;

    /*println!(
        "(current) row: {}, col: {}, tile: {:?}",
        row, col, current_pipe
    );*/
    for i in (row - 1)..(row + 2) {
        for j in (col - 1)..(col + 2) {
            let next_direction = get_direction((row as i32, col as i32), (i as i32, j as i32));
            if let Some(next_direction) = next_direction {
                if let Some(next_pipe) = tiles.get(i, j) {
                    if is_pipe_connected(current_pipe, *next_pipe, next_direction) && !(parent.borrow().pos == (i, j)) {
                        println!(
                            "(next) row: {}, col: {}, tile: {:?}, dir: {:?}",
                            i, j, next_pipe, next_direction
                        );
                        if parent.borrow().left.is_some() {
                            parent.borrow_mut().right =
                                Some(Rc::new(RefCell::new(TreeNode::new(*next_pipe, (i, j)))));
                            build_tree(i, j, tiles, parent.borrow_mut().right.as_ref().unwrap())
                        } else {
                            parent.borrow_mut().left =
                                Some(Rc::new(RefCell::new(TreeNode::new(*next_pipe, (i, j)))));
                            build_tree(i, j, tiles, parent.borrow_mut().left.as_ref().unwrap())
                        }
                    };
                };
            };
        }
    }
}

fn traverse_tree(parent: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = parent {
        let node = node.borrow();

        traverse_tree(&node.left);

        println!("{:?}", node);

        traverse_tree(&node.right);
    }
}

fn is_pipe_connected(current_pipe: Pipe, next_pipe: Pipe, next_direction: Direction) -> bool {
    match current_pipe {
        Pipe::Ground => false,
        Pipe::VerticalNS => match next_direction {
            Direction::N => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
            Direction::S => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            _ => false,
        },
        Pipe::HorizontalEW => match next_direction {
            Direction::E => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            Direction::W => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
            _ => false,
        },
        Pipe::NE90Deg => match next_direction {
            Direction::N => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
            Direction::E => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            _ => false,
        },
        Pipe::NW90Deg => match next_direction {
            Direction::N => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            Direction::W => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
            _ => false,
        },
        Pipe::SW90Deg => match next_direction {
            Direction::S => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            Direction::W => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
            _ => false,
        },
        Pipe::SE90Deg => match next_direction {
            Direction::E => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            Direction::S => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            _ => false,
        },
        Pipe::StartPos => match next_direction {
            Direction::N => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
            Direction::E => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => false,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => true,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            Direction::S => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => true,
                Pipe::HorizontalEW => false,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => true,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => false,
                Pipe::StartPos => false,
            },
            Direction::W => match next_pipe {
                Pipe::Ground => false,
                Pipe::VerticalNS => false,
                Pipe::HorizontalEW => true,
                Pipe::NE90Deg => true,
                Pipe::NW90Deg => false,
                Pipe::SW90Deg => false,
                Pipe::SE90Deg => true,
                Pipe::StartPos => false,
            },
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
        assert_eq!(0, get_farthest_steps("input/day10.txt"));
    }
}
