use std::fmt::Display;

use yew::{
    AttrValue,
    Component,
    Context,
    Html, //
    html,
};

use crate::components::utils::text_input::TextInput;

/// The sequence in signaling server URL that will be replaced by the lobby name.
const LOBBY_NAME_REPLACER: &str = "<lobby_name>";

pub enum MainMenuMessage {
    SetNickname(AttrValue),
    SetLobbyName(AttrValue),
    SetSignalingServerUrl(AttrValue),
    Join,
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

/// The opening menu where the player puts ils name and the lobby name.
/// He can optionally change the signaling server URL.
#[derive(Debug)]
pub struct MainMenu {
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
impl Component for MainMenu {
    type Message = MainMenuMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let MainMenu {
            nickname,
            lobby_name,
            signaling_server_url,
        } = self;

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
                    {"Dig In !"}
                </button>
            </div>
        }
    }
}
