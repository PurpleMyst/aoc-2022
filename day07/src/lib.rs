use std::{collections::HashMap, fmt::Display};

use peeking_take_while::PeekableExt;
use id_tree::{Tree, Node, InsertBehavior::{AsRoot, UnderNode}};

const DISK_SPACE: u64 = 70_000_000;
const NEEDED_SPACE: u64 = 30_000_000;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut input = include_str!("input.txt").trim().lines().peekable();

    let mut tree = Tree::new();

    let mut weights: HashMap<_, u64> = HashMap::new();
    let mut stack = Vec::new();

    let mut root = None;

    while let Some(line) = input.next() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            if dir == ".." {
                stack.pop().unwrap();
            } else if dir == "/" {
                root = Some(tree.insert(Node::new(0), AsRoot).unwrap());
                stack.push(root.clone().unwrap());
            } else {
                stack.push(tree.insert(Node::new(0), UnderNode(stack.last().unwrap())).unwrap());
            }
        } else if line == "$ ls" {
            input
                .by_ref()
                .peeking_take_while(|line| !line.starts_with('$'))
                .map(|line| line.split_once(' ').unwrap())
                .for_each(|(size, _name)| {
                if size == "dir" { return; }
                tree.insert(Node::new(size.parse().unwrap()), UnderNode(stack.last().unwrap())).unwrap();
                });
        } else {
            unreachable!();
        }
    }
    let root = root.unwrap();

    tree.traverse_post_order_ids(&root).unwrap().for_each(|node_id| {
        let node = tree.get(&node_id).unwrap();
        let Some(parent) = node.parent() else { return; };
        *weights.entry(parent).or_default() += if *node.data() == 0 {
         weights[&node_id]
        } else {
            *node.data()
        };
    });

    let p1: u64 = weights.values().filter(|&&v| v <= 100_000).sum();

    let delete_target = NEEDED_SPACE - (DISK_SPACE - weights[&root]);

    let p2 = tree.traverse_pre_order_ids(&root).unwrap().filter_map(|node_id| {
        let Some(&size) = weights.get(&node_id) else { return None; };
        if size < delete_target {
            return None;
        }
        Some(size)
    }).min().unwrap();

    (p1, p2)
}
