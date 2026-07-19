#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableRules {
    pub decks: u8,
    pub dealer_stands_soft17: bool,
    pub double_after_split: bool,
    pub resplit_aces: bool,
    pub max_split_hands: u8,
    pub surrender: SurrenderRule,
    pub blackjack_payout: PayoutRatio,
    pub penetration: f32,
    pub min_bet: u32,
    pub max_bet: u32,
    pub double_any_two: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SurrenderRule {
    None,
    Late,
    Early,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PayoutRatio {
    pub numerator: u32,
    pub denominator: u32,
}

impl PayoutRatio {
    pub const THREE_TO_TWO: Self = Self {
        numerator: 3,
        denominator: 2,
    };
    pub const SIX_TO_FIVE: Self = Self {
        numerator: 6,
        denominator: 5,
    };
    pub const EVEN_MONEY: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    pub fn apply(self, bet: u32) -> u32 {
        (bet * self.numerator) / self.denominator
    }

    pub fn total_return(self, bet: u32) -> u32 {
        bet + self.apply(bet)
    }
}

impl Default for TableRules {
    // These are the vegas strip rules as the default set of rules
    fn default() -> Self {
        Self {
            decks: 6,
            dealer_stands_soft17: true,
            double_after_split: true,
            resplit_aces: false,
            max_split_hands: 4,
            surrender: SurrenderRule::Late,
            blackjack_payout: PayoutRatio::THREE_TO_TWO,
            penetration: 0.75,
            min_bet: 10,
            max_bet: 500,
            double_any_two: true,
        }
    }
}

impl TableRules {
    pub fn single_deck() -> Self {
        Self {
            decks: 1,
            ..Self::default()
        }
    }
    pub fn double_deck() -> Self {
        Self {
            decks: 2,
            ..Self::default()
        }
    }
    pub fn house_rules() -> Self {
        Self {
            decks: 8,
            dealer_stands_soft17: false,
            double_after_split: false,
            blackjack_payout: PayoutRatio::SIX_TO_FIVE,
            penetration: 0.5,
            ..Self::default()
        }
    }
}
