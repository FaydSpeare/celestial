use crate::types::*;
use crate::bitboard::*;

pub struct Position {

    pub board: [i32; 64],

    // Bitboards
    pub piece_bb: [Bitboard; 12],
    pub colour_bb: [Bitboard; 3],

    pub side_to_move: bool,
    pub fifty: i32,
    pub ep: i32,

    pub castling_rights: u8,

    pub ply: i32,
    pub his_ply: i32,

    pub pos_key: Key,

    pub piece_num: [i32; 12],

    pub big_piece: [i32; 2],
    pub maj_piece: [i32; 2],
    pub min_piece: [i32; 2],

    pub king_sq: [i32; 2],

    pub material: [i32; 2],

    // Move History
    pub history: Vec<UndoEntry>,

    // Piece List
    pub piece_list: [[i32; 10]; 12]
}

pub struct UndoEntry {

    // Information to roll back
    pub motion: i32,
    pub castling_rights: u8,
    pub ep: i32,
    pub fifty: i32,
    pub pos_key: Key
}

impl Position {

    pub fn new() -> Position {
        Position {
            board: [Piece::EMPTY as i32; 64],

            piece_bb: [0u64; 12],
            colour_bb: [0u64; 3],

            side_to_move: true,
            fifty: 0,
            ep: Square::NO_SQ as i32,

            castling_rights: 0,

            ply: 0,
            his_ply: 0,

            pos_key: 0u64,

            piece_num: [0; 12],

            big_piece: [0; 2],
            maj_piece: [0; 2],
            min_piece: [0; 2],

            king_sq: [Square::NO_SQ as i32; 2],

            material: [0; 2],

            history: vec![],

            piece_list: [[0; 10]; 12]
        }
    }

    pub fn reset(&mut self){

        for i in 0..64 {
            self.board[i] = Piece::EMPTY as i32;
        }

        for i in 0..3 {
            self.big_piece[i] = 0;
            self.maj_piece[i] = 0;
            self.min_piece[i] = 0;

            self.colour_bb[i] = 0u64;
        }

        for i in 0..12 {
            self.piece_bb[i] = 0u64;
            self.piece_num[i] = 0;
        }

        self.king_sq[Colour::WHITE as usize] = Square::NO_SQ as i32;
        self.king_sq[Colour::BLACK as usize] = Square::NO_SQ as i32;

        self.side_to_move = true;
        self.fifty = 0;
        self.ep = Square::NO_SQ as i32;

        self.ply = 0;
        self.his_ply = 0;

        self.castling_rights = 0;
        self.pos_key = 0u64;
    }
}

impl UndoEntry {

}
