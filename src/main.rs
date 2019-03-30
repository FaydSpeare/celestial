#![allow(unused_imports)]

mod types;
#[macro_use]
mod bitboard;
mod position;
mod movegen;

use types::*;
use bitboard::*;
use position::*;
use movegen::*;


fn main() {
    let p = Position::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    p.print();

    white_pawn_moves(&p);

}
