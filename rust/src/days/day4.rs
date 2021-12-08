use std::{path::Path, fs::File, io::{self, Read}, fmt::Display, borrow::Borrow};


#[derive(Debug, Clone)]
pub struct BingoBoard {
    pub board: Vec<u8>, 
    pub marked: [bool; 5*5],
}

impl BingoBoard {
    pub fn num_at(&self, x: usize, y: usize) -> u8 {
        self.board[x + y * 5]
    }
    pub fn marked_at(&self, x: usize, y: usize) -> bool {
        self.marked[x + y * 5]
    }
    pub fn sum_of_unmarked(&self) -> u32 {
        self.board.iter()
            .zip(self.marked.iter())
            .filter_map(|(&value, &is_marked)| {
                if !is_marked {
                    Some(value as u32)
                } else {
                    None
                }
            })
            .sum()
    }
    pub fn apply_number(&mut self, num: u8) {
        for i in 0..25 {
            if self.board[i] == num {
                self.marked[i] = true;
            }
        }
    }
    pub fn did_win(&self) -> bool {
        // Check all Columns
        for i in 0..5 {
            if  self.marked_at(i, 0) && 
                self.marked_at(i, 1) && 
                self.marked_at(i, 2) && 
                self.marked_at(i, 3) && 
                self.marked_at(i, 4) {
                    return true;
            }
        }
        // Check all Rows
        for j in 0..5 {
            if  self.marked_at(0, j) && 
                self.marked_at(1, j) && 
                self.marked_at(2, j) && 
                self.marked_at(3, j) && 
                self.marked_at(4, j) {
                    return true;
            }
        }
        false
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_write = "".to_string();
        for i in 0..25 {
            if self.marked[i] {
                to_write = format!("{} *{:2}* ", to_write, self.board[i]);
            } else {
                to_write = format!("{}  {:2}  ", to_write, self.board[i]);
            }
            if i % 5 == 4 {
                f.write_fmt(format_args!("{}\n", to_write))?;
                to_write = "".to_string()
            }
        }

        f.write_fmt(format_args!("{}", to_write))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BingoInfo {
    pub current_number_idx: usize,
    numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoInfo {
    pub fn new(numbers: Vec<u8>, boards: Vec<BingoBoard>) -> Self {
        BingoInfo {
            current_number_idx: 0,
            numbers, 
            boards,
        }
    }
    pub fn remaining_numbers(&self) -> usize {
        self.numbers.len() - self.current_number_idx
    }
    pub fn get_last_number_called(&self) -> u8 {
        self.numbers[self.current_number_idx - 1]
    }
    pub fn pull_next(&mut self) -> u8 {
        let num = self.numbers[self.current_number_idx];
        self.current_number_idx += 1;
        num
    }
    pub fn apply_number(&mut self, num: u8) {
        self.boards.iter_mut().for_each(|board| {
            board.apply_number(num)
        });
    }
    pub fn did_any_board_win(&self) -> Option<BingoBoard> {
        for board in &self.boards {
            if board.did_win() {
                return Some(board.clone());
            }
        }
        None
    }
}

pub struct WinningResults {
    pub board: BingoBoard,
    pub last_num_called: u8,
}

pub fn run_through_bingo_game(bingo_info: &BingoInfo) -> Option<WinningResults> {
    let mut info = bingo_info.clone();
    loop {
        let new_number = info.pull_next();
        println!("Pulled number: {}", new_number);
        info.apply_number(new_number);
        // for board in &info.boards {
        //     println!("Board:\n{}", board);
        // }
        // While check
        if let Some(win_board) = info.did_any_board_win() {
            return Some(WinningResults {
                board: win_board, 
                last_num_called: info.get_last_number_called(),
            });
        }
        if info.remaining_numbers() == 0 {
            return None;
        }
    }
}


pub fn parse_bingo_boards<P: AsRef<Path>>(file_path: P) -> Result<BingoInfo, io::Error> {
    // Open file and read to string
    let mut f = File::open(file_path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;

    let mut lines = data.lines();
    let first_line = lines.next().unwrap();
    let numbers = first_line
        .split(",")
        .flat_map(|s| {
            s.parse::<u8>()
        })
        .collect();
    let boards = lines
        .filter_map(|line| {
            // If empty, don't bother parsing
            if line.is_empty() {
                return None; 
            } else {
                Some(line.split(" ").filter_map(|e| e.parse::<u8>().ok()).collect::<Vec<u8>>())
            }
        })
        .flatten()
        .collect::<Vec<u8>>()
        .chunks_exact(25)// 5 lines per bingo board  
        .map(|board| BingoBoard{ board: board.to_vec(), marked: [false; 25] })
        .collect();
    
    Ok(BingoInfo::new(numbers, boards))
}