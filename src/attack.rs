use crate::movegen::*;
use crate::types::*;
use crate::position::*;

pub fn is_attacked_by(p: &Position, sq: usize, attacker: bool) -> bool {

    // PAWNS //
    if is_attacked_by_pawn(p, sq, attacker) {
        return true;
    }

    if is_attacked_by_knight(p, sq, attacker) {
        return true;
    }

    return false;

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