use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;

const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const REVERSI_LOGO_STR: &str = "\n______  _____  _   _  _____ ______  _____  _____ \n| ___ \\|  ___|| | | ||  ___|| ___ \\/  ___||_   _|\n| |_/ /| |__  | | | || |__  | |_/ /\\ `--.   | |\n|    / |  __| | | | ||  __| |    /  `--. \\  | |\n| |\\ \\ | |___ \\ \\_/ /| |___ | |\\ \\ /\\__/ / _| |_\n\\_| \\_|\\____/  \\___/ \\____/ \\_| \\_|\\____/  \\___/ \n\n";
const BOARD_LINE: &str =  "┼───┼───┼───┼───┼───┼───┼───┼───┼";
const BOARD_LABLE : &str = " │ A │ B │ C │ D │ E │ F │ G │ H │";
const DROW: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DCOL: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Stone {
    Black,
    White,
    Empty,
}

pub struct ReversiBoard {
    board: [[Stone; COL_SIZE]; ROW_SIZE],
    turn: Stone,
}

impl ReversiBoard {
    pub fn new() -> ReversiBoard {
        let mut board = [[Stone::Empty; COL_SIZE]; ROW_SIZE];
        board[3][3] = Stone::Black;
        board[4][4] = Stone::Black;

        board[3][4] = Stone::White;
        board[4][3] = Stone::White;

        ReversiBoard { board , turn: Stone::Black }
    }
     
    pub fn get(&self, row: usize, col: usize) -> Option<&Stone> {
        if ReversiBoard::out_of_bound(&row, &col) {
            None
        } else {
            Some(&self.board[row][col])
        }
    }
    // next turn return please 
    pub fn try_set(&mut self, row: usize, col: usize) {
        if self.can_set(row, col) {
            self.board[row][col] = self.my_stone();
            self.reverse(row, col);
            self.turn = self.enemy_stone();
        }
    }
    
    fn check_can_any_set() {
    }

    fn my_stone(&self) -> Stone {
        self.turn
    }

    fn enemy_stone(&self) -> Stone {
        match self.turn {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            _ => panic!("Turn is always either Black or White."),
        }
    }

    fn can_set(&self, row: usize, col: usize) -> bool {
        match self.get(row, col) {
            Some(Stone::Empty) => (),
            _ => return false,
        }
        self.can_reverse(row, col)
    }
    
    fn can_reverse(&self, row: usize, col: usize) -> bool {
        DROW.iter().zip(DCOL.iter()).any(|(&drow, &dcol)| {
            self.can_reverse_one_dir(drow, dcol, row, col) > 0
        })
    }

    fn reverse(&mut self, row: usize, col: usize) {
        DROW.iter().zip(DCOL.iter()).for_each(|(&drow, &dcol)| {
            let skip_counter = self.can_reverse_one_dir(drow, dcol, row, col);
            (1..=skip_counter).for_each(|dist| {
                self.board[(row as i32 + drow * dist) as usize][(col as i32 + dcol * dist) as usize] = self.my_stone();
            })
        });
    }

    fn can_reverse_one_dir(&self, drow: i32, dcol: i32, row: usize, col: usize) -> i32 {
        let skip_counter = Rc::new(RefCell::new(0));
        let mut iter = 
        (1..).map(|dist| self.get((row as i32 + drow * dist) as usize, 
                                  (col as i32 + dcol * dist) as usize))
            .skip_while(|&x| match x {
                Some(&stone) if stone == self.enemy_stone() => {
                    *Rc::clone(&skip_counter).borrow_mut() += 1;
                    true
                },
                _ => false,
            });
        let brace = match iter.next().unwrap() {
            Some(&stone) if stone == self.my_stone() => true,
            _ => false,
        };
        if brace && (*skip_counter.borrow() > 0) {
            *skip_counter.borrow()
        } else {
            0
        }

    }

    fn in_bound(row: &usize, col: &usize) -> bool {
        (0..ROW_SIZE).contains(row) && (0..COL_SIZE).contains(col)
    }

    fn out_of_bound(row: &usize, col: &usize) -> bool {
        !ReversiBoard::in_bound(row, col)
    }
}

pub struct Print;

impl Print {
    pub fn logo() {
        println!("{}", REVERSI_LOGO_STR); 
    }

    pub fn clear() {
        if cfg!(target_os = "windows") {
            Command::new("cls")
        } else {
            Command::new("clear")
        }
        .status().expect("Cannot clear the terminal"); 
    }

    pub fn board(reversi_board: &ReversiBoard) {
        println!("{}", BOARD_LABLE);
        println!(" {}", BOARD_LINE);
        reversi_board.board.iter().enumerate().for_each(|(row_num, &row)| {
            let mut printed_line = String::from(&format!("{}", row_num + 1)); 

            row.iter().for_each(|&stone| {
                printed_line += &format!("│ {} ",
                match stone {
                    Stone::Black => "○",
                    Stone::White => "●",
                    Stone::Empty => " ",
                });
            });
            println!("{}│", printed_line);
            println!(" {}", BOARD_LINE);
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut board = ReversiBoard::new();
        assert_eq!(true, board.can_reverse(3, 5));
        assert_eq!(false, board.can_reverse(2, 3)); 
        assert_eq!(false, board.can_reverse(4, 5)); 
        assert_eq!(true, board.can_reverse(4, 2));

        assert_eq!(true, board.can_set(3, 5));
        assert_eq!(false, board.can_set(4, 3));
        
        board.try_set(3, 5);
        assert_eq!(Some(&Stone::Black), board.get(3, 4));
   }
}


