use crate::movegen::*;
use crate::types::*;
use crate::position::*;

pub fn is_attacked_by(p: &Position, sq: usize, attacker: bool) -> bool {

    if is_attacked_by_pawn(p, sq, attacker) {
        return true;
    }

    if is_attacked_by_knight(p, sq, attacker) {
        return true;
    }

    if is_attacked_by_bishop(p, sq, attacker) {
        return true;
    }

    if is_attacked_by_rook(p, sq, attacker) {
        return true;
    }

    if is_attacked_by_queen(p, sq, attacker) {
        return true;
    }

    if is_attacked_by_king(p, sq, attacker) {
        return true;
    }

    false

}

pub fn is_attacked_by_pawn(p: &Position, sq: usize, attacker: bool) -> bool {
   
    if attacker { // WHITE ATTACKER
        let pawn_attacks = B_PAWN_CAP[sq];

        if p.piece_bb[Piece::W_PAWN as usize] & pawn_attacks != 0 {
            return true;
        }
    } else { // BLACK ATTACKER
        let pawn_attacks = W_PAWN_CAP[sq];

        if p.piece_bb[Piece::B_PAWN as usize] & pawn_attacks != 0 {
            return true;
        }
    }
    return false;
}

pub fn is_attacked_by_knight(p: &Position, sq: usize, attacker: bool) -> bool {
   
    let knight_attacks = KN_MOVES[sq];

    if attacker {
        if knight_attacks & p.piece_bb[Piece::W_KNIGHT as usize] != 0 {
            return true;
        }
    } else {
        if knight_attacks & p.piece_bb[Piece::B_KNIGHT as usize] != 0 {
            return true;
        }
    }
    return false;
}

pub fn is_attacked_by_queen(p: &Position, sq: usize, attacker: bool) -> bool {

    let queen_attacks = sliding_attacks(sq, p.colour_bb[Colour::BOTH as usize]);

    if attacker {
        if queen_attacks & p.piece_bb[Piece::W_QUEEN as usize] != 0 {
            return true;
        }
    } else {
        if queen_attacks & p.piece_bb[Piece::B_QUEEN as usize] != 0 {
            return true;
        }
    }
    return false;
}

pub fn is_attacked_by_bishop(p: &Position, sq: usize, attacker: bool) -> bool {

    let bishop_attacks = diag_sliding_attacks(sq, p.colour_bb[Colour::BOTH as usize]);

    if attacker {
        if bishop_attacks & p.piece_bb[Piece::W_BISHOP as usize] != 0 {
            return true;
        }
    } else {
        if bishop_attacks & p.piece_bb[Piece::B_BISHOP as usize] != 0 {
            return true;
        }
    }
    return false;
}

pub fn is_attacked_by_rook(p: &Position, sq: usize, attacker: bool) -> bool {

    let rook_attacks = flat_sliding_attacks(sq, p.colour_bb[Colour::BOTH as usize]);

    if attacker {
        if rook_attacks & p.piece_bb[Piece::W_ROOK as usize] != 0 {
            return true;
        }
    } else {
        if rook_attacks & p.piece_bb[Piece::B_ROOK as usize] != 0 {
            return true;
        }
    }
    return false;
}

pub fn is_attacked_by_king(p: &Position, sq: usize, attacker: bool) -> bool {
   
    let king_attacks = KING_MOVES[sq];

    if attacker {
        if king_attacks & p.piece_bb[Piece::W_KNIGHT as usize] != 0 {
            return true;
        }
    } else {
        if king_attacks & p.piece_bb[Piece::B_KNIGHT as usize] != 0 {
            return true;
        }
    }
    return false;
}
