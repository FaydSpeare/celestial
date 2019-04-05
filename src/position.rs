use crate::types::*;
use crate::bitboard::*;
use crate::motion::*;
use std::collections::HashMap;

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
    pub piece_list: [Vec<i32>; 12],

    // PV Table
    pub pv_table: HashMap<Key, Motion>,
    pub pv_array: Vec<i32>,

    // Search Arrays
    pub search_history: [[i32; 64]; 13],
    pub search_killers: [[i32; 100]; 2]


}

pub struct UndoEntry {

    // Information to roll back
    pub motion: u16,
    pub castling_rights: u8,
    pub ep: i32,
    pub fifty: i32,
    pub pos_key: Key,
    pub capture: usize

}

impl UndoEntry {

    pub fn from(&self) -> u16 {
        self.motion & 0x3F
    }

    pub fn to(&self) -> u16 {
        (self.motion >> 6) & 0x3F
    }

    pub fn flag(&self) -> u16 {
        (self.motion >> 14) & 0x3
    }

    pub fn promotee(&self) -> u16 {
        (self.motion >> 12) & 0x3
    }

    pub fn is_promotion(&self) -> bool {
        self.flag() == Flag::PROMOTION as u16
    }

    pub fn is_enpassant(&self) -> bool {
        self.flag() == Flag::ENPASSANT as u16
    }

    pub fn is_castling(&self) -> bool {
        self.flag() == Flag::CASTLING as u16
    }

    pub fn is_none(&self) -> bool {
        self.flag() == Flag::NONE as u16
    }

    pub fn is_prom_queen(&self) -> bool {
        self.promotee() == Promotee::QUEEN as u16
    }

    pub fn is_prom_rook(&self) -> bool {
        self.promotee() == Promotee::ROOK as u16
    }

    pub fn is_prom_bishop(&self) -> bool {
        self.promotee() == Promotee::BISHOP as u16
    }

    pub fn is_prom_knight(&self) -> bool {
        self.promotee() == Promotee::KNIGHT as u16
    }

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

            piece_list: [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]],

            pv_table: HashMap::new(),
            pv_array:vec![],

            // Search Arrays
            search_history: [[0; 64]; 13],
            search_killers: [[0; 100]; 2]
        }
    }

    pub fn reset(&mut self){

        for i in 0..64 {
            self.board[i] = Piece::EMPTY as i32;
        }

        for i in 0..2 {
            self.big_piece[i] = 0;
            self.maj_piece[i] = 0;
            self.min_piece[i] = 0;
        }

        for i in 0..3 {
            self.colour_bb[i] = 0u64;
        }

        for i in 0..12 {
            self.piece_bb[i] = 0u64;
            self.piece_num[i] = 0;

            self.piece_list[i] = vec![];
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
