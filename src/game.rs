use rand::prelude::*;
use std::fmt;

pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Game {
    pub board: Vec<Vec<i16>>,
    pub size: usize,
    pub score: u32,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = thread_rng();
        let mut game = Game {
            size: 4,
            board: vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            score: 0,
        };

        let i = rng.gen_range(0..game.size) as usize;
        let j = rng.gen_range(0..game.size) as usize;

        game.board[i][j] = 2 as i16;

        let i = rng.gen_range(0..game.size) as usize;
        let j = rng.gen_range(0..game.size) as usize;
        game.board[i][j] = 2;

        game
    }

    pub fn compute(&mut self, action: Action) {
        match action {
            Action::Up => self.compute_up(),
            _ => unimplemented!(),
        };
    }

    fn is_cell_empty(&self, i: usize, j: usize) -> bool {
        self.board[i][j] == 0
    }

    fn compute_up(&mut self) {
        for j in 0..self.size {
            for i in 0..self.size {
                if !self.is_cell_empty(i, j) {
                    let mut l = i;
                    let val = self.board[i][j];
                    loop {
                        if l > 0 {
                            l -= 1;
                            if self.is_cell_empty(l, j) {
                                self.board[l][j] = val;
                                self.board[l + 1][j] = 0;
                            } else {
                                if self.board[l][j] == val {
                                    self.board[l][j] = 2 * val;
                                    self.score += 2 * val as u32;
                                    self.board[l + 1][j] = 0;
                                }
                            }
                        }
                        if l == 0 {
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result = format!("Score: {}\r\n\n", self.score);

        for i in 0..self.size {
            let mut line = String::new();
            for j in 0..self.size {
                line.push_str(&format!("{} ", self.board[i][j]));
            }
            result.push_str(&format!("{}\r\n", line));
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn when_run_up_should_move_everything_in_upper_row() {
        let mut game = Game {
            board: vec![
                vec![2, 0, 0, 0],
                vec![0, 2, 0, 0],
                vec![0, 0, 2, 0],
                vec![0, 0, 0, 2],
            ],
            size: 4,
            score: 0,
        };

        let expected_state = vec![
            vec![2, 2, 2, 2],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        game.compute(Action::Up);
        assert_eq!(game.board, expected_state);
        assert_eq!(game.score, 0);
    }

    #[test]
    fn when_run_up_and_there_is_a_double_it_should_be_summed_and_moved_up() {
        let mut game = Game {
            board: vec![
                vec![2, 0, 0, 0],
                vec![2, 0, 2, 2],
                vec![2, 0, 2, 2],
                vec![2, 2, 2, 0],
            ],
            size: 4,
            score: 0,
        };

        let expected_state = vec![
            vec![4, 2, 4, 4],
            vec![4, 0, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        game.compute(Action::Up);
        assert_eq!(game.board, expected_state);
        assert_eq!(game.score, 16);
    }
}
