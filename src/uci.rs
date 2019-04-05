use crate::position::*;
use crate::parsing::*;
use crate::movegen::*;
use crate::motion::*;
use crate::types::*;
use crate::io::*;
use crate::attack::*;

use std::io;
use rand::Rng;
use std::time::{Duration, Instant};


pub const PROM: [char; 4] = ['q','r','b','n'];
pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

pub fn uci_loop(){

    let mut pos = Position::new();
    loop {

        let mut line = String::new();
        let mut quit = false;

        match io::stdin().read_line(&mut line) {
            Ok(t) => {
                let line = line.trim();
                let split =  line.split(" ");
                let words = split.collect::<Vec<&str>>();

                //println!("first: {}", words[0]);
                
                match words[0] {
                    "uci" => {
                        print!("id name Celestial\n");
                        print!("id author Fayd Speare\n");
                        print!("uciok\n");
                    },
                    "isready" => {
                        print!("readyok\n");
                    },
                    "position" => {
                        pos = parse_position(words);
                        //print(&pos);
                    },
                    "ucinewgame" => {
                        pos = parse_position(words);
                        print(&pos);
                    },
                    "go" => {
                        parse_go(&mut pos);
                        print(&pos);
                    },
                    "quit" => {
                        quit = true;
                    },
                    _ => ()
                }

                if quit {
                    break;
                }
            },
            _ => println!("no command")
        } 

        
    }

    


}

pub fn parse_position(words: Vec<&str>) -> Position {

    if words[0] == "ucinewgame" {
        return parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    } else {
        if words[1] == "fen" {
            let fen = format!("{} {} {} {} {} {}", words[2], words[3], words[4], words[5], words[6], words[7]);
            println!("{}", fen);
            return parse_fen_string(&fen);
        } else if words[1] == "startpos" {

            let mut pos = parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

            for i in 3..(words.len()){
                pos.do_motion(&parse_move(&pos, words[i]));
            }

            return pos;
        }
    }

    Position::new()


}

pub fn parse_go(pos: &mut Position) {
    /*

    let mut rng = rand::thread_rng();

    let mut list: Vec<Motion> = vec![];
    gen_legal_moves(&mut list, pos);

    let m = &list[rng.gen_range(0, list.len())];
    */

    let mut k = 0;

    let mut list: Vec<Motion> = vec![];

    let start = Instant::now();
    let i = minimax_top(&mut list, 4, pos, &mut k);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}ms", duration.as_millis());
    
    let m = &list[i];

    println!("nodes: {}", k);
    print!("bestmove ");
    print_move(m);
    println!();
    
    pos.do_motion(m);

}

pub fn parse_move(pos: &Position, m: &str) -> Motion {

    let chars: Vec<char> = m.chars().collect();

    let mut from: usize = 0;
    let mut to: usize = 0;
    
    for i in 0..(FILES.len()) {
        if FILES[i] == chars[0] {
            from += i;
        }
    }

    match chars[1].to_digit(10) {
        Some(d) => from += 8 * (d - 1) as usize,
        _ => panic!()
    }

    for i in 0..(FILES.len()) {
        if FILES[i] == chars[2] {
            to += i;
        }
    }

    match chars[3].to_digit(10) {
        Some(d) => to += 8 * (d - 1) as usize,
        _ => panic!()
    }

    if chars.len() == 5 {
        
        let mut prom = 0;
        for i in 0..(PROM.len()) {
            if PROM[i] == chars[4] {
                prom = i;
            }
        }

        return Motion {
        motion: MOVE_INT!(from as u16, to as u16, prom as u16, Flag::PROMOTION as u16),
        score: 0
        };

    } else {

        if pos.ep == to as i32 {
            return Motion {
            motion: MOVE_INT!(from as u16, to as u16, 0, Flag::ENPASSANT as u16),
            score: 0
            };
        }

        if pos.board[from] == Piece::W_KING as i32 || pos.board[from] == Piece::B_KING as i32 {

            if to - from == 2 || from - to == 2 {
                return Motion {
                motion: MOVE_INT!(from as u16, to as u16, 0, Flag::CASTLING as u16),
                score: 0
                };
            }
        }

        return Motion {
        motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
        score: 0
        };
    }
 
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