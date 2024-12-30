use sqlx::PgPool;
use tonic::{async_trait, Request, Response, Status};

use crate::db_client::DbClient;
use crate::grpc::dictionary_server::Dictionary;
use crate::grpc::{GetRandomWordReply, GetRandomWordRequest};

pub struct DictionaryService {
    db_client: DbClient
}

impl DictionaryService {
    pub fn from(pool: PgPool) -> Self {
        DictionaryService {
            db_client: DbClient::from(pool)
        }
    }
}

type ServiceResponse<T> = Result<Response<T>, Status>;

#[async_trait]
impl Dictionary for DictionaryService {
    async fn get_random_word(
        &self,
        _request: Request<GetRandomWordRequest>,
    ) -> ServiceResponse<GetRandomWordReply> {
        let word_details = self.db_client.get_random_word().await;

        match word_details {
            Ok(word_details) => Ok(Response::new(GetRandomWordReply {
                word: Some(word_details)
            })),
            Err(err) => Err(err.into()),
        }  
    }
}
