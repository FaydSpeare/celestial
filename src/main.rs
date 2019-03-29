#![allow(unused_imports)]

mod types;
#[macro_use]
mod bitboard;
mod position;

use types::*;
use bitboard::*;
use position::*;


fn main() {

    let p = Position::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    for i in p.bit.iter() {
        display(i);
    }

    let pos = Position {
        bit: [0b0; 12]
    };

    p.print();

}
