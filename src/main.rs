use std::thread::sleep;
use std::time::{Duration, Instant};
use std::vec;

#[derive(Debug)]
struct State {
    board: Vec<Vec<bool>>,
}

impl State {
    fn new(size: u32) -> Self {
        Self {
            board: vec![vec![false; size as usize]; size as usize],
        }
    }

    fn randomize(&mut self) {
        for items in self.board.iter_mut() {
            for vals in items.iter_mut() {
                if rand::random::<bool>() == true {
                    *vals = true;
                }
            }
        }
    }

    fn display_board(&self) {
        for col in &self.board {
            println!("");
            for row in col {
                if *row == false {
                    print!("-");
                } else {
                    print!("#")
                }
            }
        }
    }

    fn iteration(&mut self) {
        let original_board = &self.board.clone();
        for cols in 0..self.board.len() {
            for row in 0..self.board[0].len() {
                let mut sum = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        let c = cols as i32 + (i);
                        let r = row as i32 + (j);
                        if i == 0 && j == 0 {
                            continue;
                        }
                        if let Some(val) = original_board
                            .get(c as usize)
                            .and_then(|row| row.get(r as usize))
                        {
                            if *val {
                                sum += 1;
                            } else {
                                continue;
                            }
                        }
                    }
                }

                // println!("THE CELL[{cols}{row}] has a sum of {sum}" );
                if original_board[cols as usize][row as usize] == true && sum < 2 {
                    self.board[cols as usize][row as usize] = false
                } else if original_board[cols as usize][row as usize] == true && sum == 2 {
                    self.board[cols as usize][row as usize] = true
                } else if original_board[cols as usize][row as usize] == true && sum == 3 {
                    self.board[cols as usize][row as usize] = true
                } else if original_board[cols as usize][row as usize] == true && sum > 3 {
                    self.board[cols as usize][row as usize] = false
                } else if original_board[cols as usize][row as usize] == false && sum == 3 {
                    self.board[cols as usize][row as usize] = true
                }
            }
        }
    }
}

fn main() {
    let mut board = State::new(250);
    board.randomize();

    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;

    loop {
        std::process::Command::new("clear").status().unwrap();
        board.display_board();
        board.iteration();
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}
