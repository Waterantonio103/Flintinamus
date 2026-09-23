use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileReadError {
    #[error("error reading file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("target not found")]
    NotFound
}