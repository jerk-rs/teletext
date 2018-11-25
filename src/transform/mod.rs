mod command;
mod to_arrow;
mod to_square;
mod to_star;
mod to_sw;

pub use self::{
    command::TransformCommand, to_arrow::to_arrow, to_square::to_square, to_star::to_star,
    to_sw::to_sw,
};
use std::{error::Error, fmt};

type Bounds = (usize, usize);

pub type TransformResult<T> = Result<T, TransformError>;

#[derive(Debug)]
pub struct TransformError {
    description: String,
}

impl TransformError {
    fn invalid_length(min: usize, max: usize) -> TransformError {
        TransformError {
            description: format!("Text must contain from {} up to {} characters", min, max),
        }
    }
}

impl Error for TransformError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for TransformError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description())
    }
}

fn collect_chars(s: &str) -> Vec<char> {
    s.chars().flat_map(|c| c.to_uppercase()).collect()
}
