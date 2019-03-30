#![allow(dead_code)]
#![allow(non_camel_case_types)]

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
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8, 
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8, NO_SQ = -1
}

pub enum Castling {
    W_OO = 1, W_OOO = 2, B_OO = 4, B_OOO = 8
}

/* Useful Arrays */

pub const MASK: [u64; 64] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000,
0x10000, 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000, 0x800000,
0x1000000, 0x2000000, 0x4000000, 0x8000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000,
0x100000000, 0x200000000, 0x400000000, 0x800000000, 0x1000000000, 0x2000000000, 0x4000000000, 0x8000000000,
0x10000000000, 0x20000000000, 0x40000000000, 0x80000000000, 0x100000000000, 0x200000000000, 0x400000000000, 0x800000000000,
0x1000000000000, 0x2000000000000, 0x4000000000000, 0x8000000000000, 0x10000000000000, 0x20000000000000, 0x40000000000000, 0x80000000000000,
0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000];

pub const ANTIMASK: [u64; 64] = [0xfffffffffffffffe, 0xfffffffffffffffd, 0xfffffffffffffffb, 0xfffffffffffffff7, 0xffffffffffffffef, 0xffffffffffffffdf, 0xffffffffffffffbf, 0xffffffffffffff7f,
0xfffffffffffffeff, 0xfffffffffffffdff, 0xfffffffffffffbff, 0xfffffffffffff7ff, 0xffffffffffffefff, 0xffffffffffffdfff, 0xffffffffffffbfff, 0xffffffffffff7fff,
0xfffffffffffeffff, 0xfffffffffffdffff, 0xfffffffffffbffff, 0xfffffffffff7ffff, 0xffffffffffefffff, 0xffffffffffdfffff, 0xffffffffffbfffff, 0xffffffffff7fffff,
0xfffffffffeffffff, 0xfffffffffdffffff, 0xfffffffffbffffff, 0xfffffffff7ffffff, 0xffffffffefffffff, 0xffffffffdfffffff, 0xffffffffbfffffff, 0xffffffff7fffffff,
0xfffffffeffffffff, 0xfffffffdffffffff, 0xfffffffbffffffff, 0xfffffff7ffffffff, 0xffffffefffffffff, 0xffffffdfffffffff, 0xffffffbfffffffff, 0xffffff7fffffffff,
0xfffffeffffffffff, 0xfffffdffffffffff, 0xfffffbffffffffff, 0xfffff7ffffffffff, 0xffffefffffffffff, 0xffffdfffffffffff, 0xffffbfffffffffff, 0xffff7fffffffffff,
0xfffeffffffffffff, 0xfffdffffffffffff, 0xfffbffffffffffff, 0xfff7ffffffffffff, 0xffefffffffffffff, 0xffdfffffffffffff, 0xffbfffffffffffff, 0xff7fffffffffffff,
0xfeffffffffffffff, 0xfdffffffffffffff, 0xfbffffffffffffff, 0xf7ffffffffffffff, 0xefffffffffffffff, 0xdfffffffffffffff, 0xbfffffffffffffff, 0x7fffffffffffffff];
