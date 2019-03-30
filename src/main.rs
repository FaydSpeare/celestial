#![allow(unused_imports)]

#[macro_use]
mod macros;

mod types;
mod bitboard;
mod position;
mod movegen;



use types::*;
use bitboard::*;
use position::*;
use movegen::*;



fn main() {

    print_bb(&MASK[2]);

}
