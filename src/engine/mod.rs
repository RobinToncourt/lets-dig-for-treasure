pub mod card;
mod game;

/*
 * Exports.
 */
pub use game::Game;

/*
 * Imports.
 */

use rand::prelude::*;

use card::Card;

/*
 * Types.
 */

enum PlayerNumberError {
    /// The game requires at least two(2) players.
    NotEnoughPlayers,
    /// The game allow to five(5) players max.
    TooManyPlayers,
}

struct PlayerNames(Vec<String>);
impl PlayerNames {
    fn new(player_names: Vec<String>) -> Result<Self, PlayerNumberError> {
        if player_names.len() < 2 {
            Err(PlayerNumberError::NotEnoughPlayers)
        } else if player_names.len() > 5 {
            Err(PlayerNumberError::TooManyPlayers)
        } else {
            Ok(Self(player_names))
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PlayerId(usize);

pub struct Player {
    /// Used to differentiate between players.
    // Should I ensure that names are different instead ?
    id: PlayerId,
    name: String,
    /// The cards collected by the player.
    collection: Vec<Card>,
    /// The number of `Loyal Dog`s that protect the player from `Cursed Skeleton`.
    loyal_dogs: Vec<Card>,
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
    fn collection_size(&self) -> AmbiguousSize {
        todo!()
    }
    fn loyal_dogs(&self) -> &[Card] {
        &self.loyal_dogs
    }
}

enum ExcavationPileNumber {
    One,
    Two,
    Three,
}
impl From<usize> for ExcavationPileNumber {
    fn from(number: usize) -> Self {
        match number {
            0 => ExcavationPileNumber::One,
            1 => ExcavationPileNumber::Two,
            2 => ExcavationPileNumber::Three,
            // It is not right to panic
            _ => unreachable!(),
        }
    }
}

// If I use these struct maybe I can avoid array indexing check ?
struct ExcavationPileOne;
struct ExcavationPileTwo;
struct ExcavationPileThree;

/// Roughly qualify a deck size, the players should not know the exact size.
// TODO: add more ?
enum AmbiguousSize {
    Alot,
    Some,
    Few,
    Empty,
}

struct ExcavationPileSizes {
    first: AmbiguousSize,
    second: AmbiguousSize,
    third: AmbiguousSize,
}

/*
 * Player actions.
 */

enum DrawingPlayerAction {
    /// The player draw a card from an excavation pile.
    // Should it take a &ExcavationPile to not have to check for emptiness ?
    DrawCard(ExcavationPileNumber),
    /// Select an excavation pile.
    SelectExcavationPile(ExcavationPileNumber),
    /// Select a player by `PlayerId`.
    SelectPlayer(PlayerId),
    /// The player decision on using the dog protection.
    UseDog(bool),
    /// The player chooses to end its turn and add the cards to its collection.
    CollectCards,
}

enum CollectPlayerAction {
    SelectPlayer(PlayerId),
    SelectCard(Card),
    SelectExcavationPile(ExcavationPileNumber),
    UseCard(bool),
}

/// Represents the different actions the player can do.
enum PlayerAction {
    /// Actions that can be done during the draw phase.
    Draw(DrawingPlayerAction),
    /// Actions that can be done during the collect phase.
    Collect(CollectPlayerAction),
}

/*
 * Outcomes and errors.
 */

// Are those names good ?
type LetsDigForTreasureResult<'g> = Result<LetsDigForTreasureOutcome<'g>, LetsDigForTreasureError>;

enum DrawOutcome<'g> {
    /// Waiting for the active player to select another player to proceed.
    /// The list contains the other players.
    ChooseAnotherPlayer(Vec<&'g Player>),
    /// The player can see (some of) the cards of an excavation pile.
    ExcavationPileCards(Vec<&'g Card>),
    /// The player has a dog that can protect him, will he use it ?
    DogCanProtect,
}

enum CollectOutcome<'g> {
    /// Waiting for the active player to select another player to proceed.
    ChooseAnotherPlayer(Vec<&'g Player>),
    /// Ask if the player wants to use the card.
    UseCard(&'g Card),
    /// The player must choose an excavation pile.
    SelectExcavationPile,
    /// The player must choose a card in the list.
    SelectCard(Vec<&'g Card>),
}

// Are the Draw and Collect variants interesting ?
enum LetsDigForTreasureOutcome<'g> {
    /// The player action was completed, the player can draw again.
    ActionDone,
    /// Outcomes that can happens during the draw phase.
    Draw(DrawOutcome<'g>),
    /// Outcomes that can happens during the collect phase.
    Collect(CollectOutcome<'g>),
    /// Indicates that the player turn ended, typically the player draw a turn ending card.
    TurnOver,
    /// Indicate that the game ended, typically an excavation pile was emptied.
    GameOver,
}

// Should it be separated like `Outcome` ?
enum LetsDigForTreasureError {
    /// Tried to select an empty excavation pile.
    ExcavationPileIsEmpty(ExcavationPileNumber),
    /// The referred player does not exist.
    NoPlayerWithId(PlayerId),
    /// The card referred can't be selected.
    CantSelectThisCard,
    /// The action was unexpected compared with the game state.
    InvalidAction,
    /// The game is over.
    GameIsOver,
}

/*
 * Expected interface.
 */

/// Methods to interact with the game.
trait LetsDigForTreasure {
    /// Create a new game.
    /// rng: used to randomize the game.
    fn new<R: Rng + ?Sized>(rng: &mut R, player_names: PlayerNames) -> Self;

    /// Return the active player.
    fn get_active_player(&self) -> &Player;

    /// Returns the excavation pile sizes.
    fn get_excavation_pile_sizes(&self) -> ExcavationPileSizes;

    /// Execute the player and advance the game accordingly.
    fn do_player_action(&mut self, action: PlayerAction) -> LetsDigForTreasureResult<'_>;
}
