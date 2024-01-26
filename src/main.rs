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
        game.make_move(column);
    }
}
