#![allow(unused_imports)]

/* EXTERNAL CRATES */
extern crate rand;
extern crate bitintr;

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


    println!("{}", is_attacked_by_knight(&p, 16, true));

    print_bb(&KN_MOVES[16]);

    print_bb(&sliding_attacks( 38, p.colour_bb[2]));

    let m = Motion {
        motion: MOVE_INT!(0,63,Promotee::BISHOP as u16,Flag::PROMOTION as u16),
        score : 0
    };

    println!("{} {} {} {}", m.from(), m.to(), m.is_prom_queen(), m.is_enpassant());

}

