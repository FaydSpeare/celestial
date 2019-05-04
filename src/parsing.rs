use crate::position::*;
use crate::types::*;
use crate::bitboard::*;
use crate::hashkey::*;

pub fn parse_fen_string(fen_string: &str) -> Position {

    let mut p = Position::new();

    let mut fen = fen_string.chars();

    let mut slashes = 0;
    let mut sq: i32 = 64;

    while slashes < 8 {
        let c = fen.next().unwrap();

        if c == '/' || c == ' ' {
            slashes += 1;
        } else {
            match c.to_digit(10) {
                Some(t) => {
                    sq -= t as i32;
                },
                _ => {
                    sq -= 1;
                    let pos = ((sq)/8)*8 + 7 - (sq % 8);

                    for i in 0..12 {
                        if PIECES[i] == c {
                            p.piece_bb[i] |= SET_MASK[pos as usize];
                            p.board[pos as usize] = i as i32;
                            break;
                        }
                    }
                }
            }
            
        }
    }

    let c = fen.next().unwrap();
    if c == 'b' {p.side_to_move = false}

    let c = fen.next().unwrap();
    let mut c = fen.next().unwrap();

    if c != '-' {
        while c != ' ' {
            match c {
                'K' => {
                    p.castling_rights ^= Castling::W_OO as u8;
                },
                'Q' => {
                    p.castling_rights ^= Castling::W_OOO as u8;
                }, 
                'k' => {
                    p.castling_rights ^= Castling::B_OO as u8;
                }, 
                'q' => {
                    p.castling_rights ^= Castling::B_OOO as u8;
                },
                _ => ()
            }
            c = fen.next().unwrap();
        }
    } else {
        c = fen.next().unwrap();
    }

    c = fen.next().unwrap();
    if c != '-' {
        let mut square = 0;
        for i in 0..8 {
            if FILES[i] == c {
                square += i as i32;
                break;
            }
        }
        c = fen.next().unwrap();
        match c.to_digit(10){
            Some(t) => square += 8*((t as i32)-1),
            _ => ()
        }
        p.ep = square;
    } 

    c = fen.next().unwrap();
    let mut a = 0;
    let mut b = 0;
    match fen.next().unwrap().to_digit(10) {
        Some(t) => {
            a = t as i32;
            match fen.next().unwrap().to_digit(10) {
                Some(t) => {
                    b = t as i32;
                    p.fifty = 10*a + b;
                },
                _ => {
                    p.fifty = a;
                }
            }
        },
        _ => {

        }
    }

    let mut a = 0;
    let mut b = 0;
    match fen.next().unwrap().to_digit(10) {
        Some(t) => {
            a = t as i32;
            match fen.next() {
                Some(k) => {
                    match k.to_digit(10) {
                        Some(t) => {
                            b = t as i32;
                            let k = 10*a + b;
                            p.ply = 2*(k-1);
                            if !p.side_to_move {
                                p.ply += 1;
                            }
                        },
                        _ => {
                            p.ply = 2*(a-1);
                    
                            if !p.side_to_move {
                                p.ply += 1;
                            }
                        }
                    }
                },
                _ => ()
            }
        },
        _ => ()
    }

    p.pos_key = generate_pos_key(&p);
    update_attributes(&mut p);

    p
  

}

pub fn update_attributes(p: &mut Position){

    for i in 0..64 {

        let piece = p.board[i];

        if piece != (Piece::EMPTY as i32) {
            let colour = PIECE_COLOUR[piece as usize];

            if PIECE_BIG[piece as usize] {
                p.big_piece[colour as usize] += 1;
            }
            if PIECE_MAJOR[piece as usize] {
                p.maj_piece[colour as usize] += 1;
            }
            if PIECE_MINOR[piece as usize] {
                p.min_piece[colour as usize] += 1;
            }

            p.material[colour as usize] += PIECE_VALUE[piece as usize];
            
            p.piece_list[piece as usize].push(i as i32);
            p.piece_num[piece as usize] += 1;

            if piece == Piece::W_KING as i32 {
                p.king_sq[Colour::WHITE as usize] = i as i32;
            } else if piece == Piece::B_KING as i32 {
                p.king_sq[Colour::BLACK as usize] = i as i32;
            }

        }
    }


    for i in 0..6 {
        p.colour_bb[Colour::WHITE as usize] |= p.piece_bb[i as usize];
    }
    for i in 6..12 {
        p.colour_bb[Colour::BLACK as usize] |= p.piece_bb[i as usize];
    }
    p.colour_bb[Colour::BOTH as usize] = p.colour_bb[Colour::BLACK as usize] | p.colour_bb[Colour::WHITE as usize];
}

/* Constants for Parsing */
pub const PIECES: [char; 12] = ['P','N','B','R','Q','K','p','n','b','r','q','k'];
pub const CASTLING: [char; 4] = ['K','Q','k','q'];
pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

pub fn print(p: &Position){
    println!("\nPosition:\n");
    for i in (0..8).rev() {
        print!(" {} | ", i+1);
        for j in 0..8 {
            let mut m = true;
            for k in 0..12 {
                if is_set(&p.piece_bb[k], 8*i + j){
                    print!("{} ", PIECES[k]);
                    m = false;
                    break;
                }
            }
            if m {print!("- ")}
        }
        println!();
    }
    println!("   |________________");
    println!("     a b c d e f g h");
    println!("\nto__play: {}", if p.side_to_move {"white"} else {"black"});
    print!("castling: ");
    for i in 0..4 {
        if (p.castling_rights & (1 << i)) != 0 {
            print!("{}",CASTLING[i]);
        }
    }
    println!();
    println!("_____ply: {}", p.ply);
    println!("___fifty: {}", p.fifty);
    println!("______ep: {}", p.ep);
    println!("_pos_key: {:x}\n", p.pos_key);
}

