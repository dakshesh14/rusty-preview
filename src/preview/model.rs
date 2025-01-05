use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MetaData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub image: Option<String>,
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            keywords: None,
            image: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PreviewParams {
    pub url: String,
}