use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct MetaData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub image: Option<String>,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl From<&MetaData> for MetaDataResponse {
    fn from(metadata: &MetaData) -> Self {
        Self {
            title: metadata.title.clone(),
            description: metadata.description.clone(),
            keywords: metadata.keywords.clone(),
            image: metadata.image.clone(),
        }
    }
}

impl TryFrom<MetaDataResponse> for MetaData {
    type Error = String;

    fn try_from(_: MetaDataResponse) -> Result<Self, Self::Error> {
        Err("Cannot convert MetaDataResponse to MetaData without a link".to_string())
    }
}

impl MetaDataResponse {
    #[allow(dead_code)]
    pub fn into_metadata(self, link: String) -> MetaData {
        MetaData {
            title: self.title,
            description: self.description,
            keywords: self.keywords,
            image: self.image,
            link,
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

impl MetaData {
    #[allow(dead_code)]
    pub fn to_response(&self) -> MetaDataResponse {
        self.into()
    }
}

impl MetaDataResponse {
    #[allow(dead_code)]
    pub fn with_link(self, link: String) -> MetaData {
        self.into_metadata(link)
    }
}
