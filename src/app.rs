use crate::command::TransformCommand;
use crate::transform::{to_arrow, to_huified, to_square, to_star, to_sw};
use teleborg::{Dispatcher, Updater};

const WITH_ARGS: bool = true;

pub fn run<S: Into<String>>(token: S) {
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("arrow", TransformCommand::new(to_arrow), WITH_ARGS);
    dispatcher.add_command_handler(
        "huify",
        TransformCommand::new(to_huified).with_monospace_reply_disabled(),
        WITH_ARGS,
    );
    dispatcher.add_command_handler("square", TransformCommand::new(to_square), WITH_ARGS);
    dispatcher.add_command_handler("star", TransformCommand::new(to_star), WITH_ARGS);
    dispatcher.add_command_handler("sw", TransformCommand::new(to_sw), WITH_ARGS);
    Updater::start(Some(token.into()), None, None, None, dispatcher);
}
