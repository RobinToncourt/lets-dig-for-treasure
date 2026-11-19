mod engine;
mod net;
mod update;
mod view;

use std::fmt::Display;
use yew::prelude::*;

use log::{
    debug, //
    info,
};

use crate::view::text_input::TextInput;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    info!("Let's Dig for Treasure!");
    yew::Renderer::<App>::new().render();
}

enum MainMenuMessage {
    SetNickname(AttrValue),
    SetLobbyName(AttrValue),
    SetSignalingServerUrl(AttrValue),
    Join,
}

#[cfg_attr(debug_assertions, derive(Debug))]
struct MainMenu {
    nickname: AttrValue,
    lobby_name: AttrValue,
    signaling_server_url: AttrValue,
}
impl Default for MainMenu {
    fn default() -> Self {
        Self {
            nickname: AttrValue::default(),
            lobby_name: AttrValue::default(),
            signaling_server_url: AttrValue::Static(
                "wss://web-socket-signaling-server.onrender.com/<lobby_name>",
            ),
        }
    }
}
impl Display for MainMenuMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetNickname(_) => "MainMenuMessage::SetNickname",
                Self::SetLobbyName(_) => "MainMenuMessage::SetLobbyName",
                Self::SetSignalingServerUrl(_) => "MainMenuMessage::SetSignalingServerUrl",
                Self::Join => "MainMenuMessage::Join",
            }
        )
    }
}

enum Message {
    MainMenu(MainMenuMessage),
}
impl From<MainMenuMessage> for Message {
    fn from(value: MainMenuMessage) -> Self {
        Message::MainMenu(value)
    }
}
// TODO: create and use macro if more `Message` variants.

#[cfg_attr(debug_assertions, derive(Debug))]
enum App {
    MainMenu(MainMenu),
}
impl Default for App {
    fn default() -> Self {
        Self::MainMenu(MainMenu::default())
    }
}
impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MainMenu(_) => {
                    "App::MainMenu"
                }
            }
        )
    }
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (self, msg) {
            (Self::MainMenu(main_menu), Message::MainMenu(message)) => match message {
                MainMenuMessage::SetNickname(n) => main_menu.nickname = n,
                MainMenuMessage::SetLobbyName(l) => main_menu.lobby_name = l,
                MainMenuMessage::SetSignalingServerUrl(s) => main_menu.signaling_server_url = s,
                MainMenuMessage::Join => debug!("JOIN"),
            },
            (app_state, msg) => debug!("Message ignored."),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            Self::MainMenu(main_menu) => {
                let MainMenu {
                    nickname,
                    lobby_name,
                    signaling_server_url,
                } = main_menu;

                let on_nickname_change = ctx.link().callback(MainMenuMessage::SetNickname);
                let on_lobby_name_change = ctx.link().callback(MainMenuMessage::SetLobbyName);
                let on_signaling_server_url_change =
                    ctx.link().callback(MainMenuMessage::SetSignalingServerUrl);

                let onclick_join = ctx.link().callback(|_| MainMenuMessage::Join);
                // TODO: check for lobby_name constrains
                // todo: and signaling server URL contains <lobby_name>
                let is_join_disable = false;

                html! {
                    <div class="menu-box">
                        <div class="fields-grid">
                            <label for="nickname">{"Nickname"}</label>
                            <TextInput
                                id="nickname"
                                name="nickname"
                                on_change={on_nickname_change}
                                value={nickname}
                            />

                            <label for="lobby">{"Lobby Name"}</label>
                            <TextInput
                                id="lobby"
                                name="lobby"
                                on_change={on_lobby_name_change}
                                value={lobby_name}
                            />

                            <label for="signaling">{"Signaling Server URL"}</label>
                            <TextInput
                                id="signaling"
                                name="signaling"
                                on_change={on_signaling_server_url_change}
                                value={signaling_server_url}
                            />
                        </div>

                        <button
                            class="join-btn"
                            onclick={onclick_join}
                            disabled={is_join_disable}
                        >
                            {"Join"}
                        </button>
                    </div>
                }
            }
        }
    }
}
