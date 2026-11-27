mod card;
pub mod game;

/*
 * Exports.
 */

pub use card::{
    CARDS_METADATA,
    CardId,
    CardType,
    CardView,
    Collect,
    EndOfGame,
    LoyalDog,
    Plunder,
    Treasure, //
};

/*
 * Imports.
 */

use std::{
    hash::{
        Hash,
        Hasher, //
    },
    ops::{
        Index,
        IndexMut, //
    },
};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use serde::{
    Deserialize,
    Serialize, //
};

use crate::engine::card::{
    Card, //
};

/*
 * Types.
 */

pub const MIN_PLAYERS: usize = 2;
pub const MAX_PLAYERS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerNumberError {
    /// The game requires at least two(2) players.
    NotEnoughPlayers,
    /// The game allow to five(5) players max.
    TooManyPlayers,
}
impl std::error::Error for PlayerNumberError {}
impl std::fmt::Display for PlayerNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotEnoughPlayers => "not enough players",
                Self::TooManyPlayers => "too many players",
            }
        )
    }
}

struct PlayerNames(Vec<String>);
impl PlayerNames {
    fn new(player_names: Vec<String>) -> Result<Self, PlayerNumberError> {
        if player_names.len() < MIN_PLAYERS {
            Err(PlayerNumberError::NotEnoughPlayers)
        } else if player_names.len() > MAX_PLAYERS {
            Err(PlayerNumberError::TooManyPlayers)
        } else {
            Ok(Self(player_names))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct PlayerId(usize);
impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Player {
    /// Used to differentiate between players.
    id: PlayerId,
    name: String,
    /// The cards collected by the player.
    collection: Vec<Card>,
    /// The list of `Loyal Dog`s that protect the player from `Cursed Skeleton`.
    loyal_dogs: Vec<Card>,
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Player {}
impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Player {
    fn new(id: usize, name: String) -> Self {
        Self {
            id: PlayerId(id),
            name,
            collection: vec![],
            loyal_dogs: vec![],
        }
    }

    fn player_id(&self) -> PlayerId {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn collection(&self) -> &[Card] {
        &self.collection
    }
    fn loyal_dogs(&self) -> &[Card] {
        &self.loyal_dogs
    }
    fn all_cards(&self) -> impl Iterator<Item = &Card> {
        self.collection.iter().chain(self.loyal_dogs.iter())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerView {
    pub id: PlayerId,
    pub name: String,
    pub collection: AmbiguousSize,
    pub loyal_dogs: Vec<CardView>,
}
impl From<&Player> for PlayerView {
    fn from(player: &Player) -> Self {
        let Player {
            id,
            name,
            collection,
            loyal_dogs,
        } = player;
        Self {
            id: *id,
            name: name.clone(),
            collection: player_collection_to_ambiguous_size(collection),
            loyal_dogs: loyal_dogs.iter().map(CardView::from).collect(),
        }
    }
}
impl Hash for PlayerView {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    pub player: PlayerView,
    pub cards: Vec<CardView>,
    pub value: i16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ExcavationPileNumber {
    One = 0,
    Two = 1,
    Three = 2,
}
impl ExcavationPileNumber {
    pub const ALL: [Self; 3] = [Self::One, Self::Two, Self::Three];
}
impl std::fmt::Display for ExcavationPileNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::One => "one",
                Self::Two => "two",
                Self::Three => "three",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct PerPile<T>([T; 3]);
impl<T> Index<ExcavationPileNumber> for PerPile<T> {
    type Output = T;

    fn index(&self, index: ExcavationPileNumber) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl<T> IndexMut<ExcavationPileNumber> for PerPile<T> {
    fn index_mut(&mut self, index: ExcavationPileNumber) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
impl<T> PerPile<T> {
    /// Iterate over the piles.
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

/// Roughly qualify a deck size, the players should not know the exact size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguousSize {
    Many,
    Several,
    Few,
    Empty,
}
fn player_collection_to_ambiguous_size(collection: &[Card]) -> AmbiguousSize {
    match collection.len() {
        30.. => AmbiguousSize::Many,
        5..30 => AmbiguousSize::Several,
        1..5 => AmbiguousSize::Few,
        0 => AmbiguousSize::Empty,
    }
}
fn excavation_pile_to_ambiguous_size(collection: &[Card]) -> AmbiguousSize {
    match collection.len() {
        25.. => AmbiguousSize::Many,
        10..25 => AmbiguousSize::Several,
        1..10 => AmbiguousSize::Few,
        0 => AmbiguousSize::Empty,
    }
}

/*
 * Player action.
 */

/// Represents the different actions the player can do.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
enum Action {
    /// Acknowledge to the game engine it can continue.
    Continue,
    /// The excavation pile to draw from.
    Draw(ExcavationPileNumber),
    /// The player stop digging and cash in the cards.
    StopDigging,
    /// The other player selected.
    SelectPlayer(PlayerId),
    /// The excavation pile on which to perform the action.
    /// (similar to Draw but semantically different)
    SelectPile(ExcavationPileNumber),
    /// The player selected card.
    SelectCard(CardId),
    /// The player reordered cards.
    OrderedCards(Vec<CardId>),
    /// Confirm, on not, the use of a card.
    Confirm(bool),
}

/*
 * Outcomes and errors.
 */

type ActionResult = Result<Vec<Event>, ActionError>;

/// The different events that can happen from a player `Action`.
/// Those events are to be known by all players.
/// ex: `CardDrawn` -> `DugCardsLost` -> `TurnEnded` -> `TurnSkipped` -> `TurnStarted`
#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    /// The card dug from an excavation pile.
    CardDrawn { card: CardView, pile: ExcavationPileNumber },
    /// The player use one of its dog to protect himself from a cursed skeleton.
    DogUsed { player: PlayerId, dog: CardView, skeleton: CardView },
    /// Cards lost before behind collected.
    DugCardsLost { player: PlayerId, cards: Vec<CardView> },
    /// Cards collected by a player.
    CardsCollected { player: PlayerId, cards: Vec<CardView> },
    /// Cards remove from an excavation pile.
    PileCardsDiscarded { pile: ExcavationPileNumber, cards: Vec<CardView> },
    /// The player inspected the excavation pile.
    PileInspected { pile: ExcavationPileNumber, player: PlayerId },
    /// Card stolen by player A from player B.
    CardStolen { from: PlayerId, to: PlayerId, card: CardView },
    /// Card given by player A from player B.
    CardGiven { from: PlayerId, to: PlayerId, card: CardView },
    /// The player obtained a card from an excavation pile.
    CardTaken { player: PlayerId, card: CardView, pile: ExcavationPileNumber },
    /// Which excavation pile has been destroyed.
    PileDestroyed(ExcavationPileNumber),
    /// This player gained a number of turn from a card.
    GainedTurn { player: PlayerId, from_card: CardView },
    /// Indicate that this player next turn will be skipped.
    TurnWillBeSkipped { originator: PlayerId, target: PlayerId },
    /// This player turn is over with the reason.
    TurnEnded { player: PlayerId, reason: TurnEndReason },
    /// The player turn has been skipped.
    TurnSkipped(PlayerId),
    /// This player turn begins.
    TurnStarted(PlayerId),
    /// The is no more cards drawable, results in the end of the game.
    NoMoreCards,
    /// The game is over and will not accept `Action` anymore.
    GameEnded,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prompt {
    source: CardView,
    request: Request,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Request {
    /// The player has to select an excavation pile.
    SelectExcavationPile,
    /// Ask the player if he wants to use the card.
    UseCard,
    /// The player can observe cards.
    ObserveCards(Vec<CardView>),
    /// The player is asked to choose a card.
    SelectCard(Vec<CardView>),
    /// The player is asked to order the cards.
    OrderCards(Vec<CardView>),
    /// The player is asked to choose a player.
    SelectPlayer(Vec<PlayerView>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TurnEndReason {
    /// A drawn card ended the turn.
    EndingCard(CardView),
    /// The player ended its turn.
    PlayerCollected,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameOver {
    /// Which emptied pile provoked the game end.
    pub emptied_pile: ExcavationPileNumber,
    /// Contain the players, theirs cards(revealed) and their score.
    pub winners: Vec<Score>,
    pub losers: Vec<Score>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionError {
    /// Tried to select an empty excavation pile.
    ExcavationPileIsEmpty(ExcavationPileNumber),
    /// The referred player does not exist.
    NoPlayerWithId(PlayerId),
    /// The player can't be selected.
    /// Can happen if the player is expected to select another player but select himself.
    CantSelectThisPlayer,
    /// The card can't be selected.
    /// Can happen if the player chose a card he can't take.
    CantSelectThisCard,
    /// The action was unexpected compared with the game state.
    InvalidAction,
    /// The game is over.
    GameIsOver,
}
impl std::error::Error for ActionError {}
impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ExcavationPileIsEmpty(nb) => write!(f, "excavation pile {nb} is empty"),
            Self::NoPlayerWithId(id) => write!(f, "no player with id {id}"),
            Self::CantSelectThisPlayer => write!(f, "cannot select this player"),
            Self::CantSelectThisCard => write!(f, "cannot select this card"),
            Self::InvalidAction => write!(f, "invalid action"),
            Self::GameIsOver => write!(f, "game is over"),
        }
    }
}

/*
 * Expected interface.
 */

/// Methods to interact with the game.
trait LetsDigForTreasure {
    /// Create a new game.
    /// rng: used to randomize the game.
    fn new(seed: <ChaCha8Rng as SeedableRng>::Seed, player_names: PlayerNames) -> Self;

    /// Return the active player.
    fn active_player(&self) -> PlayerView;

    /// Return the list of players.
    fn players(&self) -> Vec<PlayerView>;

    /// Returns the cards of the player.
    fn collection(&self, player: PlayerId) -> Vec<CardView>;

    /// Returns the cards dig out during this turn.
    fn turn_dig_out_cards(&self) -> Vec<CardView>;

    /// Returns the excavation pile sizes.
    fn excavation_pile_sizes(&self) -> PerPile<AmbiguousSize>;

    /// Return the action the game is waiting from the player.
    fn pending(&self) -> Option<Prompt>;

    /// Execute the player and advance the game accordingly.
    fn do_player_action(&mut self, action: Action) -> ActionResult;

    /// The result of the game if it is over.
    fn game_result(&self) -> Option<GameOver>;

    /// Returns the score of the players, it is calculated from the game current state.
    fn scores(&self) -> Vec<Score>;
}
