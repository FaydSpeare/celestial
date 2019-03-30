use crate::types::*;
use crate::bitboard::*;

pub const PIECES: [char; 12] = ['P','N','B','R','Q','K','p','n','b','r','q','k'];

pub const CASTLING: [char; 4] = ['K','Q','k','q'];

pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

pub struct Position {
    pub piece_boards: [Bitboard; 12],
    pub side_boards: [Bitboard; 3],
    pub side: bool,
    pub c_rights: u16,
    pub ply: i32,
    pub fifty: i32,
    pub ep: i32
}

impl Position {

    pub fn make_move(&mut self, m: Move){
        let p = m.piece as usize;
        let o = m.origin as usize;
        let d = m.dest as usize;
        match m.flag {
            MoveFlag::NONE => {
                self.piece_boards[p] ^= MASK[o];
                self.piece_boards[p] |= MASK[d];

                self.side_boards[WHITE] ^= MASK[o];
                self.side_boards[WHITE] |= MASK[d];

                self.side_boards[BOTH] = self.side_boards[WHITE] | self.side_boards[BLACK];
            },
            MoveFlag::PROMOTION => {

            },
            MoveFlag::CASTLING => {

            },
            MoveFlag::ENPASSANT => {

            }
        }
    }

    pub fn parse_fen(fen: &str) -> Position {

        let mut p = Position {
            piece_boards: [0b0; 12],
            side_boards: [0b0; 3],
            side: true,
            c_rights: 0,
            ply: 0,
            fifty: 0,
            ep: -1
        };

        let mut chars = fen.chars();
        let mut num_slashes = 0;
        let mut pos = 0;

        while num_slashes < 8 {
            let c = chars.next().unwrap();

            let mut is_piece = false;
            for i in 0..12 {
                if c == PIECES[i] {
                    is_piece = true;
                    p.piece_boards[i] |= 1 << ((63-pos)/8)*8 + (7-((63-pos)%8));
                    pos += 1;
                    break;
                }
            }
            if !is_piece { 
                if c != '/' && c != ' ' {
                    match c.to_digit(10) {
                        Some(t) => {
                            if pos + t < 64 {
                                pos += t;
                            }
                        }, 
                        _ => ()
                    }
                } else {
                    num_slashes += 1
                }
            }        
        }

        match chars.next() {
            Some(t) => if t == 'w' {p.side = true} else {p.side = false},
            _ => ()
        }

        if chars.next().unwrap() != ' ' {panic!()}
        let mut c = chars.next().unwrap(); 

        while c != ' ' {
            if c == 'k' {
                p.c_rights |= 0b0100;
            } else if c == 'q' {
                p.c_rights |= 0b1000;
            } else if c == 'K' {
                p.c_rights |= 0b0001;
            } else if c == 'Q' {
                p.c_rights |= 0b0010;   
            } 
            c = chars.next().unwrap(); 
        } 

        c = chars.next().unwrap();
        if c != '-' {
            for m in 0..8 {
                if FILES[m] == c {
                    p.ep += 1 + (m as i32);
                    break;
                }
            }  
            c = chars.next().unwrap();
            match c.to_digit(10) {
                Some(t) => {
                p.ep += 8*((t as i32)-1);
                }, 
                _ => panic!()
            }
        }

        if chars.next().unwrap() != ' ' {panic!()}

        match chars.next().unwrap().to_digit(10) {
            Some(t) => p.fifty = t as i32,
            _ => ()
        }

        if chars.next().unwrap() != ' ' {panic!()}

        let mut a = 0; 
        match chars.next().unwrap().to_digit(10) {
            Some(t) => {
                a = t;
            },
            _ => ()
        }
        
        let mut b = 0; 
        match chars.next() {
            Some(t) => {
                match t.to_digit(10) {
                    Some(t) => {
                        b = t;
                        match chars.next(){
                            Some(t) => {
                                match t.to_digit(10) {
                                    Some(t) => {
                                        p.ply = 2*(100*a + 10*b + t - 1) as i32;
                                        if !p.side { p.ply += 1}
                                    },
                                    _ => ()
                                }
                            },
                            _ => {
                                p.ply = 2*(10*a + b - 1) as i32;
                                if !p.side { p.ply += 1}
                            }
                        }
                    },
                    _ => println!("got here")
                }
            },
            _ => {
                p.ply = 2*(a - 1) as i32;
                if !p.side { p.ply += 1}
            }
        }
        
        let mut white: u64 = 0b0;
        let mut black: u64 = 0b0;
        let mut all: u64 = 0b0;

        for i in 0..6 {
            white |= p.piece_boards[i];
        }
        for i in 6..12 {
            black |= p.piece_boards[i];
        }
        
        all |= (white | black);

        p.side_boards[0] = white;
        p.side_boards[1] = black;
        p.side_boards[2] = all;

        p
    }

    pub fn print(&self){
        println!("\nPosition:\n");
        for i in (0..8).rev() {
            print!(" {} | ", i+1);
            for j in 0..8 {
                let mut p = true;
                for k in 0..12 {
                    if is_set(&self.piece_boards[k], 8*i + j){
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
        println!("______ep: {}\n", self.ep);
    }

}