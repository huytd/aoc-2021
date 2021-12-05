use crate::lib::read_lines_from;

#[derive(Debug)]
struct Board {
    data: Vec<Vec<u32>>,
    mark: Vec<Vec<u32>>
}

impl Board {
    pub fn new() -> Self {
        Self {
            data: vec![],
            mark: vec![vec![0; 5]; 5]
        }
    }

    pub fn print(&self) {
        for row in 0..5 {
            for col in 0..5 {
                if self.mark[row][col] == 1 {
                    print!("{:2} ", self.data[row][col]);
                } else {
                    print!(" - ");
                }
            }
            println!("");
        }
    }

    pub fn print_full(&self) {
        for row in 0..5 {
            for col in 0..5 {
                print!("{:2} ", self.data[row][col]);
            }
            println!("");
        }
    }

    pub fn push_row(&mut self, row: Vec<u32>) {
        self.data.push(row);
    }

    pub fn find_number(&self, num: u32) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for row in 0..5 {
            for col in 0..5 {
                if self.data[row][col] == num {
                    result.push((row, col));
                }
            }
        }
        result
    }

    pub fn mark_number_at(&mut self, row: usize, col: usize) {
        self.mark[row][col] = 1;
    }

    pub fn is_board_win(&self) -> bool {
        // check rows
        for row in &self.mark {
            let row_sum: u32 = row.iter().sum();
            if row_sum >= 5 {
                return true;
            }
        }
        // check cols
        for col in 0..5 {
            let col_sum: u32 = self.mark.iter().map(|row| row[col]).sum();
            if col_sum >= 5 {
                return true;
            }
        }
        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.mark[row][col] == 0 {
                    sum += self.data[row][col];
                }
            }
        }
        sum
    }
}

#[derive(Debug)]
struct Input {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

fn read_input() -> Input {
    let mut lines = read_lines_from("./data/day04.txt").unwrap();
    let draws: Vec<u32> = lines.next().unwrap().unwrap().split(',').map(|token| token.parse::<u32>().unwrap()).collect();
    let mut boards: Vec<Board> = vec![];
    for chunk in lines.collect::<Vec<_>>().chunks(6) {
        let mut board: Board = Board::new();
        let rows = chunk[1..].iter().map(|r| r.as_ref().unwrap());
        for row in rows {
            let row_data: Vec<u32> = row.trim().split_whitespace().filter(|c| !c.is_empty()).map(|c| c.parse::<u32>().unwrap()).collect();
            board.push_row(row_data);
        }
        boards.push(board);
    }
    Input { draws, boards }
}

pub fn part_one() {
    let Input { draws, mut boards } = read_input();
    for num in draws {
        for board in &mut boards {
            let found = board.find_number(num);
            for (row, col) in found {
                board.mark_number_at(row, col);
                if board.is_board_win() {
                    let sum = board.unmarked_sum();
                    println!("Day 04 - Part 01 = {}", sum * num);
                    return;
                }
            }
        }
    }
}

pub fn part_two() {
    let Input { draws, mut boards } = read_input();
    for num in draws {
        for index in 0..boards.len() {
            let found = boards[index].find_number(num);
            for (row, col) in found {
                boards[index].mark_number_at(row, col);
            }
            if boards[index].is_board_win() {
                let mut win_count = 0;
                for pi in 0..boards.len() {
                    if boards[pi].is_board_win() {
                        win_count += 1;
                    }
                }
                if win_count == boards.len() {
                    let sum = boards[index].unmarked_sum();
                    println!("Day 04 - Part 02 = {}", sum * num);
                    return;
                }
            }
        }
    }
}
