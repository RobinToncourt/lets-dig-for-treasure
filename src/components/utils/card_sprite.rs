use yew::prelude::*;
use yew_autoprops::autoprops;

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

const SPRITES_PATH: &str = "assets/img/sprites.jpg";

const SHEET_W: f64 = 7474.0;
const SHEET_H: f64 = 459.0;

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

#[autoprops]
#[component]
pub fn CardSprite(card_type: CardType, target_w: f64, target_h: f64) -> Html {
    let style = get_sprite(card_type)
        .map(|sprite| {
            let Sprite {
                position: Position { x, y },
                dimension: Dimension { w, h },
            } = sprite;
            let scale_x = target_w / w as f64;
            let scale_y = target_h / h as f64;

            let bg_w = SHEET_W * scale_x;
            let bg_h = SHEET_H * scale_y;
            let bg_x = -(x as f64) * scale_x;
            let bg_y = -(y as f64) * scale_y;

            format!(
                "width: {target_w}px; height: {target_h}px; \
                 background-image: url('{SPRITES_PATH}'); \
                 background-repeat: no-repeat; \
                 background-size: {bg_w:.2}px {bg_h:.2}px; \
                 background-position: {bg_x:.2}px {bg_y:.2}px;",
            )
        })
        .unwrap_or(format!(
            "width: {target_w}px; height: {target_h}px; \
             background-color: red;"
        ));

    html! {
        <div class="card-sprite" style={style} />
    }
}

fn get_sprite(card_type: CardType) -> Option<Sprite> {
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
