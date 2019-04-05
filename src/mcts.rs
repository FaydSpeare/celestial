use crate::types::*;
use crate::bitboard::*;
use crate::position::*;
use crate::movegen::*;
use crate::hashkey::*;
use crate::parsing::*;
use crate::motion::*;
use crate::attack::*;
use crate::domotion::*;
use crate::io::*;

use rand::Rng;

pub fn run_mcts(pos: &mut Position, list: &Vec<Motion>) -> usize {
    let mut rng = rand::thread_rng();

    let mut traversals = 0;
    let mut scores = vec![];

    for i in list.iter(){
        scores.push(0);
    }

    for n in 0..50000 {
        for i in 0..(list.len()) {

            pos.do_motion(&list[i]);

            scores[i] += simulate(pos);

            pos.undo_motion();

        }
        traversals += 1;
    }

    //println!("{:?}", scores);

    let scores: Vec<i32> = scores.iter().map(|x| x/50000).collect();

    //println!("{:?}", scores);


    

    if pos.side_to_move {
        let mut best = -9999;
        let mut index = 0;

        for i in 0..scores.len() {
            if scores[i] > best {
                best = scores[i];
                index = i;
            }
        }
        println!("best score: {}", best);
        return index;

    }
    else  {
        let mut best = 9999;
        let mut index = 0;

        for i in 0..scores.len() {
            if scores[i] < best {
                best = scores[i];
                index = i;
            }
        }
        println!("best score: {}", best);
        return index;

    }
    
}

pub fn simulate(pos: &mut Position) -> i32{

    let start_ply = pos.ply;

    let mut rng = rand::thread_rng();

    let mut list: Vec<Motion> = vec![];

    if pos.side_to_move {
        gen_white_moves(&mut list, pos);
    } else {
        gen_black_moves(&mut list, pos);
    }

    let mut i = 0;

    while i < 10 && list.len() > 0 {

        pos.do_motion(&list[rng.gen_range(0, list.len())]);
        
        list = vec![];

        if pos.side_to_move {
            gen_white_moves(&mut list, pos);
        } else {
            gen_black_moves(&mut list, pos);
        }

        i += 1;
    }
    

    let eval = evaluate(pos);

    let mut i = pos.ply - start_ply;

    while i > 0 {
        pos.undo_motion();
        i -= 1;
    }

    return eval;
}

pub fn evaluate(pos: &Position) -> i32 {
    let mut eval = 0;

    eval += pos.material[0];

    eval -= pos.material[1];

    if pos.side_to_move {
        if is_attacked_by(pos, pos.king_sq[1] as usize, true) {
            eval += 1000000;
        }

    } else {
        if is_attacked_by(pos, pos.king_sq[0] as usize, false) {
            eval -= 1000000;
        }
    }
    
    //println!("{}", eval);
    eval
}