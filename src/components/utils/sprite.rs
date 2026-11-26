use crate::engine::{
    CardType,
    Collect,
    EndOfGame,
    LoyalDog,
    Plunder,
    Treasure, //
};

pub static CARDS_SPRITES: &[CardSprite] = &include!(concat!(env!("OUT_DIR"), "/card_sprites.rs"));
pub static CARD_BACK: Sprite = Sprite {
    position: Position { x: 7233, y: 0 },
    dimension: Dimension { w: 241, h: 334 },
};

#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Dimension {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug)]
pub struct Sprite {
    pub position: Position,
    pub dimension: Dimension,
}

#[derive(Debug)]
pub struct CardSprite {
    card_type: CardType,
    sprite: Sprite,
}
impl CardSprite {
    pub fn card_type(&self) -> CardType {
        self.card_type
    }
    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}
