use crate::position::*;
use crate::types::*;
use crate::types::Piece::*;

pub fn generate_pos_key(p: &Position) -> u64 {

    let mut key: u64 = 0u64;

    unsafe {
        for i in 0..64 {
            let piece = p.board[i].clone();
            if piece != (Piece::EMPTY as i32) {

                assert!(piece >= (W_PAWN as i32) && piece <= (B_KING as i32));
                key ^= PIECE_KEYS[piece as usize][i as usize];
            }
        }

        if p.side_to_move {
            key ^= COLOUR_KEY;
        }

        if p.ep != (Square::NO_SQ as i32) {

            assert!(p.ep >= 0 && p.ep < 64);
            key ^= PIECE_KEYS[EMPTY as usize][p.ep as usize];
        }

        assert!(p.castling_rights < 16);
        key ^= CASTLE_KEYS[p.castling_rights as usize];
        }

    

    key
}