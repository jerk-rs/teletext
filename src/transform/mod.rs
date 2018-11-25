mod command;
mod to_arrow;
mod to_square;
mod to_star;
mod to_sw;

pub use self::{
    command::TransformCommand, to_arrow::to_arrow, to_square::to_square, to_star::to_star,
    to_sw::to_sw,
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
