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
    let mut p = Position::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    
    
    p.print();

    let m = Move {
        piece: Piece::W_KNIGHT,
        origin: 1,
        dest: 18,
        promote: Major::NONE,
        flag: MoveFlag::NONE
    };

    p.make_move(m);

    p.print();

}
