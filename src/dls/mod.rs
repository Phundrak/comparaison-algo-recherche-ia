#[path = "../movements/mod.rs"]
pub mod movements;
use crate::movements::helpers::*;
use crate::movements::*;

pub static mut SOLSIZE: usize = 0;

pub fn dls(
    state: [usize; 9],
    depth: usize,
    solution: &mut [char; MAX],
    history: &mut Vec<([usize; 9], usize)>,
    sol: &mut bool,
    max: usize,
) {
    if depth == max {
        return;
    }
    let next_states = movements::next_moves(state, history);
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
            if new_state == WINSTATE {
                unsafe {
                    SOLSIZE = depth;
                }
                *sol = true;
                return;
            }
            dls(new_state, depth + 1, solution, history, sol, max);
        }
        if *sol {
            break;
        }
    }
}
