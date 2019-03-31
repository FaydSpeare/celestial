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

    pub fn is_promotion(&self) -> bool {
        self.flag() == Flag::PROMOTION as u16
    }

    pub fn is_enpassant(&self) -> bool {
        self.flag() == Flag::ENPASSANT as u16
    }

    pub fn is_caslting(&self) -> bool {
        self.flag() == Flag::CASTLING as u16
    }

    pub fn is_none(&self) -> bool {
        self.flag() == Flag::NONE as u16
    }

    pub fn is_prom_queen(&self) -> bool {
        self.promotee == Promotee::QUEEN as u16
    }

    pub fn is_prom_rook(&self) -> bool {
        self.promotee == Promotee::ROOK as u16
    }

    pub fn is_prom_bishop(&self) -> bool {
        self.promotee == Promotee::BISHOP as u16
    }

    pub fn is_prom_knight(&self) -> bool {
        self.promotee == Promotee::KNIGHT as u16
    }

}

/* Enum for Flags */

pub enum Flag {
    PROMOTION, ENPASSANT, CASTLING, NONE
}

pub enum Promotee {
    KNIGHT, BISHOP, ROOK, QUEEN
}