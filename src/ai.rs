use crate::board::{Board, Color, MoveResult};

const WIN_VALUE: f32 = f32::INFINITY;
const DRAW_VALUE: f32 = 0.0;
const TWO_ROW_VALUE: f32 = 5.0;
const THREE_ROW_VALUE: f32 = 50.0;

const SEARCH_DEPTH: u8 = 7;

pub fn best_move(board: &mut Board) -> usize {
    let mut best_value = f32::NEG_INFINITY;
    let mut best_board_move = 0;
    for board_move in 0..7 {
        if board.can_make_move(board_move) {
            let value = match board.make_move(board_move) {
                MoveResult::None => {
                    -best_move_search(board, SEARCH_DEPTH, f32::NEG_INFINITY, f32::INFINITY)
                }
                MoveResult::Draw => {
                    DRAW_VALUE
                }
                MoveResult::Win => {
                    WIN_VALUE
                }
            };
            if value > best_value {
                best_value = value;
                best_board_move = board_move;
            }
            board.unmake_move(board_move);
        }
    }
    best_board_move
}

fn best_move_search(board: &mut Board, depth: u8, mut alpha: f32, beta: f32) -> f32 {
    let depth = depth - 1;
    let mut best_value = f32::NEG_INFINITY;
    for board_move in 0..7 {
        if board.can_make_move(board_move) {
            let value = match (board.make_move(board_move), depth) {
                (_, 0) => {
                    evaluate_board(board)
                }
                (MoveResult::None, _) => {
                    -best_move_search(board, depth, -beta, -alpha)
                }
                (MoveResult::Draw, _) => {
                    DRAW_VALUE
                }
                (MoveResult::Win, _) => {
                    WIN_VALUE
                }
            };
            board.unmake_move(board_move);
            if value >= beta {
                return beta;
            }
            alpha = alpha.max(value);
            best_value = best_value.max(value);
        }
    }
    best_value
}

fn evaluate_board(board: &mut Board) -> f32 {
    evaluate_color(board, board.turn) - evaluate_color(board, board.turn.opposite())
}

fn evaluate_color(board: &mut Board, color: Color) -> f32 {
    let mut value = 0.0;
    let opposite_color = color.opposite();

    // check diagonals
    for x in 0..4 {
        'diag: for y in 0..3 {
            let mut length: u8 = 0;
            let mut max_length: u8 = 0;
            for square in 0..4 {
                let pos_color = board.board[board.position_to_index(x + square, y + square)];
                if pos_color == Color::Empty {
                    max_length = max_length.max(length);
                    length = 0;
                } else if pos_color == opposite_color {
                    continue 'diag;
                } else {
                    length += 1;
                }
            }
            if length > max_length {
                max_length = length;
            }
            value += match max_length {
                2 => TWO_ROW_VALUE,
                3 => THREE_ROW_VALUE,
                _ => 0.0,
            }
        }
    }
    for x in 3..7 {
        'diag: for y in 0..3 {
            let mut length: u8 = 0;
            let mut max_length: u8 = 0;
            for square in 0..4 {
                let pos_color = board.board[board.position_to_index(x - square, y + square)];
                if pos_color == Color::Empty {
                    max_length = max_length.max(length);
                    length = 0;
                } else if pos_color == opposite_color {
                    continue 'diag;
                } else {
                    length += 1;
                }
            }
            if length > max_length {
                max_length = length;
            }
            value += match max_length {
                2 => TWO_ROW_VALUE,
                3 => THREE_ROW_VALUE,
                _ => 0.0,
            }
        }
    }

    // check straights
    for x in 0..7 {
        'straight: for y in 0..3 {
            let mut length: u8 = 0;
            let mut max_length: u8 = 0;
            for square in 0..4 {
                let pos_color = board.board[board.position_to_index(x, y + square)];
                if pos_color == Color::Empty {
                    max_length = max_length.max(length);
                    length = 0;
                } else if pos_color == opposite_color {
                    continue 'straight;
                } else {
                    length += 1;
                }
            }
            if length > max_length {
                max_length = length;
            }
            value += match max_length {
                2 => TWO_ROW_VALUE,
                3 => THREE_ROW_VALUE,
                _ => 0.0,
            }
        }
    }
    for x in 0..4 {
        'straight: for y in 0..6 {
            let mut length: u8 = 0;
            let mut max_length: u8 = 0;
            for square in 0..4 {
                let pos_color = board.board[board.position_to_index(x + square, y)];
                if pos_color == Color::Empty {
                    max_length = max_length.max(length);
                    length = 0;
                } else if pos_color == opposite_color {
                    continue 'straight;
                } else {
                    length += 1;
                }
            }
            if length > max_length {
                max_length = length;
            }
            value += match max_length {
                2 => TWO_ROW_VALUE,
                3 => THREE_ROW_VALUE,
                _ => 0.0,
            }
        }
    }

    value
}