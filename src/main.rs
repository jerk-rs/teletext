#![feature(proc_macro, generators, try_from)]

extern crate dotenv;
extern crate futures_await as futures;
extern crate telegram_bot;
extern crate teletext;
extern crate tokio_core;

use dotenv::dotenv;
use futures::prelude::*;
use std::convert::TryInto;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Message, MessageEntity, MessageEntityKind, MessageKind, ParseMode,
                   UpdateKind};
use teletext::{Arrow, Square, Star, Sw};
use tokio_core::reactor::Core;

fn main() {
    dotenv().ok();
    let mut core = Core::new().expect("Failed to create core");
    let token = env::var("TELETEXT_TOKEN").expect("Can not to get token");
    let api = Api::configure(token).build(core.handle()).expect("Failed to build API");
    core.run(handle_updates(api)).expect("Run failed");
}

#[async]
fn handle_updates(api: Api) -> Result<(), telegram_bot::Error> {
    #[async]
    for update in api.stream() {
        if let UpdateKind::Message(message) = update.kind {
            if let Ok(reply) = handle_command(&message) {
                if let Err(err) = await!(
                    api.send(
                        message
                            .text_reply(format!("```\n{}\n```", reply))
                            .parse_mode(ParseMode::Markdown),
                    )
                ) {
                    println!("Failed to send message: {}", err);
                }
            }
        }
    }
    Ok(())
}

fn handle_command(message: &Message) -> Result<String, ()> {
    if let MessageKind::Text {
        ref data,
        ref entities,
    } = message.kind
    {
        if let Some(MessageEntity {
            kind: MessageEntityKind::BotCommand,
            offset: 0,
            length,
        }) = entities.first()
        {
            let (cmd, text) = data.split_at(*length as usize);
            let cmd = cmd.split('@').next();
            if let Some(cmd) = cmd {
                let text = text.trim();
                return Ok(match cmd {
                    "/arrow" => text.try_into()
                        .unwrap_or(Arrow {
                            buf: "goforkurself".into(),
                        })
                        .to_string(),
                    "/square" => text.try_into()
                        .unwrap_or(Square {
                            buf: "goforkurself".into(),
                        })
                        .to_string(),
                    "/star" => text.try_into()
                        .unwrap_or(Star {
                            buf: "goforkurself".into(),
                        })
                        .to_string(),
                    "/sw" => text.try_into()
                        .unwrap_or(Sw {
                            buf: "goforkurself".into(),
                        })
                        .to_string(),
                    _ => return Err(()),
                });
            }
        }
    }
    Err(())
}
