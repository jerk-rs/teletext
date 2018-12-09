use crate::app::send_reply;
use crate::transform::{Error, Result};
use std::borrow::Cow;
use teleborg::objects::{Message, Update};
use teleborg::{Bot, Command};

pub struct TransformCommand<T>(pub T);

fn extract_data(
    Message {
        message_id,
        reply_to_message,
        ..
    }: Message,
    args: Option<Vec<&str>>,
) -> (i64, Option<String>) {
    let reply_to = reply_to_message
        .as_ref()
        .map(|reply| reply.message_id)
        .unwrap_or(message_id);
    let text = args
        .filter(|x| !x.is_empty())
        .map(|x| x.join(" "))
        .or_else(|| reply_to_message.and_then(|reply| reply.text));
    (reply_to, text)
}

impl<T> TransformCommand<T>
where
    T: FnMut(&str) -> Result<String> + Send + Sync + 'static,
{
    fn process(&mut self, bot: &Bot, msg: Message, args: Option<Vec<&str>>) {
        let chat_id = msg.chat.id;
        let (reply_to, maybe_text) = extract_data(msg, args);
        let result: CommandResult = maybe_text
            .ok_or(Error::NoText)
            .and_then(|text| (self.0)(text.trim()))
            .into();
        send_reply(bot, chat_id, &result.as_str(), reply_to);
    }
}

impl<T> Command for TransformCommand<T>
where
    T: FnMut(&str) -> Result<String> + Send + Sync + 'static,
{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        if let Some(msg) = update.message {
            self.process(bot, msg, args);
        }
    }
}

struct CommandResult {
    data: String,
    is_monospace: bool,
}

impl CommandResult {
    fn as_str(&self) -> Cow<'_, str> {
        if self.is_monospace {
            Cow::Owned(format!("```\n{}\n```", self.data))
        } else {
            Cow::Borrowed(&self.data)
        }
    }
}

impl From<Result<String>> for CommandResult {
    fn from(res: Result<String>) -> Self {
        match res {
            Ok(data) => CommandResult {
                data,
                is_monospace: true,
            },
            Err(e) => CommandResult {
                data: e.to_string(),
                is_monospace: false,
            },
        }
    }
}
