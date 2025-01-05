use crate::preview::model::MetaData;
use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool, Row};
use std::sync::Arc;

const INSERT_METADATA_QUERY: &str = r#"
    INSERT INTO preview (title, description, keywords, image, link)
    VALUES ($1, $2, $3, $4, $5)
"#;

const GET_METADATA_QUERY: &str = r#"
    SELECT title, description, keywords, image, link
    FROM preview
    WHERE link = $1
"#;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    #[error("Table does not exist")]
    TableNotFound,
    #[allow(dead_code)]
    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[async_trait]
pub trait MetadataRepository {
    async fn insert_metadata(&self, metadata: &MetaData) -> Result<()>;
    async fn get_metadata_by_url(&self, link: &str) -> Result<Option<MetaData>>;
}

pub struct Repository {
    pool: Arc<PgPool>,
}

#[derive(Default)]
pub struct RepositoryBuilder {
    pool: Option<Arc<PgPool>>,
}

impl RepositoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pool(mut self, pool: Arc<PgPool>) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn build(self) -> Result<Repository> {
        let pool = self
            .pool
            .ok_or_else(|| RepositoryError::Other("Database pool is required".to_string()))?;

        Ok(Repository { pool })
    }
}

impl Repository {
    pub fn builder() -> RepositoryBuilder {
        RepositoryBuilder::new()
    }

    fn handle_error(&self, error: SqlxError) -> RepositoryError {
        if let SqlxError::Database(db_err) = &error {
            if db_err.code().as_deref() == Some("42P01") {
                eprintln!("Warning: Table 'preview' does not exist. Please run migrations.");
                RepositoryError::TableNotFound
            } else {
                RepositoryError::Database(error)
            }
        } else {
            RepositoryError::Database(error)
        }
    }
}

#[async_trait]
impl MetadataRepository for Repository {
    async fn insert_metadata(&self, metadata: &MetaData) -> Result<()> {
        sqlx::query(INSERT_METADATA_QUERY)
            .bind(&metadata.title)
            .bind(&metadata.description)
            .bind(&metadata.keywords)
            .bind(&metadata.image)
            .bind(&metadata.link)
            .execute(&*self.pool)
            .await
            .map_err(|e| self.handle_error(e))?;

        Ok(())
    }

    async fn get_metadata_by_url(&self, link: &str) -> Result<Option<MetaData>> {
        sqlx::query(GET_METADATA_QUERY)
            .bind(link)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| self.handle_error(e))
            .map(|row| {
                row.map(|row| MetaData {
                    title: row.get("title"),
                    description: row.get("description"),
                    keywords: row.get("keywords"),
                    image: row.get("image"),
                    link: row.get("link"),
                })
            })
    }
}
