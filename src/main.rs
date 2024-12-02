use clap::Parser;
use crawler::Crawler;
use errors::AppError;
use parser::{Args, ParserError};

pub mod crawler;
pub mod errors;
pub mod parser;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let urls = {
        if args.filename.is_some() && args.url_list.is_some() {
            let error = ParserError::new("Cannot provide both a file and a list of URLs.");
            return Err(AppError::from(error));
        } else if let Some(filename) = args.filename {
            parser::parse_file(&filename)?
        } else if let Some(url_list) = args.url_list {
            url_list
        } else {
            parser::interactive_shell()?
        }
    };

    // Spawn tasks for fetching URLs
    let url_tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            tokio::spawn(async move {
                let mut crawler = Crawler::new(&url);
                crawler.run().await?;
                Ok::<Crawler, AppError>(crawler)
            })
        })
        .collect();

    // Wait for all tasks to complete and collect results
    let mut total_word_count = 0;
    for task in url_tasks {
        match task.await {
            Ok(Ok(crawler)) => {
                println!("{}: {} words", crawler.url, crawler.word_count);
                total_word_count += crawler.word_count;
            }
            Ok(Err(e)) => {
                eprintln!("Error fetching URL: {}", e);
            }
            Err(e) => {
                eprintln!("Task failed: {}", e);
            }
        }
    }

    println!("Total word count: {}", total_word_count);
    Ok(())
}
