use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MetaData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub image: Option<String>,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataResponse {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub image: Option<String>,
}

impl From<MetaData> for MetaDataResponse {
    fn from(metadata: MetaData) -> Self {
        Self {
            title: metadata.title,
            description: metadata.description,
            keywords: metadata.keywords,
            image: metadata.image,
        }
    }
}

impl Default for MetaDataResponse {
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
