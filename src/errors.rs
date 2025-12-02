use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Parsing error : {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("IO errorr : {0}")]
    IoError(#[from] std::io::Error),
    #[error("RegexError : {0}")]
    RegexError(#[from] regex::Error),
}

//impl std::error::Error for AdventError {}
