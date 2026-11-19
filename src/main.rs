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
    SetNickname(String),
    SetLobbyName(String),
    SetSignalingServerUrl(String),
    Join,
}

#[derive(Default)]
struct MainMenu {
    nickname: String,
    lobby_name: String,
    signaling_server_url: String,
}

enum Message {
    MainMenu(MainMenuMessage),
}
impl From<MainMenuMessage> for Message {
    fn from(value: MainMenuMessage) -> Self {
        Message::MainMenu(value)
    }
}

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
                    "MainMenu"
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
            (Self::MainMenu(main_menu), Message::MainMenu(message)) => {
                match message {
                    MainMenuMessage::SetNickname(n) => main_menu.nickname = n,
                    MainMenuMessage::SetLobbyName(l) => main_menu.lobby_name = l,
                    MainMenuMessage::SetSignalingServerUrl(s) => main_menu.signaling_server_url = s,
                    MainMenuMessage::Join => debug!("JOIN"),
                }
            }
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
                    <table>
                        <tbody>
                            <tr>
                                <th>{"Nickname: "}</th>
                                <td>
                                    <TextInput
                                        on_change={on_nickname_change}
                                        value={nickname.clone()}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <th>{"Lobby name: "}</th>
                                <td>
                                    <TextInput
                                        on_change={on_lobby_name_change}
                                        value={lobby_name.clone()}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <th>{"Signaling server URL: "}</th>
                                <td>
                                    <TextInput
                                        on_change={on_signaling_server_url_change}
                                        value={nickname.clone()}
                                    />
                                </td>
                            </tr>
                        </tbody>
                        <tfoot>
                            <td colspan="2">
                                <button
                                    onclick={onclick_join}
                                    disabled={is_join_disable}
                                >
                                    {"Join"}
                                </button>
                            </td>
                        </tfoot>
                    </table>
                }
            }
        }
    }
}
