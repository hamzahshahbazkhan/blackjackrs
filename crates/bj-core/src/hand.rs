use crate::card::{Card, Rank};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandValue {
    pub hard: u8,
    pub soft: Option<u8>,
}

impl HandValue {
    pub fn best(self) -> u8 {
        match self.soft {
            Some(s) if s <= 21 => s,
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
    pub fn value(&self) -> HandValue {
        let mut hard: u8 = 0;
        let mut aces: u8 = 0;
        for card in &self.cards {
            match card.rank {
                Rank::Ace => {
                    hard += 1;
                    aces += 1;
                }
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => hard += 10,
                Rank::Two => hard += 2,
                Rank::Three => hard += 3,
                Rank::Four => hard += 4,
                Rank::Five => hard += 5,
                Rank::Six => hard += 6,
                Rank::Seven => hard += 7,
                Rank::Eight => hard += 8,
                Rank::Nine => hard += 9,
            }
        }
        let soft = if aces > 0 && hard + 10 <= 21 {
            Some(hard + 10)
        } else {
            None
        };

        HandValue { hard, soft }
    }
    // Read the BJ rulese about the natural. You will get the answer for your to be questions
    pub fn is_natural(&self) -> bool {
        self.cards.len() == 2 && self.value().best() == 21
    }
    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].rank == self.cards[1].rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn new_hand_starts_empty() {
        let hand = Hand::new(100);

        assert!(hand.is_empty());
        assert_eq!(hand.len(), 0);
        assert_eq!(hand.bet, 100);
    }
    #[test]
    fn first_returns_the_first_card() {
        let mut hand = Hand::new(0);

        let ace = Card::new(Rank::Ace, Suit::Spades);
        hand.add_cards(ace);
        hand.add_cards(Card::new(Rank::King, Suit::Hearts));

        assert_eq!(hand.first(), Some(ace));
    }

    #[test]
    fn computes_soft_hand_correctly() {
        let mut hand = Hand::new(0);

        hand.add_cards(Card::new(Rank::Ace, Suit::Spades));
        hand.add_cards(Card::new(Rank::Six, Suit::Hearts));

        let value = hand.value();

        assert_eq!(value.hard, 7);
        assert_eq!(value.soft, Some(17));
        assert_eq!(value.best(), 17);
        assert!(value.is_soft());
    }

    #[test]
    fn detects_natural_blackjack() {
        let mut hand = Hand::new(0);

        hand.add_cards(Card::new(Rank::Ace, Suit::Spades));
        hand.add_cards(Card::new(Rank::King, Suit::Hearts));

        assert!(hand.is_natural());
    }

    #[test]
    fn detects_pair() {
        let mut hand = Hand::new(0);

        hand.add_cards(Card::new(Rank::Eight, Suit::Spades));
        hand.add_cards(Card::new(Rank::Eight, Suit::Hearts));

        assert!(hand.is_pair());
    }

    #[test]
    fn hand_value_reports_bust() {
        let value = HandValue {
            hard: 22,
            soft: None,
        };

        assert!(value.is_bust());
        assert_eq!(value.best(), 22);
    }
}
