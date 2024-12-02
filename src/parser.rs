use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, Error, ErrorKind, Write},
    path::Path,
};

use clap::Parser;

use crate::errors::AppError;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A simple rust program that counts the number of words on a given list of URLs.",
    long_about = None,
)]
pub struct Args {
    // Name of the file to open
    #[arg(short, long)]
    pub filename: Option<String>,

    // List of URLs to open
    #[arg(short, long, num_args(1..))]
    pub url_list: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ParserError {
    details: String,
}

impl ParserError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for ParserError {}

pub fn interactive_shell() -> Result<Vec<String>, AppError> {
    println!("Enter URLs (type END to finish):");
    let mut urls = Vec::new();
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.eq_ignore_ascii_case("END") {
            break;
        }
        urls.push(input.to_string());
    }
    Ok(urls)
}

pub fn parse_file(filename: &str) -> Result<Vec<String>, AppError> {
    if !Path::new(filename).exists() {
        return Err(AppError::from(Error::new(
            ErrorKind::NotFound,
            format!("FileNotFoundError: {}", filename),
        )));
    }

    let file = File::open(filename)?;
    let urls: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();
    Ok(urls)
}
