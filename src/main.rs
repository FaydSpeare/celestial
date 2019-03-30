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



fn main() {

    init();

}
