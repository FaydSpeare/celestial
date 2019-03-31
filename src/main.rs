#![allow(unused_imports)]

/* EXTERNAL CRATES */
extern crate rand;

#[macro_use]
mod macros;

/* MODULES */
mod types;
mod bitboard;
mod position;
mod movegen;
mod hashkey;
mod parsing;
mod motion;
mod attack;

/* SCOPE */
use types::*;
use bitboard::*;
use position::*;
use movegen::*;
use hashkey::*;
use parsing::*;
use motion::*;
use attack::*;

fn main() {
    init();
    let mut p = parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq c6 0 1");
    print(&p);

    let m = Motion {
        m: 0b1111_1111_0101_1100,
        score: 0
    };

    println!("{}", m.from());

}
