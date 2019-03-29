use crate::types::*;
use crate::bitboard::*;

pub const PIECES: [char; 12] = ['P','N','B','R','Q','K','p','n','b','r','q','k'];

pub struct Position {
    pub bit: [Bitboard; 12]
}

impl Position {

    pub fn parse_fen(fen: &str) -> Position {
        let mut p = Position {
        bit: [0b0; 12]
        };

        let mut pos = 0;
        for c in fen.chars(){
            print!("{}", c);

            let mut piece = true;
            

            for i in 0..12 {
                if c == PIECES[i] {
                    piece = false;
                    p.bit[i] |= 1 << pos;
                    println!("{}", pos);
                    break;
                }
            }

            if piece {
                if c != '/' {
                    match c.to_digit(10) {
                        Some(t) => {
                            if pos + t < 64 {
                                pos += t;
                            }
                        }, 
                        _ => ()
                    }
                } 
            } else { if pos + 1 < 64 {pos += 1}}
        }


        p
    }

    pub fn print(&self){
        println!();
        for i in (0..8).rev() {
            print!(" {} ", i+1);
            for j in 0..8 {
                
                let mut p = true;
                for k in 0..12 {
                    if is_set(&self.bit[k], 8*i + j){
                        print!("{} ", PIECES[k]);
                        p = false;
                        break;
                    }
                }
                if p {print!("- ")}
            }
            println!();
        }
        println!("   a b c d e f g h");
    }

}