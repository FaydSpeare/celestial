#![allow(dead_code)]
#![allow(unused_macros)]

use crate::types::*;

pub fn print_bb(b: &Bitboard){
    println!();
    for i in (0..8).rev() {
        for j in 0..8 {
            if is_set(b, 8*i + j){print!("X ");}
            else {print!("- ");}
        }
        println!();
    }
}

pub fn is_set(b: &Bitboard, n: usize) -> bool {
    b & SET_MASK[n] != 0
}

pub fn is_clear(b: &Bitboard, n: usize) -> bool {
    b & SET_MASK[n] == 0
}
