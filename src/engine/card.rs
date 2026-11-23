pub static CARDS_METADATA: &[CardMetadata] =
    &include!(concat!(env!("OUT_DIR"), "/card_metadata.rs"));

pub static TEST: usize = 8;

#[allow(clippy::enum_variant_names)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum Treasure {
    Mud,
    FerrymansGold,
    WeddingRing,
    TreasureChest,
    LeprechaunGold,
    ArkOfTheCovenant,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum Plunder {
    CursedSkeleton,
    RadioactiveWaste,
    Worm,
    CursedHeritage,
    SleepingGravedigger,
}

/// If added to the player collection, it will protect from one(1) `Cursed Skeleton`.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum LoyalDog {
    GermanShepherd,
    GoldenRetriever,
    JackRusselTerrier,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
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

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum EndOfGame {
    LotteryTicket,
    SkwirmzWormGod,
    WeddingDress,
    OldMurraysMilk,
    GrannyGruppoGingerCookie,
}

/// Indicates the effect type of the card.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
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

#[cfg_attr(debug_assertions, derive(Debug))]
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

// The game contains multiple instances of the same card.
// The reference is hidden behind a struct that is not Copy so the card remains unique.
// Is this separation interesting/useful/desirable ?
pub struct Card {
    metadata: &'static CardMetadata,
}
