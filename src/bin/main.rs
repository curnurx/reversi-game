use reversi_game::*;
use std::thread;
use std::time::Duration;
fn main() {
    let mut board = ReversiBoard::new();
    Print::clear();
    Print::logo();
    thread::sleep(Duration::from_secs(3));
    loop {
        Print::clear();
        Print::board(&board);
        if board.is_end() {
            Print::game_end(&board);
        }

        Print::turn(&board); 
        loop {
            match Input::new(&mut board) {
                Ok(_) => break,
                Err(e) => println!("{}", e),
            }
        } 
    }
}
