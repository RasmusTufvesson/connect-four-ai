use crate::board::Board;

pub fn best_move(board: &mut Board) -> usize {
    best_move_search(board, 5)
}

fn best_move_search(board: &mut Board, depth: u8) -> usize {
    0
}