use crate::types::*;

pub struct Position {
    pub bit: Bitboard
}

impl Position {

    pub fn print(&self){
        for i in (0..8).rev() {
            print!(" {} ", i+1);
            for _j in 0..8 {
                print!("- ");
            }
            println!();
        }
        println!("   a b c d e f g h");
    }

}