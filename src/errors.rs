use std::fmt::Display;

use crate::parser;

#[derive(Debug)]
pub enum AppError {
    ParseError(String),
    IoError(std::io::Error),
    HttpRequestError(reqwest::Error),
    // TODO: Add more error types as needed
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParseError(msg) => write!(f, "Parser error: {}", msg),
            AppError::IoError(err) => write!(f, "IO error: {}", err),
            AppError::HttpRequestError(err) => write!(f, "HTTP request error: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpRequestError(err)
    }
}

impl From<parser::ParserError> for AppError {
    fn from(err: parser::ParserError) -> Self {
        AppError::ParseError(err.to_string())
    }
}
