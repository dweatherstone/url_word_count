use crate::errors::AppError;

#[derive(Debug)]
pub struct Crawler {
    pub url: String,
    pub word_count: usize,
}

impl Crawler {
    pub fn new(url: &str) -> Self {
        Crawler {
            url: url.to_string(),
            word_count: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        self.get_word_count().await?;
        Ok(())
    }

    async fn get_word_count(&mut self) -> Result<(), AppError> {
        let response = reqwest::get(&self.url).await?;
        let body = response.text().await?;
        let word_count = body.split_whitespace().count();
        self.word_count = word_count;
        Ok(())
    }
}
