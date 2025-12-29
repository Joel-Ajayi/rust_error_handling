use std::error::Error;

use thiserror;
use base64;

// #[error] macro generates Display, 
// #[from] macro handles From implementations 
// source() for std::error::Error

#[derive(thiserror::Error)]
pub enum AppError {
    #[error("Failed to read the file.")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to decode the Input")]
    DecodeError(#[from] base64::DecodeError),

    #[error("Failed to parse decoded bytes")]
    StringError(#[from]std::string::FromUtf8Error)
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}")?;
        self.source();
        if let Some(e) = self.source() {
            writeln!(f, "The error was caused by {e:?}")?;
        }
        Ok(())
    }
}