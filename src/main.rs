pub mod dls;
pub mod ids;
pub mod movements;

static INIT: [usize; 9] = [1, 2, 3, 4, 0, 5, 6, 7, 8];

use crate::movements::helpers::*;
use std::time::Instant;

fn main() {
    test_dls();
    test_ids();
}

fn test_dls() {
    let mut solution: [char; MAX] = ['N'; MAX];
    let mut history: Vec<([usize; 9], usize)> = Vec::new();
    let mut sol: bool = false;
    let now = Instant::now();
    dls::dls(INIT, 0, &mut solution, &mut history, &mut sol, MAX);
    let end = Instant::now();
    unsafe {
        println!("Solution trouvée en {} coups !", dls::SOLSIZE);
        for i in 0..dls::SOLSIZE {
            print!("{}", solution[i]);
        }
        println!("");
    }
    println!("Trouvé avec DLS en {:?}", end.duration_since(now));
}

fn test_ids() {
    let now = Instant::now();
    let res = ids::ids(INIT);
    let end = Instant::now();

    unsafe {
        println!("Solution trouvée en {} coups!", ids::SOLSIZE);
        for i in 0..ids::SOLSIZE {
            print!("{}", res.unwrap_or(['N';MAX])[i]);
        }
        println!("");
    }

    println!("Trouvé avec IDS en {:?}", end.duration_since(now));
}

#[cfg(test)]
mod test {
    use crate::movements::*;
    #[test]
    fn t_next_moves() {
        let res = next_moves([1, 2, 3, 4, 0, 5, 6, 7, 8], &mut Vec::new());
        assert_eq!(4, res.len());
    }

    #[test]
    fn t_move_up() {
        let (moved, res) = move_up([1, 2, 3, 4, 0, 5, 6, 7, 8], 1, 1, &mut Vec::new());
        assert_eq!(true, moved);
        assert_eq!([1, 0, 3, 4, 2, 5, 6, 7, 8], res);
        let (moved, _) = move_up(res, 1, 0, &mut Vec::new());
        assert_eq!(false, moved);
    }

    #[test]
    fn t_move_down() {
        let (moved, res) = move_down([1, 2, 3, 4, 0, 5, 6, 7, 8], 1, 1, &mut Vec::new());
        assert_eq!(true, moved);
        assert_eq!([1, 2, 3, 4, 7, 5, 6, 0, 8], res);
        let (moved, _) = move_down(res, 1, 2, &mut Vec::new());
        assert_eq!(false, moved);
    }

    #[test]
    fn t_move_right() {
        let (moved, res) = move_right([1, 2, 3, 4, 0, 5, 6, 7, 8], 1, 1, &mut Vec::new());
        assert_eq!(true, moved);
        assert_eq!([1, 2, 3, 4, 5, 0, 6, 7, 8], res);
        let (moved, _) = move_right(res, 2, 1, &mut Vec::new());
        assert_eq!(false, moved);
    }

    #[test]
    fn t_move_left() {
        let (moved, res) = move_left([1, 2, 3, 4, 0, 5, 6, 7, 8], 1, 1, &mut Vec::new());
        assert_eq!(true, moved);
        assert_eq!([1, 2, 3, 0, 4, 5, 6, 7, 8], res);
        let (moved, _) = move_left(res, 0, 1, &mut Vec::new());
        assert_eq!(false, moved);
    }

}
