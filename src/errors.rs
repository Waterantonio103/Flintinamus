use std::io;

use thiserror::Error;

//custom error types describing potential errors encountered at runtime
#[derive(Debug, Error)]
pub enum FileReadError {
    #[error("error reading file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("target not found")]
    NotFound,
    #[error("Slice too short")]
    ShortSlice,
}