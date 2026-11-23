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

pub struct Player {
    name: String,
    /// The cards collected by the player.
    collection: Vec<Card>,
    /// The number of `Loyal Dog`s that protect the player from `Cursed Skeleton`.
    loyal_dogs: Vec<Card>,
}
impl Player {
    fn new(name: String) -> Self {
        Self {
            name,
            collection: vec![],
            loyal_dogs: vec![],
        }
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

type ExcavationPile = Vec<Card>;

struct ExcavationPiles {
    first: ExcavationPile,
    second: ExcavationPile,
    third: ExcavationPile,
}

/*
 * Player actions.
 */

enum DrawingPlayerAction {
    /// The player draw a card from an excavation pile.
    // Should it take a &ExcavationPile to not have to check for emptiness ?
    DrawCard(ExcavationPileNumber),
    /// Select a player by name.
    // What if two players have the same name ? I also need to check if the player name exists.
    // Should it take a &Player ?
    SelectPlayer(String),
    /// The player decision on using the dog protection.
    UseDog(bool),
    /// The player chooses to end its turn and add the cards to its collection.
    CollectCards,
}

enum CollectPlayerAction {}

/// Represents the different actions the player can do.
enum PlayerAction {
    Draw(DrawingPlayerAction),
    Collect(CollectPlayerAction),
}

/*
 * Outcome and errors.
 */

type LetsDigForTreasureResult = Result<LetsDigForTreasureOutcome, LetsDigForTreasureError>;

enum LetsDigForTreasureOutcome {
    NextPlayer,
}

enum LetsDigForTreasureError {
    ExcavationPileIsEmpty(ExcavationPileNumber),
    GameIsOver,
}

/*
 * Expected behavior.
 */

/// Methods to interact with the game.
trait LetsDigForTreasure: Sized {
    /// Create a new game.
    fn new<R: Rng + ?Sized>(rng: &mut R, player_names: PlayerNames) -> Self;

    fn get_active_player(&self) -> &Player;

    /// Returns the excavation piles remaining.
    // Should `LetsDigForTreasure::do_player_action` returns this after each move instead ?
    fn get_remaining_excavation_pile_numbers(&self) -> impl Iterator<Item = ExcavationPileNumber>;

    fn do_player_action(&mut self, action: PlayerAction) -> LetsDigForTreasureResult;
}
