use crate::transform::{self, TransformCommand};
use teleborg::objects::InlineKeyboardMarkup;
use teleborg::{Bot, Dispatcher, ParseMode, Updater};

pub fn run<S: Into<String>>(token: S) {
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("arrow", TransformCommand(transform::to_arrow), true);
    dispatcher.add_command_handler("huify", TransformCommand(transform::huify), true);
    dispatcher.add_command_handler("square", TransformCommand(transform::to_square), true);
    dispatcher.add_command_handler("star", TransformCommand(transform::to_star), true);
    dispatcher.add_command_handler("sw", TransformCommand(transform::to_sw), true);
    Updater::start(Some(token.into()), None, None, None, dispatcher);
}

pub(crate) fn send_reply(bot: &Bot, chat_id: i64, text: &str, reply_to: i64) {
    const PARSE_MODE: Option<&ParseMode> = Some(&ParseMode::Markdown);
    const DISABLE_WEB_PREVIEW: Option<&bool> = None;
    const DISABLE_NOTIFICATION: Option<&bool> = None;
    const REPLY_MARKUP: Option<&InlineKeyboardMarkup> = None;
    let res = bot.send_message(
        &chat_id,
        text,
        PARSE_MODE,
        DISABLE_WEB_PREVIEW,
        DISABLE_NOTIFICATION,
        Some(&reply_to),
        REPLY_MARKUP,
    );
    if let Err(err) = res {
        println!("Failed to send a message: {:?}", err);
    }
}
