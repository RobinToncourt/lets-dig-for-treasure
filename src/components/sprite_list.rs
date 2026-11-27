use yew::prelude::*;

use web_sys::HtmlSelectElement;

use crate::{
    components::utils::card_sprite::CardSprite,
    engine::{
        CARDS_METADATA,
        CardType, //
    },
};

pub enum SpriteListMessage {
    Selected(Option<CardType>),
}

pub struct SpriteList {
    selected: Option<CardType>,
}

impl Component for SpriteList {
    type Message = SpriteListMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { selected: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SpriteListMessage::Selected(card_type) => self.selected = card_type,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();

            let selected = select
                .value()
                .parse::<usize>()
                .ok()
                .map(|index| CardType::ALL[index]);

            SpriteListMessage::Selected(selected)
        });

        html! {
            <>
            <select onchange={on_change}>
                <option value="" selected={self.selected.is_none()}>
                    { "Select an element" }
                </option>

                {for CARDS_METADATA.iter().enumerate().map(|(index, cm)| {
                    html! {
                        <option
                            value={index.to_string()}
                            selected={self.selected.as_ref() == Some(&cm.card_type())}
                        >
                            { cm.name() }
                        </option>
                    }
                })}
            </select>

            {self.selected.and_then(|card_type|
                html! { <CardSprite
                    card_type={card_type}
                    target_w=220.0
                    target_h=308.0
                /> }.into()
            )}

            </>
        }
    }
}
