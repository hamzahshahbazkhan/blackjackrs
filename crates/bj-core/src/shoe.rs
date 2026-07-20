use crate::card::{Card, Rank};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Shoe {
    cards: Vec<Card>,
    dealt: usize,
    deck: u8,
    penetration: f32,
}

impl Shoe {
    pub fn new(deck: u8, penetration: f32) -> Self {
        Self {
            cards: Vec::new(),
            dealt: 0,
            deck,
            penetration,
        }
    }
}
