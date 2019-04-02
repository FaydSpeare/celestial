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

use rand::Rng;
//use std::io;

use types::Square::*;

fn main() {
    println!("hello");
    init();
    let mut p = parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //let mut p = parse_fen_string("R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1");
    print(&p);
    
    /*
    let m = Motion {
        motion: MOVE_INT!(6,7, 0, 0),
        score: 0
    };
    p.do_motion(&m);
    print(&p);
    
    let m = Motion {
        motion: MOVE_INT!(16, 8, 0, 0),
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

    /*
    let mut l = 0;
    perft(3, &mut p, &mut l);
    println!("{}", l);
    */
    
    
    
    /*
    let mut av = 0;
    for i in 0..1 {
        av += simulate(&mut p);
    }
    println!("average: {}", av/1);
    */
    
    
    
    for k in 0..3000 {
        
        let mut list: Vec<Motion> = vec![];
        if p.side_to_move {
            gen_white_moves(&mut list, &p);
        } else {
            gen_black_moves(&mut list, &p);
        }
        let index = run_mcts(&mut p, &list);
        println!("{}", index);
        p.do_motion(&list[index]);
        //print(&p);
        print(&p);

        loop {
            let mut input_text = String::new();
            std::io::stdin().read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            let from = match trimmed.parse::<u32>() {
                Ok(i) => { 
                    println!("your move from input: {}", i);
                    i
                },
                Err(..) => panic!(),
            };
            let mut input_text = String::new();
            std::io::stdin().read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            let to = match trimmed.parse::<u32>() {
                Ok(i) => { 
                    println!("your move to input: {}", i);
                    i
                },
                Err(..) => panic!(),
            };

            let mut list: Vec<Motion> = vec![];
            if p.side_to_move {
                gen_white_moves(&mut list, &p);
            } else {
                gen_black_moves(&mut list, &p);
            }

            let mut b = false;
            for i in 0..list.len() {
                if list[i].from() as u32 == from {
                    if list[i].to() as u32 == to {
                        p.do_motion(&list[i]);
                        b = true;
                        break;
                    }
                }
            }

            if b {
                break;
            }
        }

        print(&p);

    }
    

    
    
    
    
    //play_game(&mut p);
    
    //divide(6, &mut p);



}

fn play_game(pos: &mut Position) {

    let mut rng = rand::thread_rng();

    let mut list: Vec<Motion> = vec![];

    if pos.side_to_move {
        gen_white_moves(&mut list, pos);
    } else {
        gen_black_moves(&mut list, pos);
    }
    let mut i = 0;
    while list.len() != 0 && i < 50{

        pos.do_motion(&list[rng.gen_range(0, list.len())]);
        
        list = vec![];
        if pos.side_to_move {
            gen_white_moves(&mut list, pos);
        } else {
            gen_black_moves(&mut list, pos);
        }

        print(&pos);
        i += 1;
    }

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

