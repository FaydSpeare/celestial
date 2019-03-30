#![allow(dead_code)]
#![allow(unused_macros)]

use crate::types::*;

pub fn display(b: &Bitboard){
    println!();
    for i in (0..8).rev() {
        for j in 0..8 {
            if is_set(b, 8*i + j){print!("x");}
            else {print!("o");}
        }
        println!();
    }
}

pub fn is_set(b: &Bitboard, n: usize) -> bool {
    b & MASK[n] != 0
}

pub fn is_clear(b: &Bitboard, n: usize) -> bool {
    b & MASK[n] == 0
}
