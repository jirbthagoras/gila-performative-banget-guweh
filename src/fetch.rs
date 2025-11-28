use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleBooksResponse {
    pub items: Option<Vec<GoogleBookItem>>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleBookItem {
    #[serde(rename = "volumeInfo")]
    pub volume_info: GoogleBookVolumeInfo,
}

#[derive(Debug, Deserialize)]
pub struct GoogleBookVolumeInfo {
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
}

pub fn fetch_book(query: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let url = format!("https://www.googleapis.com/books/v1/volumes?q={}", query);

    let resp: GoogleBooksResponse = reqwest::blocking::get(&url)?.json()?;

    if let Some(items) = resp.items {
        if let Some(first) = items.first() {
            let title = first
                .volume_info
                .title
                .clone()
                .unwrap_or("Unknown Title".into());
            let authors = first
                .volume_info
                .authors
                .clone()
                .unwrap_or(vec!["Unknown".into()]);

            return Ok((title, authors.join(", ")));
        }
    }

    Err("No books found".into())
}
