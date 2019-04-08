#![allow(dead_code)]
#![allow(non_camel_case_types)]

use rand::Rng;

/* Useful Type Definitions */

pub type Bitboard = u64;
pub type Key = u64;

/* Useful Constants */

pub const MAX_GAME_MOVES: usize = 2048;
pub const BOARD_NUM: usize = 64;
pub const PIECE_NUM: usize = 6;
pub const COLOUR_NUM: usize = 2;

/* Useful Enums */

pub enum Colour {
    WHITE, BLACK, BOTH
}

#[derive(Debug)]
pub enum Piece {
    W_PAWN, W_KNIGHT, W_BISHOP, W_ROOK, W_QUEEN, W_KING, 
    B_PAWN, B_KNIGHT, B_BISHOP, B_ROOK, B_QUEEN, B_KING, EMPTY
}

pub enum Rank {
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8
}

pub enum File {
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H
}

pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    NO_SQ = -1
}

pub enum Castling {
    W_OO = 1, W_OOO = 2, B_OO = 4, B_OOO = 8
}

/* Useful Arrays */

pub const CASTLE_PERM: [u8; 64] = [13, 15, 15, 15, 12, 15, 15, 14, 
                                    15, 15, 15, 15, 15, 15, 15, 15,
                                    15, 15, 15, 15, 15, 15, 15, 15,
                                    15, 15, 15, 15, 15, 15, 15, 15,
                                    15, 15, 15, 15, 15, 15, 15, 15,
                                    15, 15, 15, 15, 15, 15, 15, 15,
                                    15, 15, 15, 15, 15, 15, 15, 15,
                                     7, 15, 15, 15,  3, 15, 15, 11];

pub const SET_MASK: [u64; 64] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000,
0x10000, 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000, 0x800000,
0x1000000, 0x2000000, 0x4000000, 0x8000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000,
0x100000000, 0x200000000, 0x400000000, 0x800000000, 0x1000000000, 0x2000000000, 0x4000000000, 0x8000000000,
0x10000000000, 0x20000000000, 0x40000000000, 0x80000000000, 0x100000000000, 0x200000000000, 0x400000000000, 0x800000000000,
0x1000000000000, 0x2000000000000, 0x4000000000000, 0x8000000000000, 0x10000000000000, 0x20000000000000, 0x40000000000000, 0x80000000000000,
0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000];

pub const CLR_MASK: [u64; 64] = [0xfffffffffffffffe, 0xfffffffffffffffd, 0xfffffffffffffffb, 0xfffffffffffffff7, 0xffffffffffffffef, 0xffffffffffffffdf, 0xffffffffffffffbf, 0xffffffffffffff7f,
0xfffffffffffffeff, 0xfffffffffffffdff, 0xfffffffffffffbff, 0xfffffffffffff7ff, 0xffffffffffffefff, 0xffffffffffffdfff, 0xffffffffffffbfff, 0xffffffffffff7fff,
0xfffffffffffeffff, 0xfffffffffffdffff, 0xfffffffffffbffff, 0xfffffffffff7ffff, 0xffffffffffefffff, 0xffffffffffdfffff, 0xffffffffffbfffff, 0xffffffffff7fffff,
0xfffffffffeffffff, 0xfffffffffdffffff, 0xfffffffffbffffff, 0xfffffffff7ffffff, 0xffffffffefffffff, 0xffffffffdfffffff, 0xffffffffbfffffff, 0xffffffff7fffffff,
0xfffffffeffffffff, 0xfffffffdffffffff, 0xfffffffbffffffff, 0xfffffff7ffffffff, 0xffffffefffffffff, 0xffffffdfffffffff, 0xffffffbfffffffff, 0xffffff7fffffffff,
0xfffffeffffffffff, 0xfffffdffffffffff, 0xfffffbffffffffff, 0xfffff7ffffffffff, 0xffffefffffffffff, 0xffffdfffffffffff, 0xffffbfffffffffff, 0xffff7fffffffffff,
0xfffeffffffffffff, 0xfffdffffffffffff, 0xfffbffffffffffff, 0xfff7ffffffffffff, 0xffefffffffffffff, 0xffdfffffffffffff, 0xffbfffffffffffff, 0xff7fffffffffffff,
0xfeffffffffffffff, 0xfdffffffffffffff, 0xfbffffffffffffff, 0xf7ffffffffffffff, 0xefffffffffffffff, 0xdfffffffffffffff, 0xbfffffffffffffff, 0x7fffffffffffffff];

pub const PIECE_BIG: [bool; 13] = [false, true, true, true, true, true, false, true, true, true, true, true, false];
pub const PIECE_MAJOR: [bool; 13] = [false, false, false, true, true, true, false, false, false, true, true, true, false];
pub const PIECE_MINOR: [bool; 13] = [false, true, true, false, false, false, false, true, true, false, false, false, false];
pub const PIECE_COLOUR: [i32; 13] = [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2];
pub const PIECE_VALUE: [i32; 13] = [100, 320, 325, 550, 1000, 50000, 100, 320, 325, 550, 1000, 50000, 0];

pub const PIECE_PAWN: [bool; 13] = [true, false, false, false, false, false, true, false, false, false, false, false, false];
pub const PIECE_KING: [bool; 13] = [false, false, false, false, false, true, false, false, false, false, false, true, false];

pub static mut PIECE_KEYS: [[u64; 64]; 13] = [[0u64; 64]; 13];
pub static mut COLOUR_KEY: u64 = 0u64;
pub static mut CASTLE_KEYS: [u64; 16] = [0u64; 16];


/* Init Function */
pub fn init(){
    let mut rng = rand::thread_rng();
    
    // init u64 keys
    unsafe {
        for i in 0..13 {
            for j in 0..64 {
                PIECE_KEYS[i][j] = rng.gen::<u64>();
            }
        }
        COLOUR_KEY = rng.gen::<u64>();
        for i in 0..16 {
            CASTLE_KEYS[i] = rng.gen::<u64>();
        }
    }

    
    
}