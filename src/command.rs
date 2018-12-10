use crate::transform::{TransformError, TransformResult};
use log::error;
use teleborg::objects::{InlineKeyboardMarkup, Message, Update};
use teleborg::{Bot, Command, ParseMode};

#[derive(Debug, Fail)]
enum CommandError {
    #[fail(display = "{}", _0)]
    Transform(#[fail(cause)] TransformError),
    #[fail(display = "You should provide some text")]
    NoText,
}
type CommandArgs<'a> = Option<Vec<&'a str>>;

const DEFAULT_PARSE_MODE: Option<&ParseMode> = Some(&ParseMode::Markdown);
const DEFAULT_WEB_PREVIEW: Option<&bool> = None;
const DEFAULT_NOTIFICATION: Option<&bool> = None;
const DEFAULT_REPLY_MARKUP: Option<&InlineKeyboardMarkup> = None;

fn send_reply(bot: &Bot, chat_id: i64, text: &str, reply_to: i64) {
    let res = bot.send_message(
        &chat_id,
        text,
        DEFAULT_PARSE_MODE,
        DEFAULT_WEB_PREVIEW,
        DEFAULT_NOTIFICATION,
        Some(&reply_to),
        DEFAULT_REPLY_MARKUP,
    );
    if let Err(err) = res {
        error!("Failed to send a message: {:?}", err);
    }
}

pub enum MonospaceReply {
    Enabled,
    Disabled,
}

pub struct TransformCommand<T> {
    transform: T,
    monospace_reply: MonospaceReply,
}

impl<T> TransformCommand<T> {
    pub fn new(transform: T) -> TransformCommand<T> {
        TransformCommand {
            transform,
            monospace_reply: MonospaceReply::Enabled,
        }
    }

    pub fn with_monospace_reply_disabled(mut self) -> TransformCommand<T> {
        self.monospace_reply = MonospaceReply::Disabled;
        self
    }
}

impl<T> TransformCommand<T>
where
    T: Fn(&str) -> TransformResult<String> + Send + Sync + 'static,
{
    fn process(&mut self, bot: &Bot, msg: Message, args: CommandArgs) {
        let chat_id = msg.chat.id;
        let reply_to = msg
            .reply_to_message
            .as_ref()
            .map(|reply| reply.message_id)
            .unwrap_or(msg.message_id);
        let maybe_text = args
            .filter(|x| !x.is_empty())
            .map(|x| x.join(" "))
            .or_else(|| msg.reply_to_message.and_then(|reply| reply.text));
        let result = maybe_text.ok_or(CommandError::NoText).and_then(|text| {
            (self.transform)(text.trim()).map_err(|err| CommandError::Transform(err))
        });
        let reply_text = match result {
            Ok(text) => match self.monospace_reply {
                MonospaceReply::Enabled => format!("```\n{}\n```", text),
                MonospaceReply::Disabled => text,
            },
            Err(err) => err.to_string(),
        };
        send_reply(bot, chat_id, &reply_text, reply_to);
    }
}

impl<T> Command for TransformCommand<T>
where
    T: Fn(&str) -> TransformResult<String> + Send + Sync + 'static,
{
    fn execute(&mut self, bot: &Bot, update: Update, args: CommandArgs) {
        if let Some(msg) = update.message {
            self.process(bot, msg, args);
        }
    }
}
