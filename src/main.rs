#![allow(unused_imports)]

extern crate rand;



#[macro_use]
mod macros;

mod types;
mod bitboard;
mod position;
mod movegen;
mod hashkey;
mod parsing;

use types::*;
use bitboard::*;
use position::*;
use movegen::*;
use hashkey::*;
use parsing::*;

fn main() {
    init();

    let mut p = parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq c6 0 1");
    print(&p);
}
