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
