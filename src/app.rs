use futures::prelude::await;
use futures::prelude::*;
use futures_retry::{RetryPolicy, StreamRetryExt};
use huify::huify_sentence;
use std::{error::Error, fmt, io::Error as IoError};
use telegram_bot::prelude::*;
use telegram_bot::{
    Api, Error as TelegramError, Message, MessageEntity, MessageEntityKind, MessageKind,
    MessageOrChannelPost, ParseMode, UpdateKind,
};
use tokio_core::reactor::Core;
use transform::*;

pub type AppResult<T> = Result<T, AppError>;

pub struct App {
    api: Api,
    core: Core,
}

impl App {
    pub fn new(token: &str) -> AppResult<App> {
        let core = Core::new()?;
        let api = Api::configure(token).build(core.handle())?;
        Ok(App {
            api: api,
            core: core,
        })
    }

    pub fn run(mut self) -> AppResult<()> {
        self.core.run(handle_updates(self.api))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AppError {
    description: String,
}

impl AppError {
    fn unknown_command(cmd: &str) -> AppError {
        AppError {
            description: format!("Unknown command: {}", cmd),
        }
    }

    fn nothing_to_huify() -> AppError {
        AppError {
            description: String::from("Nothing to huify"),
        }
    }
}

impl From<IoError> for AppError {
    fn from(err: IoError) -> AppError {
        AppError {
            description: err.to_string(),
        }
    }
}

impl From<TelegramError> for AppError {
    fn from(err: TelegramError) -> AppError {
        AppError {
            description: err.to_string(),
        }
    }
}

impl From<TransformError> for AppError {
    fn from(err: TransformError) -> AppError {
        AppError {
            description: err.to_string(),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description)
    }
}

#[async]
fn handle_updates(api: Api) -> Result<(), TelegramError> {
    #[async]
    for update in api.stream().retry(handle_update_error) {
        if let UpdateKind::Message(message) = update.kind {
            match handle_command(&message) {
                Ok(None) => { /* noop */ }
                Ok(Some(reply)) => {
                    let reply = reply.to_string();
                    let mut out = match message.reply_to_message {
                        Some(box MessageOrChannelPost::Message(ref reply_to)) => {
                            reply_to.text_reply(reply)
                        }
                        Some(box MessageOrChannelPost::ChannelPost(ref reply_to)) => {
                            reply_to.text_reply(reply)
                        }
                        _ => message.text_reply(reply),
                    };
                    if let Err(err) = await!(api.send(out.parse_mode(ParseMode::Markdown))) {
                        println!("Failed to send message: {}", err);
                    }
                }
                Err(err) => {
                    if let Err(err) = await!(api.send(message.text_reply(err.to_string()))) {
                        println!("Failed to send message: {}", err);
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_update_error(error: TelegramError) -> RetryPolicy<TelegramError> {
    println!("An error has occurred while getting update: {:?}", error);
    RetryPolicy::Repeat
}

fn handle_command(message: &Message) -> Result<Option<CommandResult>, AppError> {
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
                return Ok(Some(match cmd {
                    "/huify" => {
                        if let Some(box MessageOrChannelPost::Message(Message {
                            kind: MessageKind::Text { ref data, .. },
                            ..
                        })) = message.reply_to_message
                        {
                            CommandResult::new(huify_sentence(data), false)
                        } else {
                            if text.len() > 0 {
                                CommandResult::new(huify_sentence(text), false)
                            } else {
                                return Err(AppError::nothing_to_huify());
                            }
                        }
                    }
                    "/arrow" => CommandResult::new(text.to_arrow()?, true),
                    "/square" => CommandResult::new(text.to_square()?, true),
                    "/star" => CommandResult::new(text.to_star()?, true),
                    "/sw" => CommandResult::new(text.to_sw()?, true),
                    _ => return Err(AppError::unknown_command(cmd)),
                }));
            }
        }
    }
    Ok(None)
}

struct CommandResult {
    data: String,
    is_monospace: bool,
}

impl CommandResult {
    fn new(data: String, is_monospace: bool) -> CommandResult {
        CommandResult { data, is_monospace }
    }
}

impl fmt::Display for CommandResult {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.is_monospace {
            write!(out, "```{}```", self.data)
        } else {
            write!(out, "{}", self.data)
        }
    }
}
