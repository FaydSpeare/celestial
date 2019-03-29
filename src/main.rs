#![allow(unused_imports)]

mod types;
#[macro_use]
mod bitboard;
mod position;

use types::*;
use bitboard::*;
use position::*;


fn main() {

    let p = Position::parse_fen("r1bq1rk1/p4ppp/1pn1pn2/8/2BP4/P1B2N2/1P3PPP/2RQK2R w K - 0 12");

    p.print();

}
