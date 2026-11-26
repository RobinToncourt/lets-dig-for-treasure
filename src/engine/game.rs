use rand::SeedableRng;

use rand_chacha::ChaCha8Rng;

use crate::engine::{
    PerPile,
    Player,
    PlayerId,
    card::Card, //
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

type ExcavationPile = Vec<Card>;

pub struct Game {
    rng: ChaCha8Rng,
    players: Vec<Player>,
    active_player: PlayerId,
    /// Indicates how many new turn the active player will take.
    remaining_new_turn: usize,
    excavation_piles: PerPile<ExcavationPile>,
    discard_pile: Vec<Card>,
    stage: Stage,
    turn_number: usize,
    loto_ticket_collected_on_last_turn: bool,
}
// impl LetsDigForTreasure for Game {
//     fn new(rng: [u8; 32], player_names: PlayerNames) -> Self {
//         // TODO: create the deck, shuffle and split in 3.
//         // TODO: add randomization to the splitting so each pile is of different size
//         // todo: but close to same size.
//         let players: Vec<Player> = player_names.0.into_iter().map(Player::new).collect();
//         todo!()
//     }
//     fn get_active_player(&self) -> &Player {
//         todo!()
//     }
//     fn get_remaining_excavation_pile(&self) -> RemainingExcavationPiles {
//         RemainingExcavationPiles {
//             first: !self.excavation_piles[0].is_empty(),
//             second: !self.excavation_piles[1].is_empty(),
//             third: !self.excavation_piles[2].is_empty(),
//         }
//     }
//     fn do_player_action(&mut self, action: PlayerAction) -> LetsDigForTreasureResult {
//         todo!()
//     }
// }
impl Game {
    pub fn new(seed: [u8; 32], players: Vec<Player>) -> Self {
        let active_player = players[0].player_id();
        Self {
            rng: ChaCha8Rng::from_seed(seed),
            players,
            active_player,
            remaining_new_turn: 0,
            // TODO: create the piles.
            excavation_piles: PerPile([const { Vec::new() }; 3]),
            // 30 because there is 120 cards and 30 * 2 * 2 = 120, Vec growth is capacity *2.
            discard_pile: Vec::with_capacity(30),
            stage: Stage::Drawing(DrawingStage::FirstDrawCard),
            turn_number: 0,
            loto_ticket_collected_on_last_turn: false,
        }
    }
}
