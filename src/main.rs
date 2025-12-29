use std::{error::Error, path::PathBuf};
use anyhow::Context;
use base64::{self, engine, Engine};

mod enum_error;
mod thiserror_error;

use crate::enum_error::{AppError as EnumAppError};
use crate::thiserror_error::{AppError as ThisErrorAppError};

fn main() {
    // get path of the project directory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/test.txt");
    let res = decode_thiserror(path);
    if let Err(e) = &res {
        println!("Output is: {e} \n\n");
    }

    println!("{res:?}");
}

fn decode(path:PathBuf) -> Result<(), EnumAppError> {
    let input = std::fs::read_to_string("Output").map_err(EnumAppError::ReadError)?;
    for line in input.lines() {
        let bytes = engine::general_purpose::STANDARD.decode(line)?;
        println!("{}", String::from_utf8(bytes)?);
    }
    Ok(())
}

fn decode_thiserror(path:PathBuf) -> Result<(), ThisErrorAppError> {
    let input = std::fs::read_to_string("Output")?;
    for line in input.lines() {
        let bytes = engine::general_purpose::STANDARD.decode(line)?;
        println!("{}", String::from_utf8(bytes)?);
    }
    Ok(())
}

fn decode_anyhow(path:PathBuf) -> Result<(), anyhow::Error> {
    let input = std::fs::read_to_string(path).context("Failed to read the file.")?;
    for line in input.lines() {
        let bytes = engine::general_purpose::STANDARD.decode(line).context(
    "Failed to decode the Input"
        )?;
        println!("{}", String::from_utf8(bytes).context(
            "Failed to parse decoded bytes"
        )?);
    }
    Ok(())
}
