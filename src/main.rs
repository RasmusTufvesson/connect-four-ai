use std::io;

mod ai;
mod board;

fn main() {
    let mut game = board::Board::empty();
    let mut input_string = String::new();
    loop {
        game.print();
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        let column = input_string.trim().parse().unwrap();
        match game.make_move(column) {
            board::MoveResult::Win => {
                println!("Win");
                game.clear();
            }
            board::MoveResult::Draw => {
                println!("Draw");
                game.clear();
            }
            board::MoveResult::None => {
                game.print();
            }
        }
        let ai_move = ai::best_move(&mut game);
        game.make_move(ai_move);
    }
}
