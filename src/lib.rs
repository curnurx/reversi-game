use std::io;
use std::cmp::Ordering;
use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;

const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const REVERSI_LOGO_STR: &str = 
"\n______  _____  _   _  _____ ______  _____  _____ \
\n| ___ \\|  ___|| | | ||  ___|| ___ \\/  ___||_   _|\
\n| |_/ /| |__  | | | || |__  | |_/ /\\ `--.   | |\
\n|    / |  __| | | | ||  __| |    /  `--. \\  | |\
\n| |\\ \\ | |___ \\ \\_/ /| |___ | |\\ \\ /\\__/ / _| |_\
\n\\_| \\_|\\____/  \\___/ \\____/ \\_| \\_|\\____/  \\___/ \n\n";
const BOARD_LINE: &str =  "┼───┼───┼───┼───┼───┼───┼───┼───┼";
const BOARD_LABLE : &str = " │ A │ B │ C │ D │ E │ F │ G │ H │";
const DROW: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DCOL: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Stone {
    Black,
    White,
    Empty(bool),
}

pub struct ReversiBoard {
    board: [[Stone; COL_SIZE]; ROW_SIZE],
    turn: Stone,
    game_end: bool,
}

impl ReversiBoard {
    pub fn new() -> ReversiBoard {
        let mut board = [[Stone::Empty(false); COL_SIZE]; ROW_SIZE];
        board[3][3] = Stone::Black;
        board[4][4] = Stone::Black;

        board[3][4] = Stone::White;
        board[4][3] = Stone::White;

        let mut ret = ReversiBoard { board , turn: Stone::Black, game_end: false };
        ret.mark();
        ret
    }
     
    pub fn get(&self, row: usize, col: usize) -> Option<&Stone> {
        if ReversiBoard::out_of_bound(&row, &col) {
            None
        } else {
            Some(&self.board[row][col])
        }
    }
    // next turn return please 
    pub fn try_set(&mut self, row: usize, col: usize) -> bool {
        if self.can_set(row, col) {
            self.board[row][col] = self.my_stone();
            self.reverse(row, col);
            self.change_turn();
            true
        } else {
            false
        }
    }

    fn get_turn(&self) -> Stone {
        self.turn
    }

    pub fn is_end(&self) -> bool {
        self.game_end
    }

    fn get_winner_string(&self) -> String {
        if self.is_end() {
            let mut black_cnt = 0;
            let mut white_cnt = 0;
            for i in 0..ROW_SIZE {
                for j in 0..COL_SIZE {
                    match self.board[i][j] {
                        Stone::Black => black_cnt += 1,
                        Stone::White => white_cnt += 1,
                        _ => (),
                    };
                }
            }
            match black_cnt.cmp(&white_cnt) {
                Ordering::Less => String::from("WHITE"),
                Ordering::Equal => String::from("DRAW"),
                Ordering::Greater => String::from("BLACK"),
            }
        } else {
            String::from("DRAW")
        }
    }

    fn change_turn(&mut self) {
        self.turn = self.enemy_stone();
        if self.mark() == 0 {
            self.turn = self.enemy_stone();
        };
        if self.mark() == 0 {
            self.game_end = true; 
        }
    }

    fn mark (&mut self) -> usize {
        let mut mark_cnt = 0;
        for i in 0..ROW_SIZE {
            for j in 0..COL_SIZE {
                if self.can_set(i, j) {
                    mark_cnt += 1;
                }
            }
        }
        mark_cnt
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

    fn can_set(&mut self, row: usize, col: usize) -> bool {
        match self.get(row, col) {
            Some(Stone::Empty(_)) => (),
            _ => return false,
        }
        if self.can_reverse(row, col) {
            self.board[row][col] = Stone::Empty(true);
            true
        } else {
            self.board[row][col] = Stone::Empty(false);
            false
        }
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

pub struct Input;

impl Input {
    pub fn new(board: &mut ReversiBoard) -> Result<(),&'static str> {
        println!("Input [1-8] [A-H] you want to set Stone\n
Example) 4 F");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input = input.split_whitespace();

        let row = match input.next() {
            Some(n) => match n.parse::<i32>() {
                Ok(v) if (1..=8).contains(&v) => v,
                _ => return Err("[!] Please input [1-8] number at first parameter"),
            },
            None => return Err("[!] Please input two parameter"),
        };
        let col = match input.next() {
            Some(n) => match n.parse::<char>() {
                Ok(v) if ('A'..='H').contains(&v) => v,
                _ => return Err("[!] Please input [A-H] character at second parameter"),
            },
            None => return Err("[!] Please input two parameter"),
        };
        let col = col as u8 - 65;
        
        if !board.try_set((row - 1) as usize, col as usize) {
            Err("[!] Cannot put stone this")
        } else {
            Ok(())
        }
    }

}

pub struct Print;

impl Print {
    pub fn logo() {
        println!("{}", REVERSI_LOGO_STR); 
    }
    
    pub fn game_end(board: &ReversiBoard) {
        println!("== GAME IS END ==");
        println!("WINNER IS {}", board.get_winner_string());
    }

    pub fn turn(board: &ReversiBoard) {
        let (string, stone) = match board.get_turn() {
            Stone::Black => ("BLACK","○"),
            Stone::White => ("WHITE","●"),
            _ => panic!("turn is always either black or white"),
        };
        
        println!("=== It's {} ({}) turn! ===", string, stone); 
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
                    Stone::Empty(true) => " ",
                    Stone::Empty(false) => "X",
                });
            });
            println!("{}│", printed_line);
            println!(" {}", BOARD_LINE);
        });
    }
}


