enum Plunder {
    CursedSkeleton,
    RadioactiveWaste,
    Worm,
    CursedHeritage,
    SleepingGravedigger,
}

/// If added to the player collection, it will protect from one(1) `Cursed Skeleton`.
enum LoyalDog {
    GermanShepherd,
    GoldenRetriever,
    JackRusselTerrier,
}

enum Collect {
    LoyalDog(LoyalDog),
    PirateCutlass,
    PirateHat,
    Sewer,
    TreasureMap,
    LichKingsCrown,
    DrugStash,
    AntipersonnelMine,
}

enum EndOfGame {
    LotteryTicket,
    SkwirmzWormGod,
    WeddingDress,
    OldMurraysMilk,
    GrannyGruppoGingerCookie,
}

/// Indicates the effect type of the card.
enum CardType {
    /// No effect, only points.
    Treasure,
    /// Resolve its effects on draw.
    Plunder(Plunder),
    /// Resolve its effect when adding the cards to the player collection.
    Collect(Collect),
    /// Resolve its effect when calculating the points.
    EndOfGame(EndOfGame),
}

struct CardMetadata {
    name: &'static str,
    description: &'static str,
    /// The points given by the card when calculating the point at the end of the game.
    value: isize,
    card_type: CardType,
}
impl CardMetadata {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        value: isize,
        card_type: CardType,
    ) -> Self {
        Self {
            name,
            description,
            value,
            card_type,
        }
    }
}

// The game contains multiple instances of the same card.
// The reference is hidden behind a struct that is not Copy so the card remains unique.
// Is this separation interesting/useful/desirable ?
pub struct Card {
    metadata: &'static CardMetadata,
}

//
// Cards metadata.
//

/// Used to reduce line width.
type CM = CardMetadata;

const MUD: CM = CM::new("Gadoue", "", 1, CardType::Treasure);
const LICH_KINGS_CROWN: CM = CM::new(
    "Couronne du Seigneur Liche",
    "",
    5,
    CardType::Collect(Collect::LichKingsCrown),
);
const LOYAL_DOG: CM = CM::new(
    "Chien Fidèle",
    "Laissez-le devant vous lors de la collecte:\n\
    Quand cette carte est devant vous, vous pouvez la placer dans votre pile de score\
    pour ANNULER et DÉFAUSSER un SQUELETTE MAUDIT que vous venez de déterrer!",
    5,
    CardType::Collect(Collect::LoyalDog(LoyalDog::GoldenRetriever)),
);
