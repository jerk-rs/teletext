use std::fmt;
use teleborg::objects::{Chat, Message, Update};
use teleborg::{Bot, Command, ParseMode};

pub struct TransformCommand<T>(pub T);

impl<T, E> Command for TransformCommand<T>
where
    T: FnMut(&str) -> Result<String, E> + Send + Sync + 'static,
    E: ToString,
{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        if let Some(Message {
            message_id,
            chat: Chat { id: chat_id, .. },
            reply_to_message,
            ..
        }) = update.message
        {
            let result = match args {
                Some(args) => match (self.0)(args.join(" ").trim()) {
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
