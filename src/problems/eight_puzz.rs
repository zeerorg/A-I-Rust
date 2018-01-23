use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Puzzle {
    pub puzzle: [[i32; 3] ; 3],
}
impl Puzzle {
    pub fn new(puzzle: [[i32; 3] ; 3]) -> Puzzle {
        Puzzle { 
            puzzle: puzzle,
        }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in &(*self).puzzle {
            for cell in row {
                output += &cell.to_string();
            }
        }
        write!(f, "{}", output)
    }
}

pub fn from_up(x: &Puzzle) -> Puzzle {
    let mut new_puzzle = x.puzzle.clone();
    for row in 1..3 {
        for cell in 0..3 {
            if x.puzzle[row][cell] == 0 {
                new_puzzle[row][cell] = new_puzzle[row-1][cell];
                new_puzzle[row-1][cell] = 0;
            }
        }
    }
    Puzzle {
        puzzle: new_puzzle
    }
}

pub fn from_down(x: &Puzzle) -> Puzzle {
    let mut new_puzzle = x.puzzle.clone();
    for row in 0..2 {
        for cell in 0..3 {
            if x.puzzle[row][cell] == 0 {
                new_puzzle[row][cell] = new_puzzle[row+1][cell];
                new_puzzle[row+1][cell] = 0;
            }
        }
    }
    Puzzle {
        puzzle: new_puzzle
    }
}

pub fn from_left(x: &Puzzle) -> Puzzle {
    let mut new_puzzle = x.puzzle.clone();
    for row in 0..3 {
        for cell in 1..3 {
            if x.puzzle[row][cell] == 0 {
                new_puzzle[row][cell] = new_puzzle[row][cell-1];
                new_puzzle[row][cell-1] = 0;
            }
        }
    }
    Puzzle {
        puzzle: new_puzzle
    }
}

pub fn from_right(x: &Puzzle) -> Puzzle {
    let mut new_puzzle = x.puzzle.clone();
    for row in 0..3 {
        for cell in 0..2 {
            if x.puzzle[row][cell] == 0 {
                new_puzzle[row][cell] = new_puzzle[row][cell+1];
                new_puzzle[row][cell+1] = 0;
            }
        }
    }
    Puzzle {
        puzzle: new_puzzle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // 8 Puzzle Problem

    #[test]
    fn move_test() {
        let start_puzzle = Puzzle::new([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 0]
        ]);
        let start_up = Puzzle::new([
            [1, 2, 3],
            [4, 5, 0],
            [7, 8, 6]
        ]);
        let start_left = Puzzle::new([
            [1, 2, 3],
            [4, 5, 6],
            [7, 0, 8]
        ]);
        assert_eq!(from_right(&start_puzzle), start_puzzle);
        assert_eq!(from_down(&start_puzzle), start_puzzle);
        assert_eq!(from_up(&start_puzzle), start_up);
        assert_eq!(from_left(&start_puzzle), start_left);
    }
}