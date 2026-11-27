use std::fmt::Display;

use yew::prelude::*;

use crate::components::{
    main_menu::{
        MainMenu,
        MainMenuMessage, //
    },
    sprite_list::SpriteList,
    utils::ignored, //
};

#[derive(Debug)]
pub enum State {
    MainMenu(MainMenu),
    WaitingForSignalingServer,
    Lobby,
    Game,
    GameOver,
    SpriteList,
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MainMenu(_) => "App::MainMenu",
                Self::WaitingForSignalingServer => "App::WaitingForSignalingServer",
                Self::Lobby => "App::Lobby",
                Self::Game => "App::Game",
                Self::GameOver => "App::GameOver",
                Self::SpriteList => "App::SpriteList",
            }
        )
    }
}
impl Default for State {
    fn default() -> Self {
        Self::MainMenu(MainMenu::default())
    }
}

pub enum Message {
    MainMenu(MainMenuMessage),
}
impl From<MainMenuMessage> for Message {
    fn from(value: MainMenuMessage) -> Self {
        Message::MainMenu(value)
    }
}
// TODO: create and use macro if more `Message` variants.
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Message::({})",
            match self {
                Message::MainMenu(main_menu) => main_menu,
            }
        )
    }
}

#[derive(Default)]
pub struct App {
    state: State,
}
impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (&mut self.state, msg) {
            (State::MainMenu(main_menu), Message::MainMenu(message)) => {
                // main_menu.update(message);
            },
            (app_state, msg) => ignored(app_state, &msg),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            State::MainMenu(_main_menu) => html! { <MainMenu /> },
            State::SpriteList => html! { <SpriteList /> },
            _ => todo!(),
        }
    }
}
