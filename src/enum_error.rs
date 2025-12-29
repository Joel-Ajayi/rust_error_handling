use std::error::Error;


pub enum AppError {
    ReadError(std::io::Error),
    DecodeError(base64::DecodeError),
    StringError(std::string::FromUtf8Error)
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>{
        use AppError::*;
        match self {
            ReadError(e) => Some(e),
            DecodeError(e)=> Some(e),
            StringError(e)=> Some(e),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AppError::*;
        let msg = match self {
            ReadError(_) =>"Failed to read file.",
            DecodeError(_)=> "Failed to decode the Input",
            StringError(_)=> "Failed to parse decoded bytes",
        };
        write!(f, "{msg}")
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}")?;
        
        if let Some(e) = self.source() { 
            writeln!(f, "\tError Caused by: {e:?}")?;
        }
        Ok(())
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::ReadError(value)
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(value: base64::DecodeError) -> Self {
        AppError::DecodeError(value)
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        AppError::StringError(value)
    }
}
