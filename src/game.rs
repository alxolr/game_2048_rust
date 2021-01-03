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
    pub fn new(size: usize) -> Game {
        let mut rng = thread_rng();
        let mut board: Vec<Vec<i16>> = vec![];

        let mut line: Vec<i16> = vec![];
        for _ in 0..size {
            line.push(0);
        }
        for _ in 0..size {
            board.push(line.clone());
        }

        let mut game = Game {
            size,
            board,
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
            Action::Down => self.compute_down(),
            Action::Left => self.compute_left(),
            Action::Right => self.compute_right(),
        };
    }

    pub fn generate_new_cell(&mut self) {
        let mut rng = thread_rng();
        if !self.is_board_full() {
            loop {
                let i = rng.gen_range(0..self.size) as usize;
                let j = rng.gen_range(0..self.size) as usize;

                if self.is_cell_empty(i, j) {
                    if rng.gen_bool(0.25) {
                        self.board[i][j] = 4;
                    } else {
                        self.board[i][j] = 2;
                    }
                    break;
                }
            }
        }
    }

    pub fn is_board_full(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.is_cell_empty(i, j) {
                    return false;
                }
            }
        }

        return true;
    }

    fn is_cell_empty(&self, i: usize, j: usize) -> bool {
        self.board[i][j] == 0
    }

    fn compute_right(&mut self) {
        for i in 0..self.size {
            for j in (0..self.size).rev() {
                if !self.is_cell_empty(i, j) {
                    let mut pivot = j;
                    let val = self.board[i][j];
                    loop {
                        if pivot < self.size - 1 {
                            pivot += 1;
                            if self.is_cell_empty(i, pivot) {
                                self.board[i][pivot] = val;
                                self.board[i][pivot - 1] = 0;
                            } else {
                                if self.board[i][pivot] == val {
                                    self.board[i][pivot] = 2 * val;
                                    self.score += 2 * val as u32;
                                    self.board[i][pivot - 1] = 0;
                                }
                            }
                        }
                        if pivot == self.size - 1 {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn compute_left(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                if !self.is_cell_empty(i, j) {
                    let mut pivot = j;
                    let val = self.board[i][j];
                    loop {
                        if pivot > 0 {
                            pivot -= 1;
                            if self.is_cell_empty(i, pivot) {
                                self.board[i][pivot] = val;
                                self.board[i][pivot + 1] = 0;
                            } else {
                                if self.board[i][pivot] == val {
                                    self.board[i][pivot] = 2 * val;
                                    self.score += 2 * val as u32;
                                    self.board[i][pivot + 1] = 0;
                                }
                            }
                        }
                        if pivot == 0 {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn compute_down(&mut self) {
        for j in 0..self.size {
            for i in (0..self.size).rev() {
                if !self.is_cell_empty(i, j) {
                    let mut pivot = i;
                    let val = self.board[i][j];
                    loop {
                        if pivot < self.size - 1 {
                            pivot += 1;
                            if self.is_cell_empty(pivot, j) {
                                self.board[pivot][j] = val;
                                self.board[pivot - 1][j] = 0;
                            } else {
                                if self.board[pivot][j] == val {
                                    self.board[pivot][j] = 2 * val;
                                    self.score += 2 * val as u32;
                                    self.board[pivot - 1][j] = 0;
                                }
                            }
                        }
                        if pivot == self.size - 1 {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn compute_up(&mut self) {
        for j in 0..self.size {
            for i in 0..self.size {
                if !self.is_cell_empty(i, j) {
                    let mut pivot = i;
                    let val = self.board[i][j];
                    loop {
                        if pivot > 0 {
                            pivot -= 1;
                            if self.is_cell_empty(pivot, j) {
                                self.board[pivot][j] = val;
                                self.board[pivot + 1][j] = 0;
                            } else {
                                if self.board[pivot][j] == val {
                                    self.board[pivot][j] = 2 * val;
                                    self.score += 2 * val as u32;
                                    self.board[pivot + 1][j] = 0;
                                }
                            }
                        }
                        if pivot == 0 {
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
                line.push_str(&format!(
                    "{number:>width$} ",
                    number = self.board[i][j],
                    width = 4
                ));
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

    #[test]
    fn when_run_down_it_should_compute_and_move_everything_down() {
        let mut game = Game {
            board: vec![vec![2, 0, 0], vec![2, 0, 2], vec![2, 0, 2]],
            size: 3,
            score: 0,
        };

        let expected_state = vec![vec![0, 0, 0], vec![2, 0, 0], vec![4, 0, 4]];
        game.compute(Action::Down);
        assert_eq!(game.board, expected_state);
        assert_eq!(game.score, 8);
    }

    #[test]
    fn when_run_left_it_should_compute_and_move_everything_left() {
        let mut game = Game {
            board: vec![vec![2, 2, 2], vec![2, 2, 0], vec![0, 0, 2]],
            size: 3,
            score: 0,
        };

        let expected_state = vec![vec![4, 2, 0], vec![4, 0, 0], vec![2, 0, 0]];

        game.compute(Action::Left);
        assert_eq!(game.board, expected_state);
        assert_eq!(game.score, 8);
    }

    #[test]
    fn when_run_right_should_compute_and_move_everything_right() {
        let mut game = Game {
            board: vec![vec![2, 2, 2], vec![2, 2, 0], vec![0, 0, 2]],
            size: 3,
            score: 0,
        };

        let expected_state = vec![vec![0, 2, 4], vec![0, 0, 4], vec![0, 0, 2]];

        game.compute(Action::Right);
        assert_eq!(game.board, expected_state);
        assert_eq!(game.score, 8);
    }
}
