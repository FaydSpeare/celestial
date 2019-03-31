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
    let mut p = parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //let mut p = parse_fen_string("rnbqkb1r/pp1p1pPp/8/2p1pP2/iP1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
    print(&p);


    println!("{}", is_attacked_by_knight(&p, 16, true));

    print_bb(&KN_MOVES[16]);

    //print_bb(&sliding_attacks( 38, p.colour_bb[2]));

    let m = Motion {
        motion: MOVE_INT!(0,63,Promotee::BISHOP as u16,Flag::PROMOTION as u16),
        score : 0
    };

    println!("{} {} {} {}", m.from(), m.to(), m.is_prom_queen(), m.is_enpassant());

    let mut list: Vec<Motion> = vec![];
    //gen_black_pawn_moves(&mut list, &p);
    //gen_black_bishop_moves(&mut list, &p);

    print_bb(&sliding_attacks(0, p.colour_bb[Colour::BOTH as usize]));
    println!("{}", list.len());

    for m in list.iter() {
        println!("{} {} {} {}", m.from(), m.to(), m.is_prom_queen(), m.is_enpassant());
    }

}

