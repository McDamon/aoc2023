// https://adventofcode.com/2023/day/8

use regex::Regex;

use super::utils::get_lines;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Definition for a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: String,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: String) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    instructions: Vec<Instruction>,
    tree_root: Rc<RefCell<TreeNode>>,
}

fn parse_input(input_file: &str) -> Input {
    lazy_static! {
        static ref RE_TREENODE: Regex =
            Regex::new(r"(?P<root>\w+)\s+=\s+\((?P<left>\w+),\s+(?P<right>\w+)\)").unwrap();
    }

    let lines = get_lines(input_file);

    let instructions = parse_instructions(lines.first().unwrap());

    let mut tree_nodes: Vec<String> = vec![];
    let mut tree_entries: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.into_iter().skip(2) {
        let caps_tree_nodes = RE_TREENODE.captures(&line);
        if let Some(caps_tree_nodes) = caps_tree_nodes {
            let root = caps_tree_nodes["root"].to_string();
            let left = caps_tree_nodes["left"].to_string();
            let right = caps_tree_nodes["right"].to_string();
            tree_nodes.push(root.clone());
            tree_entries.insert(root, (left, right));
        }
    }

    let tree_root = Rc::new(RefCell::new(TreeNode::new(
        tree_nodes.first().unwrap().clone(),
    )));

    for tree_node in tree_nodes {
        let tree_child = find_child(Some(tree_root.clone()), tree_node.as_str());
        
        if let Some(t) = tree_child {
            match tree_entries.get(&t.clone().borrow().val) {
                Some((left, right)) => {
                    t.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(
                        left.to_string(),
                    ))));
                    t.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(
                        right.to_string(),
                    ))));
                },
                None => (),
            }
        }
    }

    println!("{:?}", tree_root);

    Input {
        instructions,
        tree_root,
    }
}

fn find_child(
    root: Option<Rc<RefCell<TreeNode>>>,
    child_node_name: &str,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = root {
        if child_node_name == r.borrow().val.clone() {
            return Some(r);
        }

        let left = r.borrow().left.clone();
        let right = r.borrow().right.clone();

        let l = find_child(left, child_node_name);
        if l.is_some() {
            return l;
        }
        let r = find_child(right, child_node_name);
        if r.is_some() {
            return r;
        }
    }
    None
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for instruction_str in instructions_str.chars() {
        instructions.push(parse_instruction(instruction_str));
    }
    instructions
}

fn parse_instruction(instruction_char: char) -> Instruction {
    return match instruction_char {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("invalid instruction: {}", instruction_char),
    };
}

fn get_num_steps(input_file: &str) -> u64 {
    let mut num_steps: u64 = 1;
    let input = parse_input(input_file);
    num_steps
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
        assert_eq!(0, get_num_steps("input/day08.txt"));
    }
}
