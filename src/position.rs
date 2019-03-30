use crate::types::*;
use crate::bitboard::*;

pub struct Position {

    // Bitboards
    pub piece_bb: [Bitboard; 12],
    pub color_bb: [Bitboard; 3],

    pub side_to_move: bool,
    pub fifty: i32,
    pub ep: i32,

    pub castling_rights: u16,

    pub ply: i32,

    pub pos_key: Key,

    pub piece_num: [i32; 12],

    pub big_piece: [i32; 3],
    pub maj_piece: [i32; 3],
    pub min_piece: [i32; 3]
}

impl Position {

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

}