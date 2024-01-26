use crate::board::{Board, Color};

pub fn best_move(board: Board, color: Color) -> usize {
    best_move_search(board, color, 5)
}

fn best_move_search(board: Board, color: Color, depth: u8) -> usize {
    0
}