use rand::prelude::*;

use crate::engine::{
    DrawingStage,
    ExcavationPile,
    ExcavationPileNumber,
    GameStage,
    LetsDigForTreasure,
    LetsDigForTreasureResult,
    Player,
    PlayerAction,
    PlayerNames, //
};

pub struct Game {
    players: Vec<Player>,
    /// Indicates how many new turn the active player will take.
    remaining_new_turn: usize,
    excavation_piles: [ExcavationPile; 3],
    game_stage: GameStage,
}
impl LetsDigForTreasure for Game {
    fn new<R: Rng + ?Sized>(rng: &mut R, player_names: PlayerNames) -> Self {
        // TODO: create the deck, shuffle and split in 3.
        // TODO: add randomization to the splitting so each pile is of different size
        // todo: but close to same size.
        let players: Vec<Player> = player_names.0.into_iter().map(Player::new).collect();
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
            game_stage: GameStage::Drawing(DrawingStage::FirstDrawCard),
        }
    }
}
