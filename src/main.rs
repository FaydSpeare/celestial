#![allow(unused_imports)]

mod types;
mod bitboard;
mod position;

use types::*;
use bitboard::*;
use position::*;


fn main() {

    let pos = Position {
        bit: 0b0
    };

    pos.print();

}
