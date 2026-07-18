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
