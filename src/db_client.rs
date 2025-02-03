use sqlx::postgres::PgPool;
use thiserror::Error;
use tonic::Status;

use crate::{
    grpc::WordDetails,
    services::db_models::{DbDefinition, DbWord},
};

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl Into<Status> for DbError {
    fn into(self) -> Status {
        Status::from_error(Box::new(self))
    }
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Clone, Debug)]
pub(crate) struct DbClient {
    pool: PgPool,
}

impl DbClient {
    pub fn from(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_random_word(&self) -> DbResult<WordDetails> {
        let word = sqlx::query_as::<_, DbWord>(
            r#"
                SELECT * FROM words
                ORDER BY RANDOM() LIMIT 1;
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let definitions: Vec<DbDefinition> = sqlx::query_as::<_, DbDefinition>(
            r#"
                    SELECT * FROM definitions
                    WHERE word_id = $1
                "#,
        )
        .bind(word.id)
        .fetch_all(&self.pool)
        .await?;

        return Ok(WordDetails::from(word, definitions));
    }

    pub async fn get_user_favorites(&self) -> DbResult<()> {
        Ok(())
    }
}
