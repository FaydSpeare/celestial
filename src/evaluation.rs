use crate::position::*;
use crate::attack::*;
use crate::types::*;
use crate::movegen::*;
use crate::search::*;
use std::collections::HashSet;

pub const PAWN_TABLE: [i32; 64] = [   0,   0,   0,   0,   0,   0,   0,   0, 
                                     40,  40,  40,  40,  40,  40,  40,  40, 
                                     24,  24,  24,  24,  24,  24,  24,  24, 
                                     10,  10,  10,  15,  15,  10,  10,  10, 
                                      5,   5,  15,  20,  20,   5,   5,   5, 
                                      5,   5, -10,  10,  10, -10,   5,   5,  
                                     10,  15,  15, -20,  -20,  15,  15,  10, 
                                      0,   0,   0,   0,   0,   0,   0,   0, 
                                    ];

pub const KNIGHT_TABLE: [i32; 64] = [ -10,   0,   0,   0,   0,   0,   0, -10, 
                                        0,   0,   5,   5,   5,   5,   0,   0, 
                                        0,   5,  10,  10,  10,  10,   5,   0, 
                                        0,   5,  10,  15,  15,  10,   5,   0, 
                                        0,   5,  10,  15,  15,  10,   5,   0, 
                                        0,   5,  10,  10,  10,  10,   5,   0,  
                                        0,   0,   5,   5,   5,   5,   0,   0, 
                                      -10,   0,   0,   0,   0,   0,   0, -10, 
                                    ];

pub const BISHOP_TABLE: [i32; 64] = [   0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,  10,  10,   0,   0,   0, 
                                        0,   5,  10,  10,  10,  10,   5,   0, 
                                        0,   5,   5,   5,   5,   5,   5,   0,  
                                        0,  10,   5,   5,   5,   5,  10,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                    ];

pub const ROOK_TABLE: [i32; 64] = [     0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   5,   5,   5,   5,   5,   5,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                       -5,   0,   0,   0,   0,   0,   0,  -5,  
                                       -5,   0,   0,   5,   5,   0,   0,  -5, 
                                        0,   2,   3,   8,   8,   3,   2,   0, 
                                    ];

pub const KING_TABLE: [i32; 64] = [     0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                        0,   0,   0,   0,   0,   0,   0,   0,  
                                        3,   3,   5,   0,   0,   0,   5,   3, 
                                       10,  20,  20,   5,   0,  10,  30,  10, 
                                    ];

pub const PAWN_END_TABLE: [i32; 64] = [ 0,   0,   0,   0,   0,   0,   0,   0, 
                                       60,  60,  60,  60,  60,  60,  60,  60, 
                                       38,  38,  38,  38,  38,  38,  38,  38, 
                                       25,  25,  25,  25,  25,  25,  25,  25, 
                                       10,  10,  10,  10,  10,  10,  10,  10, 
                                       -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,  
                                      -25, -25, -25, -25, -25, -25, -25, -25, 
                                        0,   0,   0,   0,   0,   0,   0,   0, 
                                    ];
        

pub fn transform_white(i: i32) -> usize {
    let rank = (i / 8);
    let file = i % 8;
    let ret = (file + 8 * (7 - rank)) as usize;
    return ret;
}

pub fn eval(pos: &Position) -> i32 {

    let mut eval = 0;

    eval += pos.material[0];
    eval -= pos.material[1];

    if pos.big_piece[0] + pos.big_piece[1] < 7  {
        for i in 0..pos.piece_list[Piece::W_PAWN as usize].len() {
            eval += PAWN_END_TABLE[transform_white(pos.piece_list[Piece::W_PAWN as usize][i]) as usize];
        }

        for i in 0..pos.piece_list[Piece::B_PAWN as usize].len() {
            eval -= PAWN_END_TABLE[pos.piece_list[Piece::B_PAWN as usize][i] as usize];
        }

        for i in 0..pos.piece_list[Piece::W_KING as usize].len() {
            eval += KNIGHT_TABLE[transform_white(pos.piece_list[Piece::W_KING as usize][i]) as usize];
        }

        for i in 0..pos.piece_list[Piece::B_KING as usize].len() {
            eval -= KNIGHT_TABLE[pos.piece_list[Piece::B_KING as usize][i] as usize];
        }

    } 
    else {
        for i in 0..pos.piece_list[Piece::W_PAWN as usize].len() {
            eval += PAWN_TABLE[transform_white(pos.piece_list[Piece::W_PAWN as usize][i]) as usize];
        }

        for i in 0..pos.piece_list[Piece::B_PAWN as usize].len() {
            eval -= PAWN_TABLE[(pos.piece_list[Piece::B_PAWN as usize][i]) as usize];
        }
    }



    for i in 0..pos.piece_list[Piece::W_KNIGHT as usize].len() {
        eval += KNIGHT_TABLE[transform_white(pos.piece_list[Piece::W_KNIGHT as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::B_KNIGHT as usize].len() {
        eval -= KNIGHT_TABLE[(pos.piece_list[Piece::B_KNIGHT as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::W_BISHOP as usize].len() {
        eval += BISHOP_TABLE[transform_white(pos.piece_list[Piece::W_BISHOP as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::B_BISHOP as usize].len() {
        eval -= BISHOP_TABLE[(pos.piece_list[Piece::B_BISHOP as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::W_ROOK as usize].len() {
        eval += ROOK_TABLE[transform_white(pos.piece_list[Piece::W_ROOK as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::B_ROOK as usize].len() {
        eval -= ROOK_TABLE[(pos.piece_list[Piece::B_ROOK as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::W_QUEEN as usize].len() {
        eval += KNIGHT_TABLE[transform_white(pos.piece_list[Piece::W_QUEEN as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::B_QUEEN as usize].len() {
        eval -= KNIGHT_TABLE[(pos.piece_list[Piece::B_QUEEN as usize][i]) as usize];
    }

    for i in 0..pos.piece_list[Piece::W_ROOK as usize].len() {
        let file = pos.piece_list[Piece::W_ROOK as usize][i] % 8;
        if FILE_MASK[file as usize] & pos.piece_bb[Piece::W_PAWN as usize] == 0 {
            eval += 25;
            if FILE_MASK[file as usize] & pos.piece_bb[Piece::B_PAWN as usize] == 0 {
                eval += 12;
            }
        }
    } 

    for i in 0..pos.piece_list[Piece::B_ROOK as usize].len() {
        let file = pos.piece_list[Piece::B_ROOK as usize][i] % 8;
        if FILE_MASK[file as usize] & pos.piece_bb[Piece::B_PAWN as usize] == 0 {
            eval -= 25;
            if FILE_MASK[file as usize] & pos.piece_bb[Piece::W_PAWN as usize] == 0 {
                eval -= 12;
            }
        }
    } 

    for i in 0..pos.piece_list[Piece::W_QUEEN as usize].len() {
        let file = pos.piece_list[Piece::W_QUEEN as usize][i] % 8;
        if FILE_MASK[file as usize] & pos.piece_bb[Piece::W_PAWN as usize] == 0 {
            eval += 15;
            if FILE_MASK[file as usize] & pos.piece_bb[Piece::B_PAWN as usize] == 0 {
                eval += 8;
            }
        }
    } 

    for i in 0..pos.piece_list[Piece::B_QUEEN as usize].len() {
        let file = pos.piece_list[Piece::B_QUEEN as usize][i] % 8;
        if FILE_MASK[file as usize] & pos.piece_bb[Piece::B_PAWN as usize] == 0 {
            eval -= 15;
            if FILE_MASK[file as usize] & pos.piece_bb[Piece::W_PAWN as usize] == 0 {
                eval -= 8;
            }
        }
    } 

    for i in 0..8 {
        if (pos.piece_bb[Piece::W_PAWN as usize] & FILE_MASK[i]).count_ones() > 1 {
            eval -= 20;
            if i == 0 || i == 7 {
                eval -= 10;
            }
        }
        if (pos.piece_bb[Piece::B_PAWN as usize] & FILE_MASK[i]).count_ones() > 1 {
            eval += 20;
            if i == 0 || i == 7 {
                eval += 10;
            }
        }
    }

    if pos.piece_num[Piece::W_BISHOP as usize] > 1 {
        eval += 25;
    }
    if pos.piece_num[Piece::B_BISHOP as usize] > 1 {
        eval -= 25;
    }
    
    if !pos.side_to_move {
        return -eval;
    }

    eval
}