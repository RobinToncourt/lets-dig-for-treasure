use crate::engine::{
    CardType,
    Collect,
    EndOfGame,
    LoyalDog,
    Plunder,
    Treasure, //
};

pub static CARDS_SPRITES: &[CardTypeSprite] =
    &include!(concat!(env!("OUT_DIR"), "/card_sprites.rs"));
pub static CARD_BACK: Sprite = Sprite {
    position: Position { x: 7233, y: 0 },
    dimension: Dimension { w: 241, h: 334 },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dimension {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sprite {
    pub position: Position,
    pub dimension: Dimension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardTypeSprite {
    card_type: CardType,
    sprite: Sprite,
}
impl CardTypeSprite {
    pub fn card_type(&self) -> CardType {
        self.card_type
    }
    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}

pub fn get_sprite(card_type: CardType) -> Option<Sprite> {
    for card_sprite in CARDS_SPRITES {
        if card_type == card_sprite.card_type {
            return Some(card_sprite.sprite);
        }
    }

    None
}

#[cfg(test)]
mod sprite_tests {
    use super::*;

    #[test]
    fn test_get_sprite() {
        let expected = Sprite {
            position: Position { x: 3829, y: 0 },
            dimension: Dimension { w: 241, h: 338 },
        };
        assert_eq!(
            Some(expected),
            get_sprite(CardType::Collect(Collect::LoyalDog(
                LoyalDog::GoldenRetriever
            ))),
        );
    }
}
