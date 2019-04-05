use crate::position::*;
use crate::motion::*;
use crate::movegen::*;
use crate::attack::*;
use crate::io::*;


use std::time::{Duration, Instant};


pub fn iterative_deepening(time: u128, pos: &mut Position) {
    let start = Instant::now();
    let mut duration = start.elapsed();

    let mut k = 0;

    let mut motion: &Motion = &Motion::new();

    let mut list: Vec<Motion> = vec![];

    let mut depth = 1;

    while duration.as_millis() < time {

        list = vec![];
        k = 0;

        let index = minimax_top(&mut list, depth, pos, &mut k);
        motion = &list[index];

        depth += 1;
        duration = start.elapsed();

    }

    println!("Time elapsed in expensive_function() is: {:?}ms", duration.as_millis());
    println!("nodes: {}", k);
    print!("bestmove ");
    print_move(motion);
    println!();
    
    pos.do_motion(motion);


}

pub fn minimax_top(list: &mut Vec<Motion>, depth: i32, pos: &mut Position, k: &mut i32) -> usize {
    gen_legal_moves(list, pos);

    if pos.side_to_move {

        let mut best_index = 0;
        let mut best_score = -99999;

        for i in 0..(list.len()) {

            pos.do_motion(&list[i]);

            let score = minimax(depth-1, pos, false, k);
            if score > best_score {
                best_score = score;
                best_index = i;
            }

            pos.undo_motion();
        }
        println!("best score: {}", best_score);
        return best_index;

    } else {

        let mut best_index = 0;
        let mut best_score = 99999;

        for i in 0..(list.len()) {

            pos.do_motion(&list[i]);

            let score = minimax(depth-1, pos, true, k);
            if score < best_score {
                best_score = score;
                best_index = i;
            }

            pos.undo_motion();
        }
        println!("best score: {}", best_score);
        return best_index;
        

    }
}

pub fn minimax(depth: i32, pos: &mut Position, player: bool, k: &mut i32) -> i32 {

    if check_3_fold(pos){
        return 0;
    }

    if depth == 0 {
        *k += 1;
        return eval(pos);
    }

    let mut list: Vec<Motion> = vec![];
    gen_legal_moves(&mut list, pos);

    if player {

        let mut best = -999999;
        for m in list.iter() {
            pos.do_motion(&m);
            let score = minimax(depth-1, pos, !player, k);
            if score > best {
                best = score;
            }
            pos.undo_motion();
        }
        return best;

    } else {

        let mut best = 999999;
        for m in list.iter() {
            pos.do_motion(&m);
            let score = minimax(depth-1, pos, !player, k);
            if score < best {
                best = score;
            }
            pos.undo_motion();
        }
        return best;

    }

}

pub fn eval(pos: &Position) -> i32 {
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

pub fn check_3_fold(pos: &Position) -> bool {
    let key = pos.pos_key;
    let mut count = 0;

    for i in pos.history.iter() {
        if key == i.pos_key {
            count += 1;
        }
    }

    if count > 1 {
        return true;
    }

    false
}