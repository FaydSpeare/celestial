use crate::position::*;

pub fn parse_fen_string(fen_string: &str){

}

/* Constants for Parsing */
pub const PIECES: [char; 12] = ['P','N','B','R','Q','K','p','n','b','r','q','k'];
pub const CASTLING: [char; 4] = ['K','Q','k','q'];
pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];
