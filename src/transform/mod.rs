mod arrow;
mod command;
mod huify;
mod square;
mod star;
mod sw;

pub use self::{
    arrow::to_arrow, command::TransformCommand, huify::huify, square::to_square, star::to_star,
    sw::to_sw,
};

/// Transformation error.
#[derive(Fail, Debug)]
#[fail(display = "Transformation error")]
pub enum Error {
    #[fail(display = "Text must contain from {} up to {} characters", min, max)]
    InvalidLength { min: usize, max: usize },
    #[fail(display = "You should provide some text")]
    NoText,
}

type Bounds = (usize, usize);

type Result<T> = ::std::result::Result<T, Error>;

fn collect_chars(s: &str) -> Vec<char> {
    s.chars().flat_map(char::to_uppercase).collect()
}
