use crate::types::*;
use crate::bitboard::*;

pub struct Position {

    pub board: [i32; 64],

    // Bitboards
    pub piece_bb: [Bitboard; 12],
    pub color_bb: [Bitboard; 3],

    pub side_to_move: bool,
    pub fifty: i32,
    pub ep: i32,

    pub castling_rights: u8,

    pub ply: i32,
    pub his_ply: i32,

    pub pos_key: Key,

    pub piece_num: [i32; 12],

    pub big_piece: [i32; 3],
    pub maj_piece: [i32; 3],
    pub min_piece: [i32; 3],

    pub king_sq: [i32; 2],

    // Move History
    pub history: [UndoEntry; MAX_GAME_MOVES],

    // Piece List
    pub piece_list: [[i32; 12]; 10]
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

    pub fn reset(&mut self){

        for i in 0..64 {
            self.board[i] = Piece::EMPTY as i32;
        }

        for i in 0..3 {
            self.big_piece[i] = 0;
            self.maj_piece[i] = 0;
            self.min_piece[i] = 0;

            self.color_bb[i] = 0u64;
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
