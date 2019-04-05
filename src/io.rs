use crate::motion::*;

pub const FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

pub fn print_move(motion: &Motion){

    let from = motion.from() as usize;
    let to = motion.to() as usize;

    print!("{}{}{}{}", FILES[from%8 as usize], 1 + from/8, FILES[to%8 as usize], 1 + to/8);
    if motion.is_promotion(){
        if motion.is_prom_queen(){
            print!("{}", 'q');
        } else if motion.is_prom_rook(){
            print!("{}", 'r');
        } else if motion.is_prom_bishop(){
            print!("{}", 'b');
        } else if motion.is_prom_knight(){
            print!("{}", 'n');
        }
    }
    //println!();
}

pub fn print_moves(list: &Vec<Motion>){
    for m in list.iter() {
        print_move(m);
    }
}