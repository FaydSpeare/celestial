use crate::bitboard::*;

#[macro_export]
macro_rules! LSB {($b: expr) => {$b.trailing_zeros()};}

#[macro_export]
macro_rules! CNT {($b: expr) => {$b.count_ones()};}

#[macro_export]
macro_rules! SET {($b: expr, $n: expr) => { is_set($b, $n)};}

#[macro_export]
macro_rules! CLR {($b: expr, $n: expr) => { is_clear($b, $n)};}

#[macro_export]
macro_rules! MOVE_INT {($from: expr, $to: expr, $prom: expr, $flag: expr) => { $flag << 14 | $prom << 12 | $to << 6 | $from }}
