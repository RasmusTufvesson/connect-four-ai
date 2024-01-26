#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Empty,
}

impl Color {
    fn opposite(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Blue => Self::Red,
            Self::Red => Self::Blue,
        }
    }
}

pub struct Board {
    board: [Color; 42],
    turn: Color,
}

impl Board {
    pub fn empty() -> Self {
        Self { board: [Color::Empty; 42], turn: Color::Blue }
    }

    pub fn clear(&mut self) {
        self.board.fill(Color::Empty);
    }

    pub fn make_move(&mut self, column: usize) {
        let mut cur_index = column;
        loop {
            let next_index = cur_index + 7;
            if next_index >= 42 {
                break;
            }
            if self.board[next_index] != Color::Empty {
                break;
            }
            cur_index = next_index;
        };
        self.board[cur_index] = self.turn;
        self.switch_turn();
    }

    fn position_to_index(&self, x: usize, y: usize) -> usize {
        x + y * 7
    }

    pub fn can_make_move(&self, column: usize) -> bool {
        self.board[column] == Color::Empty
    }

    pub fn unmake_move(&mut self, column: usize) {
        let mut cur_index = self.position_to_index(column, 0);
        while cur_index < 42 {
            cur_index += 7;
            if self.board[cur_index] != Color::Empty {
                break;
            }
        };
        self.board[cur_index] = Color::Empty;
        self.switch_turn();
    }

    fn switch_turn(&mut self) {
        self.turn = self.turn.opposite()
    }

    pub fn print(&self) {
        let mut lines = vec![];
        let mut line = vec![];
        for color in self.board {
            line.push(match color {
                Color::Empty => "-",
                Color::Blue => "x",
                Color::Red => "o",
            });
            if line.len() == 7 {
                lines.push(line.join(" "));
                line.clear();
            }
        }
        lines.push(line.join(" "));
        println!("{}", lines.join("\n"));
    }
}