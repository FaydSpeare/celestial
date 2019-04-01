use crate::position::*;
use crate::types::*;
use crate::motion::*;
use crate::attack::*;
use crate::parsing::*;

impl Position {

    pub fn add_piece(&mut self, piece: usize, square: usize){

        unsafe { self.pos_key ^= PIECE_KEYS[piece][square];}

        self.piece_list[piece].push(square as i32);

        self.piece_bb[piece] |= SET_MASK[square];
        self.colour_bb[PIECE_COLOUR[piece] as usize] |= SET_MASK[square];

        if PIECE_BIG[piece] {self.big_piece[PIECE_COLOUR[piece] as usize] += 1}
        if PIECE_MAJOR[piece] {self.maj_piece[PIECE_COLOUR[piece] as usize] += 1}
        if PIECE_MINOR[piece] {self.min_piece[PIECE_COLOUR[piece] as usize] += 1}

        self.piece_num[piece] += 1;

        self.board[square] = piece as i32;
    }

    pub fn move_piece(&mut self, from: usize, to: usize) {

        let piece = self.board[from] as usize;

        unsafe { 
            self.pos_key ^= PIECE_KEYS[piece][from];
            self.pos_key ^= PIECE_KEYS[piece][to];
        }

        //println!("from: {}, to: {}", from, to);
        //print(&self);
        for i in 0..(self.piece_list[piece].len()) {
            if self.piece_list[piece][i] == (from as i32) {
                self.piece_list[piece][i] = (to as i32);
                break;
            }
        }

        self.piece_bb[piece] ^= SET_MASK[from];
        self.colour_bb[PIECE_COLOUR[piece] as usize] ^= SET_MASK[from];

        self.piece_bb[piece] |= SET_MASK[to];
        self.colour_bb[PIECE_COLOUR[piece] as usize] |= SET_MASK[to];

        self.board[from] = Piece::EMPTY as i32;
        self.board[to] = piece as i32;

    }

    pub fn clear_piece(&mut self, square: usize) {

        let piece = self.board[square] as usize;

        unsafe { self.pos_key ^= PIECE_KEYS[piece][square];}

        for i in 0..(self.piece_list[piece].len()) {
            if self.piece_list[piece][i] == square as i32 {
                self.piece_list[piece].swap_remove(i);
                break;
            }
        }

        

        self.piece_bb[piece] ^= SET_MASK[square];
        self.colour_bb[PIECE_COLOUR[piece] as usize] ^= SET_MASK[square];

        if PIECE_BIG[piece] {self.big_piece[PIECE_COLOUR[piece] as usize] -= 1}
        if PIECE_MAJOR[piece] {self.maj_piece[PIECE_COLOUR[piece] as usize] -= 1}
        if PIECE_MINOR[piece] {self.min_piece[PIECE_COLOUR[piece] as usize] -= 1}

        self.piece_num[piece] -= 1;

        self.board[square] = Piece::EMPTY as i32;

    }

    pub fn do_motion(&mut self, motion: &Motion) -> bool {

        //print(&self);

        let from = motion.from() as usize;
        let to = motion.to() as usize;
        let piece = self.board[from] as usize;

        //println!("1");

        let mut u = UndoEntry {
            motion: motion.motion,
            castling_rights: self.castling_rights,
            ep: self.ep,
            fifty: self.fifty,
            pos_key: self.pos_key,
            capture: self.board[to] as usize
        };

        //println!("2");

        if motion.is_enpassant() {

            if self.side_to_move {
                self.clear_piece((to-8) as usize);
            } else {
                self.clear_piece((to+8) as usize);
            }

        } else if motion.is_castling() {

            if to == Square::C1 as usize {
                self.move_piece(Square::A1 as usize, Square::D1 as usize);
            } else if to == Square::C8 as usize {
                self.move_piece(Square::A8 as usize, Square::D8 as usize);
            } else if to == Square::G1 as usize {
                self.move_piece(Square::H1 as usize, Square::F1 as usize);
            } else if to == Square::G8 as usize { 
                self.move_piece(Square::H8 as usize, Square::F8 as usize);
            } else {
                panic!();
            }

        }

        //println!("3");

        if self.ep != Square::NO_SQ as i32 {
            unsafe {
                self.pos_key ^= PIECE_KEYS[Piece::EMPTY as usize][self.ep as usize];
            }
        }

        unsafe {
            self.pos_key ^= CASTLE_KEYS[self.castling_rights as usize];
        }

        //println!("4");

        u.fifty = self.fifty;
        u.ep = self.ep;
        u.castling_rights = self.castling_rights;

        self.history.push(u);

        self.castling_rights &= CASTLE_PERM[from];
        self.castling_rights &= CASTLE_PERM[to];
        self.ep = Square::NO_SQ as i32;

        unsafe {
            self.pos_key ^= CASTLE_KEYS[self.castling_rights as usize];
        }

        //println!("5");

        self.fifty += 1;

        let captured = self.board[to] as usize;
        if captured != Piece::EMPTY as usize {
            //println!("CAPTURE");
            self.clear_piece(to);
            self.fifty = 0;
        }

        self.ply += 1;

        

        if PIECE_PAWN[piece] {
            
            self.fifty = 0;

            if self.side_to_move {
                if from == to - 16 {
                    self.ep = (to - 8) as i32;
                    unsafe { self.pos_key ^= PIECE_KEYS[Piece::EMPTY as usize][self.ep as usize];}
                }

            } else {
                if from == to + 16 {
                    self.ep = (to + 8) as i32;
                    unsafe { self.pos_key ^= PIECE_KEYS[Piece::EMPTY as usize][self.ep as usize];}
                }
            }
        }

        

        self.move_piece(from, to);

        if motion.is_promotion(){
            self.clear_piece(to);
            self.add_piece(motion.promotee() as usize, to);
        }

        if PIECE_KING[piece] {
            self.king_sq[PIECE_COLOUR[piece] as usize] = to as i32;
        }
        
        self.side_to_move = !self.side_to_move;

        unsafe { self.pos_key ^= COLOUR_KEY; }

        self.colour_bb[2] = self.colour_bb[0] | self.colour_bb[1];

        if self.side_to_move {
            if is_attacked_by(self, self.king_sq[Colour::BLACK as usize] as usize, true) {
                self.undo_motion();
                //println!("illegal");
                //println!("{}", self.king_sq[Colour::BLACK as usize]);
                return false;
            }
        } else {
            if is_attacked_by(self, self.king_sq[Colour::WHITE as usize] as usize, false) {
                self.undo_motion();
                //println!("illegal");
                //println!("{}", self.king_sq[Colour::WHITE as usize]);
                return false;
            }
        }
        
        return true;
    }

    pub fn undo_motion(&mut self){

        self.ply -= 1;

        let undo_motion = self.history.pop().unwrap();

        let from = undo_motion.from() as usize;
        let to = undo_motion.to() as usize;
        let piece = self.board[to] as usize;

        if self.ep != Square::NO_SQ as i32 {
            unsafe {
                self.pos_key ^= PIECE_KEYS[Piece::EMPTY as usize][self.ep as usize];
            }
        }

        unsafe {
            self.pos_key ^= CASTLE_KEYS[self.castling_rights as usize];
        }

        self.castling_rights = undo_motion.castling_rights;
        self.ep = undo_motion.ep;
        self.fifty = undo_motion.fifty;

        if self.ep != Square::NO_SQ as i32 {
            unsafe {
                self.pos_key ^= PIECE_KEYS[Piece::EMPTY as usize][self.ep as usize];
            }
        }

        unsafe {
            self.pos_key ^= CASTLE_KEYS[self.castling_rights as usize];
        }

        self.side_to_move = !self.side_to_move;

        unsafe { self.pos_key ^= COLOUR_KEY; }

        if undo_motion.is_enpassant() {

            if self.side_to_move {
                self.add_piece(Piece::B_PAWN as usize, (to-8) as usize);
            } else {
                self.add_piece(Piece::W_PAWN as usize, (to+8) as usize);
            }

        } else if undo_motion.is_castling() {
            
            if to == Square::C1 as usize {
                self.move_piece(Square::D1 as usize, Square::A1 as usize);
            } else if to == Square::C8 as usize {
                self.move_piece(Square::D8 as usize, Square::A8 as usize);
            } else if to == Square::G1 as usize {
                self.move_piece(Square::F1 as usize, Square::H1 as usize);
            } else if to == Square::G8 as usize { 
                self.move_piece(Square::F8 as usize, Square::H8 as usize);
            } else {
                panic!();
            }

        }

        self.move_piece(to, from);

        if PIECE_KING[piece] {
            self.king_sq[PIECE_COLOUR[piece] as usize] = from as i32;
        }

        let captured = undo_motion.capture;
        if captured != Piece::EMPTY as usize {
            self.add_piece(captured, to);
        }

        if undo_motion.is_promotion(){
            self.clear_piece(from);
            if PIECE_COLOUR[piece] == Colour::WHITE as i32 {
                self.add_piece(Piece::W_PAWN as usize, to);
            } else {
                self.add_piece(Piece::B_PAWN as usize, to);
            }
            
        }

        self.colour_bb[2] = self.colour_bb[0] | self.colour_bb[1];

    }
    
}