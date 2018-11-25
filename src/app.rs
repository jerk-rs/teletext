use huify::huify_sentence;
use teleborg::objects::{Chat, InlineKeyboardMarkup, Message, Update};
use teleborg::{Bot, Dispatcher, ParseMode, Updater};
use transform::{self, TransformCommand};

pub fn run<S: Into<String>>(token: S) {
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("arrow", TransformCommand(transform::to_arrow), true);
    dispatcher.add_command_handler("huify", handle_huify, true);
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
