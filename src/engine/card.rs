use std::{
    fmt::Display,
    hash::Hash, //
};

use serde::{
    Deserialize,
    Serialize, //
};
use yew::html::ImplicitClone;

pub static CARDS_METADATA: &[CardMetadata] =
    &include!(concat!(env!("OUT_DIR"), "/card_metadata.rs"));

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Treasure {
    Mud,
    FerrymansGold,
    WeddingRing,
    TreasureChest,
    LeprechaunGold,
    ArkOfTheCovenant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Plunder {
    CursedSkeleton,
    RadioactiveWaste,
    Worm,
    CursedHeritage,
    SleepingGravedigger,
}

/// If added to the player collection, it will protect from one(1) `Cursed Skeleton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoyalDog {
    GermanShepherd,
    GoldenRetriever,
    JackRusselTerrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Collect {
    LoyalDog(LoyalDog),
    PirateCutlass,
    PirateHat,
    Sewer,
    TreasureMap,
    LichKingsCrown,
    DrugStash,
    AntipersonnelMine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndOfGame {
    LotteryTicket,
    SkwirmzWormGod,
    WeddingDress,
    OldMurraysMilk,
    GrannyGruppoGingerCookie,
}

/// Indicates the effect type of the card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    /// No effect, only points.
    Treasure(Treasure),
    /// Resolve its effects on draw.
    Plunder(Plunder),
    /// Resolve its effect when adding the cards to the player collection.
    Collect(Collect),
    /// Resolve its effect when calculating the points.
    EndOfGame(EndOfGame),
}
impl ImplicitClone for CardType {
    fn implicit_clone(&self) -> Self {
        *self
    }
}
impl CardType {
    /// All card types.
    pub const ALL: &[CardType] = &[
        CardType::Treasure(Treasure::Mud),
        CardType::Treasure(Treasure::FerrymansGold),
        CardType::Treasure(Treasure::WeddingRing),
        CardType::Treasure(Treasure::TreasureChest),
        CardType::Treasure(Treasure::LeprechaunGold),
        CardType::Treasure(Treasure::ArkOfTheCovenant),
        CardType::Plunder(Plunder::CursedSkeleton),
        CardType::Plunder(Plunder::RadioactiveWaste),
        CardType::Plunder(Plunder::Worm),
        CardType::Plunder(Plunder::CursedHeritage),
        CardType::Plunder(Plunder::SleepingGravedigger),
        CardType::Collect(Collect::LoyalDog(LoyalDog::GoldenRetriever)),
        CardType::Collect(Collect::LoyalDog(LoyalDog::JackRusselTerrier)),
        CardType::Collect(Collect::LoyalDog(LoyalDog::GermanShepherd)),
        CardType::Collect(Collect::PirateCutlass),
        CardType::Collect(Collect::PirateHat),
        CardType::Collect(Collect::Sewer),
        CardType::Collect(Collect::TreasureMap),
        CardType::Collect(Collect::LichKingsCrown),
        CardType::Collect(Collect::DrugStash),
        CardType::Collect(Collect::AntipersonnelMine),
        CardType::EndOfGame(EndOfGame::LotteryTicket),
        CardType::EndOfGame(EndOfGame::SkwirmzWormGod),
        CardType::EndOfGame(EndOfGame::WeddingDress),
        CardType::EndOfGame(EndOfGame::OldMurraysMilk),
        CardType::EndOfGame(EndOfGame::GrannyGruppoGingerCookie),
    ];
}

#[derive(Debug)]
pub struct CardMetadata {
    card_type: CardType,
    name: &'static str,
    description: &'static str,
    /// The points given by the card when calculating the point at the end of the game.
    value: i8,
    quantity: u8,
}
impl CardMetadata {
    pub fn card_type(&self) -> CardType {
        self.card_type
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn description(&self) -> &'static str {
        self.description
    }
    pub fn value(&self) -> i8 {
        self.value
    }
    pub fn quantity(&self) -> u8 {
        self.quantity
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct CardId(usize);

// The game contains multiple instances of the same card.
// The reference is hidden behind a struct that is not `Copy` so the card remains unique.
#[derive(Debug)]
pub struct Card {
    /// Used to uniquely identify each card.
    id: CardId,
    metadata: &'static CardMetadata,
}
impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.id == other.id
    }
}
impl Eq for Card {}
impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CardView {
    pub id: CardId,
    pub metadata: &'static CardMetadata,
}
impl From<&Card> for CardView {
    fn from(card: &Card) -> Self {
        Self {
            id: card.id,
            metadata: card.metadata,
        }
    }
}
impl PartialEq for CardView {
    fn eq(&self, other: &CardView) -> bool {
        self.id == other.id
    }
}
impl Eq for CardView {}
impl Hash for CardView {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn create_deck() -> Vec<Card> {
    CARDS_METADATA
        .iter()
        .flat_map(|m| std::iter::repeat_n(m, m.quantity as usize))
        .enumerate()
        .map(|(i, metadata)| Card {
            id: CardId(i),
            metadata,
        })
        .collect()
}

// TODO: @FEATURE: add a deck builder to add or remove cards from the deck.

#[cfg(test)]
mod card_tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_120_cards_deck() {
        assert_eq!(120, create_deck().len());
    }

    #[test]
    fn test_cards_has_unique_id() {
        let mut set = HashSet::new();
        for card in create_deck() {
            let card_id = card.id;
            if !set.insert(card_id) {
                panic!("CARD IDs duplicate: {card_id:?}");
            }
        }
    }
}
