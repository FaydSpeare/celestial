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
mod mcts;
mod uci;
mod search;
mod evaluation;

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
use mcts::*;
use uci::*;
use search::*;
use evaluation::*;

use types::Piece::*;

use rand::Rng;
use std::time::{Duration, Instant};

use types::Square::*;

fn main() {
    init();

    //uci_loop();

    let mut list: [[i32; 13]; 13] = [[0; 13]; 13];

    let values = [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600, 0];

    for i in 0..13 {
        for j in 0..13 {

            let attack = match i {
                0 => W_PAWN,
                1 => W_KNIGHT,
                2 => W_BISHOP, 
                3 => W_ROOK, 
                4 => W_QUEEN, 
                5 => W_KING, 
                6 => B_PAWN, 
                7 => B_KNIGHT, 
                8 => B_BISHOP, 
                9 => B_ROOK, 
                10 => B_QUEEN, 
                11 => B_KING, 
                12 => EMPTY,
                _ => panic!()
            };

            let defend = match j {
                0 => W_PAWN,
                1 => W_KNIGHT,
                2 => W_BISHOP, 
                3 => W_ROOK, 
                4 => W_QUEEN, 
                5 => W_KING, 
                6 => B_PAWN, 
                7 => B_KNIGHT, 
                8 => B_BISHOP, 
                9 => B_ROOK, 
                10 => B_QUEEN, 
                11 => B_KING, 
                12 => EMPTY,
                _ => panic!()
            };

            let score = values[j] - values[i] + values[j]/10;
            println!("{:?} attacks {:?} = {}", attack, defend, score);

            list[i][j] = score;
        }
    }

    println!("{}", list[0][10]);
    println!("{:?}", list);
    

}


fn divide(depth: i32, pos: &mut Position){

    let mut list: Vec<Motion> = vec![];

    if pos.side_to_move {
        gen_white_moves(&mut list, pos);
    } else {
        gen_black_moves(&mut list, pos);
    }
    //println!("moves: {}", list.len());

    let mut ell = 0;

    for m in list.iter() {

        let mut l = 0;

        if !pos.do_motion(&m) {
            continue;
        }
        //println!();
        //println!();
        perft(depth-1, pos, &mut l);

        pos.undo_motion();
        //print_move(&m);
        //println!(" {}", l);
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

