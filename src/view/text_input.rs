use yew::{
    Callback,
    Html,
    prelude::*, //
};
use yew_autoprops::autoprops;

use wasm_bindgen::{
    JsCast,
    UnwrapThrowExt, //
};

use web_sys::{
    Event,
    HtmlInputElement,
    InputEvent, //
};

use log::debug;

/// Controlled text input component.
#[autoprops]
#[component]
pub fn TextInput(
    #[prop_or_default] id: AttrValue,
    #[prop_or_default] name: AttrValue,
    value: AttrValue,
    on_change: Callback<AttrValue>,
) -> Html {
    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input
            type="text"
            id={id}
            name={name}
            value={value}
            oninput={oninput}
        />
    }
}

fn get_value_from_input_event(e: InputEvent) -> AttrValue {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    debug!("target: {}", target.value());
    target.value().into()
}
