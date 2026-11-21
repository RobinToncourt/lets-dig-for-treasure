use serde::Deserialize;
use serde_json::Value;

use std::{
    env,
    fmt::Write as _,
    fs,
    path::Path, //
};

#[derive(Deserialize)]
struct Raw {
    card_type: Value,
    name: String,
    description: String,
    value: i32,
    quantity: usize,
}

// {"Collect": {"LoyalDog": "GermanShepherd"}}
//   -> CardType::Collect(Collect::LoyalDog(LoyalDog::GermanShepherd))
fn path(ty: &str, v: &Value) -> String {
    match v {
        Value::String(variant) => format!("{ty}::{variant}"),
        Value::Object(m) => {
            let (variant, inner) = m.iter().next().unwrap();
            format!("{ty}::{variant}({})", path(variant, inner))
        }
        _ => panic!("unexpected card_type shape"),
    }
}

fn card_metadata() {
    println!("cargo:rerun-if-changed=cards_metadata.json");
    let raw: Vec<Raw> =
        serde_json::from_str(&fs::read_to_string("assets/cards_metadata.json").unwrap()).unwrap();

    let mut out = String::from("[\n");
    for c in &raw {
        writeln!(
            out,
            "    CardMetadata {{ card_type: {}, name: {:?}, description: {:?}, value: {}, quantity: {} }},",
            path("CardType", &c.card_type), c.name, c.description, c.value, c.quantity
        ).unwrap();
    }
    out.push_str("]\n");

    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("card_metadata.rs"),
        out,
    )
    .unwrap();
}

#[derive(Deserialize)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Deserialize)]
struct Dimension {
    w: u32,
    h: u32,
}

#[derive(Deserialize)]
struct Sprite {
    position: Position,
    dimension: Dimension,
}

#[derive(Deserialize)]
pub struct CardSprite {
    card_type: Value,
    sprite: Sprite,
}

fn card_sprites() {
    println!("cargo:rerun-if-changed=cards_metadata.json");
    let sprites: Vec<CardSprite> =
        serde_json::from_str(&fs::read_to_string("assets/sprites.json").unwrap()).unwrap();

    let mut out = String::from("[\n");
    for s in &sprites {
        writeln!(
            out,
            "    CardSprite {{ card_type: {}, sprite: Sprite {{ position: Position {{ x: {}, y:{} }}, dimension: Dimension {{ w: {}, h: {} }} }} }},",
            path("CardType", &s.card_type), s.sprite.position.x, s.sprite.position.y, s.sprite.dimension.w, s.sprite.dimension.h,
        ).unwrap();
    }
    out.push_str("]\n");

    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("card_sprites.rs"),
        out,
    )
    .unwrap();
}

fn main() {
    card_metadata();
    card_sprites();
}
