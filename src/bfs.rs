#[path = "./movements.rs"]
pub mod movements;
use crate::movements::*;

use std::collections::{HashSet, VecDeque};

fn is_in(open_set: &VecDeque<([usize; 9], Vec<char>)>, elem: &[usize; 9]) -> bool {
    for (child, _) in open_set {
        if child == elem {
            return true;
        }
    }
    false
}

pub fn bfs(root: [usize; 9]) -> Vec<char> {
    let mut open_set: VecDeque<([usize; 9], Vec<char>)> = VecDeque::new();
    let mut closed_set: HashSet<[usize; 9]> = HashSet::new();
    open_set.push_front((root, Vec::new()));
    while !open_set.is_empty() {
        let (subtree_root, path) = open_set.pop_back().unwrap();
        if subtree_root == movements::helpers::WINSTATE {
            return path;
        }
        for (child, action) in next_moves(subtree_root, &mut Vec::new()) {
            if closed_set.contains(&child) {
                continue;
            }
            if !is_in(&open_set, &child) {
                let mut new_path = path.clone();
                new_path.push(action);
                open_set.push_front((child, new_path));
            }
        }
        closed_set.insert(subtree_root);
    }
    return Vec::new();
}
