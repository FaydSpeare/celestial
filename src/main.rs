#![allow(unused_imports)]

mod types;
#[macro_use]
mod bitboard;
mod position;

use types::*;
use bitboard::*;
use position::*;


fn main() {
    let p = Position::parse_fen2("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //let p = Position::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    p.print();
}
