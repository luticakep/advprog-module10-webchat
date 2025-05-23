use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
        <div class="flex w-screen h-screen">
            <div
                class="flex-none w-64 p-4 overflow-auto"
                style="background-color: #FDFAF6; border-right: 1px solid #E4EFE7;"
            >
            <h2
                class="text-xl mb-4"
                style="color: #99BC85;"
            >
                {"👥 Active Users"}
            </h2>
            {
                self.users.iter().map(|u| html! {
                <div class="flex items-center mb-3">
                    <img
                    class="w-10 h-10 rounded-full mr-3"
                    src={u.avatar.clone()}
                    alt="avatar"
                    />
                    <div>
                    <div class="font-medium">{ &u.name }</div>
                    <div class="text-xs" style="color: #6B6B6B;">{"Online"}</div>
                    </div>
                </div>
                }).collect::<Html>()
            }
            </div>

            <div class="flex-grow flex flex-col">
            <div
                class="flex items-center px-6"
                style="height: 60px; background-color: #FAF1E6; border-bottom: 1px solid #E4EFE7;"
            >
                <h1
                    class="text-2xl font-bold"
                    style="color: #99BC85;"
                >
                {"💬 KayChat"}
                </h1>
                <span class="ml-4 italic" style="color: #4A4A4A;">{"Connect, share & laugh!"}</span>
            </div>

            // Messages
            <div
                class="flex-grow overflow-auto p-6 space-y-4"
                style="background-color: #FDFAF6;"
            >
                {
                if self.messages.is_empty() {
                    html! {
                    <p class="italic" style="color: #6B6B6B; text-align: center; margin-top: 2rem;">
                        {"No messages yet – be the first to say 👋!"}
                    </p>
                    }
                } else {
                    self.messages.iter().map(|m| {
                    let u = self.users.iter().find(|u| u.name == m.from).unwrap();
                    html! {
                        <div class="flex items-start max-w-2/3">
                        <img
                            class="w-8 h-8 rounded-full mr-3"
                            src={u.avatar.clone()}
                            alt="avatar"
                        />
                        <div
                            class="p-3 rounded-lg"
                            style="
                            background-color: #E4EFE7;
                            border: 1px solid #FAF1E6;
                            "
                        >
                            <div class="font-semibold" style="color: #333;">{ &m.from }</div>
                            <div class="mt-1 text-sm" style="color: #4A4A4A;">
                            if m.message.ends_with(".gif") {
                                <img class="rounded" src={m.message.clone()} alt="gif"/>
                            } else {
                                { &m.message }
                            }
                            </div>
                        </div>
                        </div>
                    }
                    }).collect::<Html>()
                }
                }
            </div>

            <div
                class="flex items-center p-4"
                style="background-color: #FAF1E6; border-top: 1px solid #E4EFE7;"
            >
                <input
                ref={self.chat_input.clone()}
                type="text"
                placeholder="Type your message…"
                class="flex-grow py-2 px-4 rounded-full outline-none"
                style="background-color: #E4EFE7; border: 1px solid #FAF1E6; color: #333;"
                />
                <button
                onclick={submit}
                class="ml-3 w-10 h-10 rounded-full flex items-center justify-center"
                style="background-color: #99BC85;"
                >
                <svg viewBox="0 0 24 24" class="w-5 h-5 fill-[#FDFAF6]">
                    <path d="M0 0h24v24H0z" fill="none"/>
                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                </svg>
                </button>
            </div>
            </div>
        </div>
        }
    }
}