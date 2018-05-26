extern crate dotenv;
extern crate futures;
extern crate telegram_bot;
extern crate teletext;
extern crate tokio_core;

use dotenv::dotenv;
use futures::Stream;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, MessageKind, ParseMode, UpdateKind};
use teletext::Transformer;
use tokio_core::reactor::Core;

fn main() {
    dotenv().ok();
    let mut core = Core::new().unwrap();
    let token = env::var("TELETEXT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    let trans = Transformer::new();
    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if let Some(reply) = trans.transform(data) {
                    api.spawn(
                        message
                            .text_reply(format!("```\n{}\n```", reply))
                            .parse_mode(ParseMode::Markdown),
                    );
                }
            }
        }
        Ok(())
    });
    core.run(future).unwrap();
}
