use crate::card::Card;
use rand;

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
    pub fn new_shuffled(decks: u8, penetration: f32, rng: &mut impl rand::Rng) -> Self {
        let mut shoe = Self::new(decks, penetration);
        shoe.shuffle(rng);
        shoe
    }
    pub fn shuffle(&mut self, rng: &mut impl rand::Rng) {
        use rand::seq::SliceRandom;
        self.populate();
        self.cards.shuffle(rng);
        self.dealt = 0;
    }
    pub fn draw(&mut self) -> Option<Card> {
        if self.dealt < self.cards.len() {
            let card = self.cards[self.dealt];
            self.dealt += 1;
            Some(card)
        } else {
            None
        }
    }

    pub fn needs_reshuffle(&self) -> bool {
        let fraction_dealt = self.dealt as f32 / self.cards.len() as f32;
        fraction_dealt >= self.penetration
    }

    pub fn decks_remaining(&self) -> f32 {
        (self.cards.len() - self.dealt) as f32 / 52.0
    }

    pub fn dealt_cards(&self) -> &[Card] {
        &self.cards[..self.dealt]
    }
}
