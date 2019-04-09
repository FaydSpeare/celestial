use crate::position::*;
use crate::motion::*;
use crate::movegen::*;
use crate::attack::*;
use crate::io::*;
use crate::evaluation::*;
use crate::types::*;

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub const INF: i32 = 1000000;

pub struct SearchInfo {

    pub start_time: Instant,
    pub stop_time: i32,

    pub depth: i32,
    pub depth_set: i32,

    pub time_set: i32,
    pub moves_to_go: i32,

    pub infinite: bool,

    pub nodes: i32,

    pub quit: bool,
    pub stopped: bool,

    pub fail_high: f32,
    pub fail_high_first: f32

} 

impl SearchInfo {

    pub fn new() -> SearchInfo {

        SearchInfo {
            start_time: Instant::now(),
            stop_time: 0,

            depth: 0,
            depth_set: 0,

            time_set: 0,
            moves_to_go: 0,

            infinite: false,

            nodes: 0,

            quit: false,
            stopped: false,

            fail_high: 0.0,
            fail_high_first: 0.0
        }
    }

    pub fn check_time(&mut self){
        if self.start_time.elapsed().as_millis() > self.stop_time as u128 {
            self.stopped = true;
        }
    }
}

pub fn order(list: &mut Vec<Motion>, index: usize){

    let mut s = -1000000;
    let mut b = index;

    for i in index..(list.len()){
        if list[i].score > s {
            s = list[i].score;
            b = i;
        }
    }

    list.swap(index, b);
}


pub fn alpha_beta(mut alpha: i32, beta: i32, depth: i32, pos: &mut Position, info: &mut SearchInfo, null: bool) -> i32 {
    
    if depth == 0 {
        return quiescence(alpha, beta, pos, info);
        //info.nodes += 1;
        //return eval(pos);
    }

    if info.nodes % 10000 == 0 {
        info.check_time();
    }

    info.nodes += 1;

    if check_3_fold(pos) || pos.fifty >= 100 {
        return 0;
    }

    let mut list: Vec<Motion> = vec![];
    gen_moves(&mut list, pos);

    let mut move_num = 0;
    let mut legal = 0;
    let old_alpha = alpha;
    //let mut best_move = &Motion::new();
    let mut best_index = 0;
    let mut score = -INF;

    for index in 0..(list.len()) {

        // ordering //

        order(&mut list, index);

        //

        //let m = &list[index];

        if !pos.do_motion(&list[index]) {
            continue;
        }

        legal += 1;
        score = -alpha_beta(-beta, -alpha, depth-1, pos, info, true);
        pos.undo_motion();

        if info.stopped {
            return 0;
        }

        if score > alpha {
            if score >= beta {

                if legal == 1 { info.fail_high_first += 1.0}
                info.fail_high += 1.0;

                /*
                if pos.board[*&list[index].to() as usize] == Piece::EMPTY as i32 {
                    pos.search_killers[1][pos.search_ply as usize] = pos.search_killers[0][pos.search_ply as usize];
                    pos.search_killers[0][pos.search_ply as usize] =  list[index].motion;
                }
                */

                return beta;
            }
            alpha = score;

            best_index = index;
        }
    }

    if legal == 0 {
        if pos.side_to_move {
            if is_attacked_by(pos, pos.king_sq[0] as usize, false) {
                 return -1000000 + pos.search_ply;
            } else {
                return 0;
            }
        } else {
            if is_attacked_by(pos, pos.king_sq[1] as usize, true) {
                return -1000000 + pos.search_ply;
            } else {
                return 0;
            }
        }
    }

    if old_alpha != alpha {
        pos.pv_table.insert(pos.pos_key, *(&list[best_index].motion));
    }

    return alpha;
    
}

pub fn think(pos: &mut Position, info: &mut SearchInfo){

    let mut best_move: u16 = 0;
    let mut best_score = -INF;
    let mut pv_moves = 0;
    let mut pv_num = 0;

    clear_for_search(pos, info);

    for depth in 1..=(30) {

        best_score = alpha_beta(-INF, INF, depth, pos, info, true);

        if info.stopped {
            break;
        }

        best_move = match pos.pv_table.get(&pos.pos_key){
            Some(t) => *t,
            _ => panic!("PANIC IN THINKING")
        };
        print!("info score cp {} depth {} nodes {} move ", best_score, depth, info.nodes);
        print_move(&Motion {
            motion: best_move,
            score: 0
        });
        print!(" ordering:{:.2}", info.fail_high_first/info.fail_high);
        print_pv(pos, depth);
        println!();
    }

    pos.do_motion(&Motion {
            motion: best_move,
            score: 0
    });

    print!("bestmove ");
    print_move(&Motion {
            motion: best_move,
            score: 0
    });



}

pub fn print_pv(pos: &mut Position, depth: i32){
    print!(" pv");

    let mut count = 0;
    for i in 0..depth {
        match pos.pv_table.get(&pos.pos_key) {
            Some(t) => {
                let m = &Motion {
            motion: *t,
            score: 0
            };
            pos.do_motion(m);
            print!(" ");
            print_move(m);
            count += 1;
            }
            _ => ()
        }
    }

    for i in 0..count {
        pos.undo_motion();
    }
}


pub fn quiescence(mut alpha: i32, beta: i32, pos: &mut Position, info: &mut SearchInfo) -> i32 {
    if info.nodes % 10000 == 0 {
        info.check_time();
    }
    
    info.nodes += 1;

    if check_3_fold(pos) || pos.fifty >= 100 {
        return 0;
    }

    let score = eval(pos);

    if score >= beta {
        return beta;
    }

    if score > alpha {
        alpha = score;
    }

    let mut list: Vec<Motion> = vec![];
    gen_captures(&mut list, pos);

    let mut move_num = 0;
    let mut legal = 0;
    let old_alpha = alpha;
    //let mut best_move = &Motion::new();
    let mut best_index = 0;
    let mut score = -INF;

    for index in 0..(list.len()) {

        // ordering //

        order(&mut list, index);

        //

        //let m = &list[index];

        if !pos.do_motion(&list[index]) {
            continue;
        }

        legal += 1;
        score = -quiescence(-beta, -alpha, pos, info);
        pos.undo_motion();

        if info.stopped {
            return 0;
        }

        if score > alpha {
            if score >= beta {

                if legal == 1 { info.fail_high_first += 1.0}
                info.fail_high += 1.0;

                return beta;
            }
            alpha = score;

            best_index = index;
        }
    }

    if old_alpha != alpha {
        pos.pv_table.insert(pos.pos_key, *(&list[best_index].motion));
    }

    return alpha;
}


pub fn clear_for_search(pos: &mut Position, info: &mut SearchInfo) {

    pos.search_history = [[0; 64]; 13];
    pos.search_killers = [[0; 100]; 2];

    pos.pv_table = HashMap::new();
    pos.search_ply = 0;

    info.start_time = Instant::now();
    info.stopped = false;
    info.nodes = 0;
    info.fail_high_first = 0.0;
    info.fail_high = 0.0;

}


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