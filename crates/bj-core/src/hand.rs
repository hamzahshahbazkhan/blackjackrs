#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandValue {
    pub hard: u8,
    pub soft: Option<u8>,
}

impl HandValue {
    pub fn best(self) -> u8 {
        match self.soft {
            Some(s) if s < 21 => s,
            _ => self.hard,
        }
    }
    pub fn is_bust(self) -> bool {
        self.hard > 21
    }

    pub fn is_soft(self) -> bool {
        self.soft.map(|s| s <= 21).unwrap_or(false)
        // match self.soft {
        //     Some(s) => s <= 21,
        //     None => false,
        // }
    }
    pub fn is_twenty_one(self) -> bool {
        self.best() == 21
    }
}
