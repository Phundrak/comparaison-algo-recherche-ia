pub const MAX: usize = 80;
pub static WINSTATE: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 0];

pub fn at(x: usize, y: usize) -> usize {
    x + y * 3
}

pub fn not_in(state: [usize; 9], history: &mut Vec<([usize; 9], usize)>) -> bool {
    for (elem, _) in history {
        if *elem == state {
            return false;
        }
    }
    true
}

pub fn better_value(
    state: [usize; 9],
    history: &mut Vec<([usize; 9], usize)>,
    depth: usize,
) -> (bool, usize) {
    for i in 0..history.len() {
        if history[i].0 == state && history[i].1 > depth {
            return (true, i);
        }
    }
    (false, 0)
}
