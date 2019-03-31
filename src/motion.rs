pub struct Motion {
    pub m: u16,
    pub score: i32
}

impl Motion {

    pub fn from(&self) -> u16 {
        self.m & 0x3F
    }

    pub fn to(&self) -> u16 {
        (self.m >> 6) & 0x3F
    }

    pub fn flag(&self) -> u16 {
        (self.m >> 14) & 0x3
    }

    pub fn promotee(&self) -> u16 {
        (self.m >> 12) & 0x3
    }
}

/* Enum for Flags */

pub enum Flag {
    PROMOTE, ENPASSANT, CASTLE, NONE
}

pub enum Promotee {
    KNIGHT, BISHOP, ROOK, QUEEN
}