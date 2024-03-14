use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::api::converse;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            user: true,
            text: new_message.clone(),
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/rust-chatbot.css"/>
        <Title text="DecAI Chatbot"/>
        <ChatArea conversation />
        <TypeArea send />        
    }
}
