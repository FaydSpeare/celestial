use crate::position::*;
use crate::parsing::*;
use crate::movegen::*;
use crate::motion::*;
use crate::types::*;
use crate::io::*;
use crate::attack::*;
use crate::search::*;
use crate::evaluation::*;

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
                        print(&pos);
                    },
                    "ucinewgame" => {
                        pos = parse_position(words);
                        print(&pos);
                    },
                    "go" => {
                        parse_go(&mut pos, words);
                        print(&pos);
                    },
                    "quit" => {
                        quit = true;
                    },
                    "eval" => {
                        println!("{}", eval(&pos));
                    },
                    "moves" => {
                        let mut list: Vec<Motion> = vec![];
                        gen_moves(&mut list, &pos);
                        println!("{}", list.len());
                        for m in list {
                            pos.do_motion(&m);
                            print_move(&m);
                            println!("{}", eval(&pos));
                            pos.undo_motion();
                        }
                        
                        for i in 0..64 {
                            println!("({} -> {})", i, transform_white(i))
                        }
                    }
                    "think" => {
                        let mut si = SearchInfo::new();
                        si.stop_time = 10000;
                        think(&mut pos, &mut si , 6);    
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

pub fn parse_go(pos: &mut Position, words: Vec<&str>) {

    let mut si = SearchInfo::new();
    let mut white_time = words[2].parse::<i32>().unwrap() / 30;
    let mut black_time = words[4].parse::<i32>().unwrap() / 30;

    if white_time*30 < 2000 {
        white_time /= 5;
    } else if white_time*30 < 20000 {
        white_time /= 2;
    }
   
    if black_time*30 < 2000 {
        black_time /= 5;
    } else if black_time*30 < 20000 {
        black_time /= 2;
    }

    

    if pos.side_to_move {
        si.stop_time = white_time;
    } else {
        si.stop_time = black_time;
    }

    if words.len() == 9 {
        let white_time = words[6].parse::<i32>().unwrap() * 3/4;
        let black_time = words[8].parse::<i32>().unwrap() * 3/4;
        if pos.side_to_move {
            si.stop_time += white_time;
        } else {
            si.stop_time += black_time;
        }
    }
    think(pos, &mut si, 30);

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
        _ => panic!("panic on parse move 1")
    }

    for i in 0..(FILES.len()) {
        if FILES[i] == chars[2] {
            to += i;
        }
    }

    match chars[3].to_digit(10) {
        Some(d) => to += 8 * (d - 1) as usize,
        _ => panic!("panic on parse move 1")
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
            score: 0,
            capture: false
        };

    } else {

        if pos.ep == to as i32 {
            return Motion {
                motion: MOVE_INT!(from as u16, to as u16, 0, Flag::ENPASSANT as u16),
                score: 0,
                capture: false
            };
        }

        if pos.board[from] == Piece::W_KING as i32 || pos.board[from] == Piece::B_KING as i32 {

            if to - from == 2 || from - to == 2 {
                return Motion {
                    motion: MOVE_INT!(from as u16, to as u16, 0, Flag::CASTLING as u16),
                    score: 0,
                    capture: false
                };
            }
        }

        return Motion {
            motion: MOVE_INT!(from as u16, to as u16, 0, Flag::NONE as u16),
            score: 0,
            capture: false
        };
    }
 
}

