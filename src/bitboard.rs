#![allow(dead_code)]
#![allow(unused_macros)]

use crate::types::*;

macro_rules! SET {($b: expr, $n: expr) => { is_set($b, $n)};}
macro_rules! CLEAR {($b: expr, $n: expr) => { is_clear($b, $n)};}

pub fn display(b: Bitboard){
    for i in (0..8).rev() {
        for j in 0..8 {
            if SET!(b, 8*i + j){print!("x");}
            else {print!("o");}
        }
        println!();
    }
}

pub fn is_set(b: Bitboard, n: usize) -> bool {
    b & MASK[n] != 0
}

pub fn is_clear(b: Bitboard, n: usize) -> bool {
    b & MASK[n] == 0
}
