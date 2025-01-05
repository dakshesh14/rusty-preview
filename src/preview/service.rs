use headless_chrome::Browser;
use scraper::{Html as ScraperHTML, Selector};
use thiserror::Error;

use crate::config::constants::Settings;

use super::model::MetaData;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Browser error: {0}")]
    BrowserError(String),
    #[allow(dead_code)]
    #[error("Unknown error")]
    Unknown,
}

/// Fetches the HTML content of a URL using reqwest.
///
/// # Arguments
/// * `url` - The URL to fetch.
///
/// # Returns
/// * `Ok(String)` containing the HTML content if successful.
/// * `Err(FetchError)` if an error occurs.
pub async fn fetch_with_request(url: &str) -> Result<String, FetchError> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

/// Fetches the HTML content of a URL using a headless browser.
///
/// # Arguments
/// * `url` - The URL to fetch.
///
/// # Returns
/// * `Ok(String)` containing the HTML content if successful.
/// * `Err(FetchError)` if an error occurs.
pub async fn fetch_with_headless_browser(url: &str) -> Result<String, FetchError> {
    let browser = Browser::default()
        .map_err(|e| FetchError::BrowserError(format!("Failed to initialize browser: {}", e)))?;

    let tab = browser
        .new_tab()
        .map_err(|e| FetchError::BrowserError(format!("Failed to create new tab: {}", e)))?;

    tab.navigate_to(url)
        .map_err(|e| FetchError::BrowserError(format!("Failed to navigate to {}: {}", url, e)))?;

    tab.wait_for_element("html")
        .map_err(|e| FetchError::BrowserError(format!("Failed to wait for HTML element: {}", e)))?;

    let html = tab
        .get_content()
        .map_err(|e| FetchError::BrowserError(format!("Failed to get page content: {}", e)))?;

    Ok(html)
}

/// Extracts metadata from the given HTML content.
///
/// # Arguments
/// * `html` - The HTML content to extract metadata from.
///
/// # Returns
/// * `MetaData` containing the extracted metadata.
pub fn extract_metadata(html: &str) -> MetaData {
    let document = ScraperHTML::parse_document(html);

    let extract_meta_content = |property: &str| {
        let selector = Selector::parse(&format!(
            r#"meta[property="{}"], meta[name="{}"]"#,
            property, property
        ))
        .unwrap();
        document
            .select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(String::from)
    };

    let title = extract_meta_content("og:title").or_else(|| {
        let title_selector = Selector::parse("title").unwrap();
        document
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
    });

    let description =
        extract_meta_content("og:description").or_else(|| extract_meta_content("description"));

    let keywords = extract_meta_content("keywords");

    let image = extract_meta_content("og:image");

    MetaData {
        title,
        description,
        keywords,
        image,
    }
}

/// Fetches metadata from a URL, using a headless browser if necessary.
///
/// # Arguments
/// * `url` - The URL to fetch metadata from.
///
/// # Returns
/// * `Ok(MetaData)` containing the extracted metadata if successful.
/// * `Err(FetchError)` if an error occurs.
pub async fn fetch_metadata(url: &str) -> Result<MetaData, FetchError> {
    let settings = Settings::from_env();

    if settings.use_headless_browser_only {
        let html = fetch_with_headless_browser(url).await?;
        let metadata = extract_metadata(&html);
        Ok(metadata)
    } else {
        match fetch_with_request(url).await {
            Ok(html) => {
                let metadata = extract_metadata(&html);
                if metadata.title.is_some() && metadata.description.is_some() {
                    return Ok(metadata);
                }
            }
            Err(e) => eprintln!("Failed to fetch with request: {}", e),
        }

        let html = fetch_with_headless_browser(url).await?;
        let metadata = extract_metadata(&html);
        Ok(metadata)
    }
}
