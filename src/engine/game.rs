use rand::prelude::*;

use crate::engine::{
    ExcavationPile,
    ExcavationPileNumber,
    LetsDigForTreasure,
    LetsDigForTreasureResult,
    Player,
    PlayerAction,
    PlayerNames, //
};

/*
 * Game stage.
 */

enum DrawingStage {
    /// The game requires the player to draw at this stage.
    FirstDrawCard,
    /// The player can draw.
    DrawCard,
    /// The player has to select another player.
    SelectAnotherPlayer,
    /// The player has to choose to use a dog.
    UseDog,
}

enum CollectStage {
    /// The player has to select another player.
    SelectAnotherPlayer,
    /// The player has to choose a pile.
    SelectAPile,
    /// The player tells if he wants to use the card.
    UseCard,
}

/// Indicate in which stage the game and is waiting for a `PlayerAction` to advance.
enum Stage {
    /// The drawing stage.
    Drawing(DrawingStage),
    /// The cards are added to the active player collection.
    CollectCards(CollectStage),
    /// The game is over.
    GameOver,
}

pub struct Game {
    players: Vec<Player>,
    /// Indicates how many new turn the active player will take.
    remaining_new_turn: usize,
    excavation_piles: [ExcavationPile; 3],
    stage: Stage,
}
impl LetsDigForTreasure for Game {
    fn new<R: Rng + ?Sized>(rng: &mut R, player_names: PlayerNames) -> Self {
        // TODO: create the deck, shuffle and split in 3.
        // TODO: add randomization to the splitting so each pile is of different size
        // todo: but close to same size.
        let players: Vec<Player> = player_names.0.into_iter().map(Player::new).collect();
        todo!()
    }
    fn get_active_player(&self) -> &Player {
        todo!()
    }
    fn get_remaining_excavation_pile_numbers(&self) -> impl Iterator<Item = ExcavationPileNumber> {
        // Should I iter on ExcavationPileNumber and index the excavation_piles ?
        // That would just move the problem to the indexing.
        // Maybe create a type ?
        self.excavation_piles
            .iter()
            .enumerate()
            .filter(|(i, ep)| !ep.is_empty())
            .map(|(i, _)| i.into())
    }
    fn do_player_action(&mut self, action: PlayerAction) -> LetsDigForTreasureResult {
        todo!()
    }
}
impl Game {
    pub fn test() {}
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players,
            remaining_new_turn: 0,
            excavation_piles: [const { Vec::new() }; 3],
            stage: Stage::Drawing(DrawingStage::FirstDrawCard),
        }
    }
}
