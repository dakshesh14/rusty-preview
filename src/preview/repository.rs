use crate::preview::model::MetaData;
use sqlx::Error;
use sqlx::{PgPool, Result, Row};

pub struct MetaDataRepository {
    pool: PgPool,
}

impl MetaDataRepository {
    pub fn new(pool: PgPool) -> Self {
        MetaDataRepository { pool }
    }

    pub async fn insert_metadata(&self, metadata: &MetaData) -> Result<()> {
        let result = sqlx::query(
            r#"
            INSERT INTO preview (title, description, keywords, image, link)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&metadata.title)
        .bind(&metadata.description)
        .bind(&metadata.keywords)
        .bind(&metadata.image)
        .bind(&metadata.link)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Error::Database(db_err) = &e {
                    if db_err.code().as_deref() == Some("42P01") {
                        // 42P01 is the error code for "relation does not exist"
                        eprintln!(
                            "Warning: Table 'preview' does not exist. Please run migrations."
                        );
                        Ok(())
                    } else {
                        Err(e)
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn get_metadata_by_url(&self, link: &str) -> Result<Option<MetaData>> {
        let result = sqlx::query(
            r#"
            SELECT title, description, keywords, image, link
            FROM preview
            WHERE link = $1
            "#,
        )
        .bind(link)
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(row) => {
                if let Some(row) = row {
                    Ok(Some(MetaData {
                        title: row.get("title"),
                        description: row.get("description"),
                        keywords: row.get("keywords"),
                        image: row.get("image"),
                        link: row.get("link"),
                    }))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                if let Error::Database(db_err) = &e {
                    if db_err.code().as_deref() == Some("42P01") {
                        eprintln!(
                            "Warning: Table 'preview' does not exist. Please run migrations."
                        );
                        Ok(None)
                    } else {
                        Err(e)
                    }
                } else {
                    Err(e)
                }
            }
        }
    }
}
