use crate::card::Card;
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

pub struct Hand {
    cards: Vec<Card>,
    pub bet: u32,
}

impl Hand {
    pub fn new(bet: u32) -> Self {
        Self {
            cards: Vec::with_capacity(8),
            bet,
        }
    }
    pub fn add_cards(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
    // Above we are returning &[Card] because it is more idiomatic. The caller dosen't need to know
    // that cards is a Vec
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    pub fn first(&self) -> Option<Card> {
        self.cards.first().copied()
    }
    // pub fn value()
    pub fn is_natural(&self) -> bool {
        self.cards.len() == 2 && self.value().best() == 21
    }
    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].rank == self.cards[1].rank
    }
}
