use crate::types::*;
use crate::bitboard::*;

pub const PIECES: [char; 12] = ['P','N','B','R','Q','K','p','n','b','r','q','k'];

pub const CASTLING: [char; 4] = ['K','Q','k','q'];

pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

pub struct Position {
    pub bit: [Bitboard; 12],
    pub side: bool,
    pub c_rights: u16,
    pub ply: i32,
    pub fifty: i32,
    pub ep: i32
}

impl Position {

    pub fn parse_fen(fen: &str) -> Position {
        let mut p = Position {
            bit: [0b0; 12],
            side: true,
            c_rights: 0,
            ply: 0,
            fifty: 0,
            ep: -1
        };

        let mut pos = 0;
        let mut pieces = true;
        let mut counter = 0;
        let mut ep: i32 = -1;

        for c in fen.chars(){
            //println!("{}", c);

            if pieces {
                let mut piece = true;
            
                for i in 0..12 {
                    if c == PIECES[i] {
                        piece = false;
                        p.bit[i] |= 1 << ((63-pos)/8)*8 + (7-((63-pos)%8));
                        break;
                    }
                }

                if piece {
                    if c != '/' {
                        match c.to_digit(10) {
                            Some(t) => {
                                if pos + t < 64 {
                                    pos += t;
                                } else {pieces = false}
                            }, 
                            _ => ()
                        }
                    } 
                } else { if pos + 1 < 64 {pos += 1} else {pieces = false}}
            }
            else {
                if c == ' ' {
                    counter += 1;
                    continue
                }
                else if c == 'w' && counter == 1 {
                    p.side = true;
                }
                else if c == 'b' && counter == 1 {
                    p.side = false;
                }
                else if c == 'k' && counter == 2 {
                    p.c_rights |= 0b0010;
                }
                else if c == 'q' && counter == 2 {
                    p.c_rights |= 0b0001;
                }
                else if c == 'K' && counter == 2 {
                    p.c_rights |= 0b1000;
                }
                else if c == 'Q' && counter == 2 {
                    p.c_rights |= 0b0100;
                } else {
                    if counter == 3 {
                        if c == '-' {
                            continue
                        } 
                        let mut n = true;
                        for m in 0..8 {
                            if FILES[m] == c {
                                ep += 1 + (m as i32);
                                n = false;
                                break;
                            }
                        }
                        if n {
                            p.ep= match c.to_digit(10) {
                                Some(t) => {
                                    ep += 8*((t as i32)-1);
                                    ep
                                }, 
                                _ => panic!()
                            }
                        }
                    }
                    else if counter == 4 {
                        p.fifty= match c.to_digit(10) {
                            Some(t) => {
                                t as i32
                            }, 
                            _ => panic!()
                        }
                    }
                    else if counter == 5 {
                        p.ply = match c.to_digit(10) {
                            Some(t) => {
                                (2*t - if p.side{2}else{1}) as i32
                            }, 
                            _ => panic!()
                        }
                        
                    }
                }
            }
        }


        p
    }

    pub fn print(&self){
        println!("\nPosition:\n");
        for i in (0..8).rev() {
            print!(" {} | ", i+1);
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
        println!("   |________________");
        println!("     a b c d e f g h");
        println!("\nto__play: {}", if self.side {"white"} else {"black"});
        print!("castling: ");
        for i in 0..4 {
            if (self.c_rights & (1 << i)) != 0 {
                print!("{}",CASTLING[i]);
            }
        }
        println!();
        println!("_____ply: {}", self.ply);
        println!("___fifty: {}", self.fifty);
        println!("______ep: {}", self.ep);
    }

}