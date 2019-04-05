pub struct MotionList {
    pub motions: Vec<Motion>
}

pub struct Motion {
    pub motion: u16,
    pub score: i32
}

impl Motion {

    pub fn new() -> Motion {
        Motion {
            motion: MOVE_INT!(0, 0, 0, Flag::NONE as u16),
            score: 0
        }
    }

    pub fn from(&self) -> u16 {
        self.motion & 0x3F
    }

    pub fn to(&self) -> u16 {
        (self.motion >> 6) & 0x3F
    }

    pub fn flag(&self) -> u16 {
        (self.motion >> 14) & 0x3
    }

    pub fn promotee(&self) -> u16 {
        (self.motion >> 12) & 0x3
    }

    pub fn is_promotion(&self) -> bool {
        self.flag() == Flag::PROMOTION as u16
    }

    pub fn is_enpassant(&self) -> bool {
        self.flag() == Flag::ENPASSANT as u16
    }

    pub fn is_castling(&self) -> bool {
        self.flag() == Flag::CASTLING as u16
    }

    pub fn is_none(&self) -> bool {
        self.flag() == Flag::NONE as u16
    }

    pub fn is_prom_queen(&self) -> bool {
        self.promotee() == Promotee::QUEEN as u16
    }

    pub fn is_prom_rook(&self) -> bool {
        self.promotee() == Promotee::ROOK as u16
    }

    pub fn is_prom_bishop(&self) -> bool {
        self.promotee() == Promotee::BISHOP as u16
    }

    pub fn is_prom_knight(&self) -> bool {
        self.promotee() == Promotee::KNIGHT as u16
    }

}

/* Enum for Flags */

pub enum Flag {
    NONE, PROMOTION, ENPASSANT, CASTLING
}

pub enum Promotee {
    QUEEN, ROOK, BISHOP, KNIGHT
}