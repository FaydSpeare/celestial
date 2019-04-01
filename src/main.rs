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
mod domotion;
mod io;

/* SCOPE */
use types::*;
use bitboard::*;
use position::*;
use movegen::*;
use hashkey::*;
use parsing::*;
use motion::*;
use attack::*;
use domotion::*;
use io::*;

use types::Square::*;

fn main() {
    println!("hello");
    init();
    //let mut p = parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut p = parse_fen_string("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    print(&p);
    /*
    let m = Motion {
        motion: MOVE_INT!(11, 19, 0, 0),
        score: 0
    };
    p.do_motion(&m);
    print(&p);
    
    let m = Motion {
        motion: MOVE_INT!(57, 42, 0, 0),
        score: 0
    };
    p.do_motion(&m);
    print(&p);
    
    let m = Motion {
        motion: MOVE_INT!(4, 11, 0, 0),
        score: 0
    };
    p.do_motion(&m);
    print(&p);
    
    let m = Motion {
        motion: MOVE_INT!(42, 27, 0, 0),
        score: 0
    };
    p.do_motion(&m);
    print(&p);
    
    */

    
    let mut l = 0;
    perft(3, &mut p, &mut l);
    println!("{}", l);
    
    
    
    
    
    
    
    //divide(1, &mut p);



}

fn divide(depth: i32, pos: &mut Position){

    let mut list: Vec<Motion> = vec![];

    if pos.side_to_move {
        gen_white_moves(&mut list, pos);
    } else {
        gen_black_moves(&mut list, pos);
    }
    println!("moves: {}", list.len());

    let mut ell = 0;

    for m in list.iter() {

        let mut l = 0;

        if !pos.do_motion(&m) {
            continue;
        }
        println!();
        println!();
        perft(depth-1, pos, &mut l);

        pos.undo_motion();
        print_move(&m);
        println!(" {}", l);
        ell += l;
    }

    println!(" all : {}", ell);
}

fn perft(depth: i32, pos: &mut Position, leaves: &mut i32){
    //println!("{}", depth);
    if depth == 0 {
        *leaves += 1;
        //println!("{}", leaves);
        return;
    }

    

    let mut list: Vec<Motion> = vec![];

    if pos.side_to_move {
        gen_white_moves(&mut list, pos);
    } else {
        gen_black_moves(&mut list, pos);
    }



    for m in list.iter() {
        //println!("pre");
        if !pos.do_motion(&m) {
            continue;
        }
        //print(&pos);
        //print_move(m);
        //println!();
        
        //println!("post");
        perft(depth-1, pos, leaves);
        //println!("post perft");
        pos.undo_motion();
        //println!("post undo");
    }

}

