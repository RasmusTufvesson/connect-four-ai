#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Empty,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Blue => Self::Red,
            Self::Red => Self::Blue,
        }
    }
}

pub enum MoveResult {
    None,
    Win,
    Draw,
}

pub struct Board {
    pub board: [Color; 42],
    pub turn: Color,
}

impl Board {
    pub fn empty() -> Self {
        Self { board: [Color::Empty; 42], turn: Color::Blue }
    }

    pub fn clear(&mut self) {
        self.board.fill(Color::Empty);
    }

    pub fn make_move(&mut self, column: usize) -> MoveResult {
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
        let result = self.get_result();
        self.switch_turn();
        result
    }

    pub fn position_to_index(&self, x: usize, y: usize) -> usize {
        x + y * 7
    }

    pub fn can_make_move(&self, column: usize) -> bool {
        self.board[column] == Color::Empty
    }

    pub fn unmake_move(&mut self, column: usize) {
        let mut cur_index = self.position_to_index(column, 0);
        if self.board[cur_index] == Color::Empty {
            while cur_index < 42 {
                cur_index += 7;
                if self.board[cur_index] != Color::Empty {
                    break;
                }
            }
        }
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

    fn win(&self, color: Color) -> bool {
        // check diagonals
        for x in 0..4 {
            'diag: for y in 0..3 {
                for square in 0..4 {
                    if self.board[self.position_to_index(x + square, y + square)] != color {
                        continue 'diag;
                    }
                }
                return true;
            }
        }
        for x in 3..7 {
            'diag: for y in 0..3 {
                for square in 0..4 {
                    if self.board[self.position_to_index(x - square, y + square)] != color {
                        continue 'diag;
                    }
                }
                return true;
            }
        }

        // check straights
        for x in 0..7 {
            'straight: for y in 0..3 {
                for square in 0..4 {
                    if self.board[self.position_to_index(x, y + square)] != color {
                        continue 'straight;
                    }
                }
                return true;
            }
        }
        for x in 0..4 {
            'straight: for y in 0..6 {
                for square in 0..4 {
                    if self.board[self.position_to_index(x + square, y)] != color {
                        continue 'straight;
                    }
                }
                return true;
            }
        }

        // everything passed
        false
    }

    fn draw(&self) -> bool {
        for x in 0..7 {
            if self.can_make_move(x) {
                return false;
            }
        }
        true
    }

    fn get_result(&self) -> MoveResult {
        if self.win(self.turn) {
            return MoveResult::Win;
        } else if self.draw() {
            return MoveResult::Draw;
        } else {
            return MoveResult::None;
        }
    }
}