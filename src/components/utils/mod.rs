pub mod card_sprite;
/// Define an HTML input with props.
pub mod text_input;

use log::debug;

use crate::components::app::{
    Message,
    State, //
};

pub fn ignored(state: &State, message: &Message) {
    debug!("Ignore message({message}) in current state({state})");
}
