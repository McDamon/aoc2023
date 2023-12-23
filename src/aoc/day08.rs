// https://adventofcode.com/2023/day/8

use regex::Regex;

use super::utils::get_lines;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    instructions: Vec<Instruction>,
    tree_root: String,
    tree_leaf: String,
    tree_nodes: HashMap<String, (String, String)>,
}

fn parse_input(input_file: &str) -> Input {
    lazy_static! {
        static ref RE_TREENODE: Regex =
            Regex::new(r"(?P<root>\w+)\s+=\s+\((?P<left>\w+),\s+(?P<right>\w+)\)").unwrap();
    }

    let lines = get_lines(input_file);

    let instructions = parse_instructions(lines.first().unwrap());

    let mut tree_node_names: Vec<String> = vec![];
    let mut tree_nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.into_iter().skip(2) {
        let caps_tree_nodes = RE_TREENODE.captures(&line);
        if let Some(caps_tree_nodes) = caps_tree_nodes {
            let root = caps_tree_nodes["root"].to_string();
            let left = caps_tree_nodes["left"].to_string();
            let right = caps_tree_nodes["right"].to_string();
            tree_node_names.push(root.clone());
            tree_nodes.insert(root, (left, right));
        }
    }

    tree_node_names.sort();

    Input {
        instructions,
        tree_root: tree_node_names.first().unwrap().to_string(),
        tree_leaf: tree_node_names.last().unwrap().to_string(),
        tree_nodes,
    }
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for instruction_str in instructions_str.chars() {
        instructions.push(parse_instruction(instruction_str));
    }
    instructions
}

fn parse_instruction(instruction_char: char) -> Instruction {
    match instruction_char {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("Invalid instruction: {}", instruction_char),
    }
}

fn get_num_steps(input_file: &str) -> u64 {
    let mut num_steps: u64 = 0;
    let input = parse_input(input_file);
    let mut current_node = Some(input.tree_root);
    for instruction in input.instructions.iter().cycle() {
        current_node = traverse_tree(
            &input.tree_nodes,
            current_node.unwrap().as_str(),
            &input.tree_leaf,
            instruction,
        );
        num_steps += 1;
        if current_node.is_none() {
            break;
        }
    }

    num_steps
}

fn traverse_tree(
    tree_nodes: &HashMap<String, (String, String)>,
    current_node: &str,
    leaf_node: &str,
    instruction: &Instruction,
) -> Option<String> {
    match tree_nodes.get(current_node) {
        Some((left, right)) => {
            if *instruction == Instruction::Left && current_node != left && left != leaf_node {
                return Some(left.clone());
            } else if *instruction == Instruction::Right && current_node != right && right != leaf_node {
                return Some(right.clone());
            }
        }
        None => panic!("Invalid current_node"),
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_steps_test01() {
        assert_eq!(2, get_num_steps("input/day08_test01.txt"));
    }

    #[test]
    fn test_get_num_steps_test02() {
        assert_eq!(6, get_num_steps("input/day08_test02.txt"));
    }

    #[test]
    fn test_get_num_steps() {
        assert_eq!(21883, get_num_steps("input/day08.txt"));
    }
}
