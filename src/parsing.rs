use crate::position::*;

pub fn parse_fen_string(fen_string: &str){

}

/* Constants for Parsing */
pub const PIECES: [char; 12] = ['P','N','B','R','Q','K','p','n','b','r','q','k'];
pub const CASTLING: [char; 4] = ['K','Q','k','q'];
pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

/*
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

*/
