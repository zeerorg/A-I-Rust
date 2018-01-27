use std::ops::Mul;
use std::convert::Into;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Moves {
    Blank = 2,
    X = 3,
    O = 5
}

impl Mul for Moves {
    type Output = i32;

    fn mul(self, rhs: Self) -> i32 {
        return self as i32 * rhs as i32;
    }

}

impl Into<i32> for Moves {
    fn into(self) -> i32 {
        match self {
            Moves::Blank => 2,
            Moves::X => 3,
            Moves::O => 5
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    board: [Moves; 10],
    turn: i32
}

impl Board {
    /// Returns a new `Board` instance with all blanks with value 2, and turn set to 0
    pub fn new() -> Board {
        Board {
            board: [Moves::Blank; 10],
            turn: 0
        }
    }
    
    /// Returns `5` if center of `board` is blank, otherwise returns any blank non-corner square
    pub fn make_2(&self) -> Option<i32> {
        for place in &[5, 2, 4, 6, 8] {
            if self.board[*place] == Moves::Blank {
                return Some(*place as i32);
            }
        }
        return None;
    }

    /// Returns 0 if `p` cannot win else return the block where `p` can win
    /// `p` can be Moves::X or Moves::O
    pub fn poss_win(&self, p: Moves) -> i32 {
        let to_check = p * p * 2;
        // Check all rows
        for rc in &[1, 4, 7] {
            let positions: Vec<i32> = vec![*rc, *rc+1, *rc+2];
            if self.get_product(&positions) == to_check {
                return self.get_blank(&positions);
            }
        }

        // Check all columns
        for col in 1..4 {
            let positions: Vec<i32> = vec![col, col+3, col+6];
            if self.get_product(&positions) == to_check {
                return self.get_blank(&positions);
            }
        }

        // Check 2 diagonals
        if self.get_product(&[1, 5, 9]) == to_check { 
            return self.get_blank(&[1, 5, 9]);
        }

        if self.get_product(&[3, 5, 7]) == to_check { 
            return self.get_blank(&[1, 5, 9]);
        }
        return 0;
    }

    pub fn go(&mut self, pos: &i32, mv: Moves) {
        self.board[*pos as usize] = mv;
        self.turn += 1;
    }

    /// Function which takes a board and positions and returns the blank position or 0
    fn get_blank(&self, positions: &[i32]) -> i32 {
        positions.iter().fold(0, |acc, &pos| if self.board[pos as usize] == Moves::Blank { pos as i32 } else { acc })
    }

    /// returns product of positions
    fn get_product(&self, positions: &[i32]) -> i32 {
        positions.iter().fold(1, |acc, &pos| self.board[pos as usize] as i32 * acc)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output += "| ";
        for point in &self.board {
            output += &(*point as i32).to_string();
            output += " | ";

            if *point as i32 % 3 == 0 {
                output += "\n";
            }
        }
        write!(f, "{}", output)
    }
}
