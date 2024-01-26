use std::io;

use crate::board::Color;

mod ai;
mod board;

fn main() {
    let mut game = board::Board::empty();
    let mut input_string = String::new();
    println!("Player begins (y/n)?");
    io::stdin().read_line(&mut input_string).unwrap();
    game.turn =  if input_string.trim().to_lowercase() == "y" { Color::Blue } else { Color::Red };
    loop {
        if game.turn == Color::Blue {
            game.print();
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            let column = input_string.trim().parse().unwrap();
            match game.make_move(column) {
                board::MoveResult::Win => {
                    println!("Player win");
                    game.print();
                    game.clear();
                }
                board::MoveResult::Draw => {
                    println!("Draw");
                    game.print();
                    game.clear();
                }
                board::MoveResult::None => {
                    game.print();
                }
            }
        } else {
            let ai_move = ai::best_move(&mut game);
            match game.make_move(ai_move) {
                board::MoveResult::Win => {
                    println!("AI win");
                    game.print();
                    game.clear();
                }
                board::MoveResult::Draw => {
                    println!("Draw");
                    game.print();
                    game.clear();
                }
                _ => {}
            }
        }
    }
}
