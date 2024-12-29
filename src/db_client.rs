use sqlx::postgres::PgPool;
use thiserror::Error;

use crate::services::dictionary_models::{DbDefinition, DbWord};

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error)
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Clone, Debug)]
pub(crate) struct DbClient {
    pool: PgPool
}


impl DbClient {
    pub async fn get_random_word(&self) -> DbResult<DbWord> {
        let word = sqlx::query_as::<_, DbWord>(
            r#"
                SELECT * FROM words
                ORDER BY RANDOM() LIMIT 1;
            "#
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(word) = word {
            let definitions: Vec<DbDefinition> = sqlx::query_as(
                r#"
                    SELECT * FROM definitions
                    WHERE word_id = $1
                "#
            )
            .bind(word.id)
            .fetch_all(&self.pool)
            .await?;

            return Ok(word);
        }

        Ok(DbWord::default())
    }
}


