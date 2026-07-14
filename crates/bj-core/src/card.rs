// Derive macros: each add a trait impl automatically
// Debug: lets you print with {:?}.
// Clone: lets you .clone().
// Copy: lets you use a value after assignment ( implicit duplicate, stack only);
// PartialEq, eq: lets you use = and !=
// Hash: lets you use it as hashmap key
// Serde: library that lets you serialize and deserialize (save files, netowrks and config);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Suit {
    Clubs,
    Spades,
    Heart,
    Diamond,
}

impl Suit {
    pub fn is_red(self) -> bool {
        // matches is the macro for  the match in a special case of bool.
        matches!(self, Suit::Heart | Suit::Diamond)
    }

    pub fn all() -> [Suit; 4] {
        [Suit::Clubs, Suit::Spades, Suit::Heart, Suit::Diamond]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    // pub fn hi_lo -> it should live in the strategy crate or some other crate as this has no
    // business here

    pub fn is_face(self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }

    pub fn is_ten_valued(self) -> bool {
        matches!(self, Rank::Ten | Rank::Jack | Rank::Queen | Rank::King)
    }

    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rank_str = match self.rank {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        let suit_str = match self.suit {
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
            Suit::Heart => "♥",
            Suit::Diamond => "♦",
        };
        write!(f, "{}{}", rank_str, suit_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_display_format() {
        assert_eq!(Card::new(Rank::Ace, Suit::Spades).to_string(), "A♠");
        assert_eq!(Card::new(Rank::Ten, Suit::Diamond).to_string(), "T♦");
    }

    #[test]
    fn card_is_copy_semantics() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        let also_card = card;
        assert_eq!(card, also_card);
    }
}
