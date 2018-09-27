#[path = "../movements/mod.rs"]
pub mod movements;

use crate::movements::helpers::*;
use crate::movements::*;

pub static mut SOLSIZE: usize = 0;

fn dls(
    state: [usize; 9],
    depth: usize,
    solution: &mut [char; MAX],
    history: &mut Vec<([usize; 9], usize)>,
    max: usize,
) -> (Option<[usize; 9]>, bool) {
    if depth == max {
        if state == WINSTATE {
            unsafe {
                SOLSIZE = depth;
            }
            return (Some(state), true);
        } else {
            return (None, true); // peut encore avoir des enfants
        }
    }
    let mut any_remaining = false;
    let next_states = movements::next_moves(state, history);
    // foreach child of node
    for (new_state, move_char) in next_states {
        solution[depth] = move_char;
        let (notin, (betterval, index)) = (
            helpers::not_in(new_state, history),
            helpers::better_value(new_state, history, depth),
        );
        if notin || betterval {
            if betterval {
                history[index].1 = depth;
            } else {
                history.push((new_state, depth));
            }
            let (found, remaining) = dls(new_state, depth + 1, solution, history, max);
            if found != None {
                return (found, true);
            }
            if remaining {
                any_remaining = true;
            }
        }
    }
    return (None, any_remaining);
}

pub fn ids(root: [usize; 9]) -> Option<[char; MAX]> {
    for depth in 0..MAX {
        let mut solution: [char; MAX] = ['N'; MAX];
        let (found, remaining) = dls(root, 0, &mut solution, &mut Vec::new(), depth);
        if found != None {
            return Some(solution);
        } else if !remaining {
            unsafe {
                SOLSIZE = 0;
            }
            return None;
        }
    }
    return None;
}
