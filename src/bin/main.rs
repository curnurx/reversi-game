use reversi_game::*;
use std::thread;
use std::time::Duration;
use std::io;
fn main() {
    let mut board = ReversiBoard::new();
    Print::clear();
    Print::logo();
    thread::sleep(Duration::from_secs(3));
    loop {
        Print::clear();
        Print::board(&board);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input = input.split_whitespace();
        let i = input.next().unwrap().parse::<i32>().unwrap() as usize;
        let j = input.next().unwrap().parse::<i32>().unwrap() as usize;
        board.try_set(i,j);
        
    }
}
