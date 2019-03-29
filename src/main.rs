#![allow(unused_imports)]

mod types;
#[macro_use]
mod bitboard;
mod position;

use types::*;
use bitboard::*;
use position::*;


fn main() {

    let p = Position::parse_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");

    p.print();

}
