#[path = "./helpers.rs"]
pub mod helpers;

use crate::movements::helpers::at;

pub fn move_up(
    state: [usize; 9],
    x: usize,
    y: usize,
    history: &mut Vec<([usize; 9], usize)>,
) -> (bool, [usize; 9]) {
    let mut new_state = state;
    if y == 0 {
        return (false, new_state);
    }
    let temp = new_state[x + y * 3];
    new_state[helpers::at(x, y)] = new_state[at(x, y - 1)];
    new_state[helpers::at(x, y - 1)] = temp;

    if helpers::not_in(new_state, history) {
        (true, new_state)
    } else {
        (false, new_state)
    }
}

pub fn move_down(
    state: [usize; 9],
    x: usize,
    y: usize,
    history: &mut Vec<([usize; 9], usize)>,
) -> (bool, [usize; 9]) {
    let mut new_state = state;
    if y == 2 {
        return (false, new_state);
    }
    let temp = new_state[x + y * 3];
    new_state[helpers::at(x, y)] = new_state[helpers::at(x, y + 1)];
    new_state[helpers::at(x, y + 1)] = temp;

    if helpers::not_in(new_state, history) {
        (true, new_state)
    } else {
        (false, new_state)
    }
}

pub fn move_left(
    state: [usize; 9],
    x: usize,
    y: usize,
    history: &mut Vec<([usize; 9], usize)>,
) -> (bool, [usize; 9]) {
    let mut new_state = state;
    if x == 0 {
        return (false, new_state);
    }
    let temp = new_state[x + y * 3];
    new_state[helpers::at(x, y)] = new_state[helpers::at(x - 1, y)];
    new_state[helpers::at(x - 1, y)] = temp;

    if helpers::not_in(new_state, history) {
        (true, new_state)
    } else {
        (false, new_state)
    }
}

pub fn move_right(
    state: [usize; 9],
    x: usize,
    y: usize,
    history: &mut Vec<([usize; 9], usize)>,
) -> (bool, [usize; 9]) {
    let mut new_state = state;
    if x == 2 {
        return (false, new_state);
    }
    let temp = new_state[x + y * 3];
    new_state[helpers::at(x, y)] = new_state[helpers::at(x + 1, y)];
    new_state[helpers::at(x + 1, y)] = temp;

    if helpers::not_in(new_state, history) {
        (true, new_state)
    } else {
        (false, new_state)
    }
}

pub fn next_moves(
    state: [usize; 9],
    history: &mut Vec<([usize; 9], usize)>,
) -> Vec<([usize; 9], char)> {
    let mut res: Vec<([usize; 9], char)> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for i in 0..8 {
        if state[i] == 0 {
            x = i % 3;
            y = i / 3;
            break;
        }
    }
    let (moved, next_state) = move_up(state, x, y, history);
    if moved {
        res.push((next_state, 'U'));
    }
    let (moved, next_state) = move_down(state, x, y, history);
    if moved {
        res.push((next_state, 'D'));
    }
    let (moved, next_state) = move_left(state, x, y, history);
    if moved {
        res.push((next_state, 'L'));
    }
    let (moved, next_state) = move_right(state, x, y, history);
    if moved {
        res.push((next_state, 'R'));
    }
    res
}
