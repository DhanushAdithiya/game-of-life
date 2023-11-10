use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct State {
    board: Vec<Vec<bool>>,
    dim: u32,
}

impl State {
    fn new(size: u32) -> Self {
        Self { 
            board: vec![vec![false; size as usize];size as usize], 
            dim: size,
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
        for cols in 0..self.dim {
            for row in 0..self.dim {
                let mut sum: u32 = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        let r = cols as i32 + (i);
                        let c = row as i32 + (j);
                        if j == 0 && i == 0 {
                            continue;
                        } 
                        if let Some(val) = self.board.get(c as usize).and_then(|row| row.get(r as usize)) {
                            if *val {
                                sum += 1;
                            }
                        } else {
                            continue;
                        }
                    }
                }

/*
Any live cell with fewer than two live neighbours dies, as if by underpopulation.
Any live cell with two or three live neighbours lives on to the next generation.
Any live cell with more than three live neighbours dies, as if by overpopulation.
Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
*/

                if self.board[cols as usize][row as usize] == true && sum < 2{
                    self.board[cols as usize][row as usize] = false 
                } else if self.board[cols as usize][row as usize] == true && (sum == 2 || sum == 3) {
                    self.board[cols as usize][row as usize] = true 
                }  else if self.board[cols as usize][row as usize] == true && sum > 3 {
                    self.board[cols as usize][row as usize] = false
                } else if self.board[cols as usize][row as usize] == false && sum == 3 {
                    self.board[cols as usize][row as usize] = true
                } 
            }
        }
    }

}


fn main() {
    let mut board = State::new(100);
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
