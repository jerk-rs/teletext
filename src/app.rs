use huify::huify_sentence;
use std::fmt;
use teleborg::objects::{Chat, Message, Update};
use teleborg::{Bot, Dispatcher, ParseMode, Updater};
use transform::*;

pub fn run<S: Into<String>>(token: S) {
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("arrow", handle_arrow, true);
    dispatcher.add_command_handler("huify", handle_huify, true);
    dispatcher.add_command_handler("square", handle_square, true);
    dispatcher.add_command_handler("star", handle_star, true);
    dispatcher.add_command_handler("sw", handle_sw, true);
    Updater::start(Some(token.into()), None, None, None, dispatcher);
}

macro_rules! handle_figure {
    ($name:ident, $transform:ident) => {
        fn $name(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
            if let Some(Message {
                message_id,
                chat: Chat { id: chat_id, .. },
                reply_to_message,
                ..
            }) = update.message
            {
                let result = match args {
                    Some(args) => match args.join(" ").trim().$transform() {
                        Ok(result) => CommandResult::new(result, true),
                        Err(err) => CommandResult::new(err.to_string(), false),
                    },
                    None => CommandResult::new("You should provide some text", false),
                };
                let reply_to_id = match reply_to_message {
                    Some(r) => r.message_id,
                    None => message_id,
                };
                if let Err(err) = bot.send_message(
                    &chat_id,
                    &result.to_string(),
                    Some(&ParseMode::Markdown),
                    None,
                    None,
                    Some(&reply_to_id),
                    None,
                ) {
                    println!("Failed to send a message: {:?}", err);
                }
            }
        }
    };
}

handle_figure!(handle_arrow, to_arrow);
handle_figure!(handle_square, to_square);
handle_figure!(handle_star, to_star);
handle_figure!(handle_sw, to_sw);

fn handle_huify(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    let args: Option<String> = args
        .and_then(|x| if x.len() != 0 { Some(x) } else { None })
        .map(|x| x.join(" "));
    if let Some(Message {
        message_id,
        chat: Chat { id: chat_id, .. },
        reply_to_message,
        ..
    }) = update.message
    {
        let (message_id, input) = if let Some(box Message {
            message_id,
            text: Some(text),
            ..
        }) = reply_to_message
        {
            let input = match args {
                Some(args) => args,
                None => text,
            };
            (message_id, input)
        } else {
            let input = match args {
                Some(args) => args,
                None => String::from("Nothing to huify"),
            };
            (message_id, input)
        };
        let output = huify_sentence(&input);
        if let Err(err) = bot.send_message(
            &chat_id,
            &output,
            Some(&ParseMode::Markdown),
            None,
            None,
            Some(&message_id),
            None,
        ) {
            println!("Failed to send a message: {:?}", err);
        }
    }
}

struct CommandResult {
    data: String,
    is_monospace: bool,
}

impl CommandResult {
    fn new<S: Into<String>>(data: S, is_monospace: bool) -> CommandResult {
        CommandResult {
            data: data.into(),
            is_monospace,
        }
    }
}

impl fmt::Display for CommandResult {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.is_monospace {
            write!(out, "```\n{}\n```", self.data)
        } else {
            write!(out, "{}", self.data)
        }
    }
}
