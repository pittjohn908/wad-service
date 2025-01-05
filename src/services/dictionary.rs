use sqlx::PgPool;
use tonic::{async_trait, Request, Response, Status};

use crate::db_client::DbClient;
use crate::grpc::dictionary_service_server::DictionaryService;
use crate::grpc::{GetRandomWordRequest, GetRandomWordResponse};

pub struct DictionaryGrpcService {
    db_client: DbClient,
}

impl DictionaryGrpcService {
    pub fn from(pool: PgPool) -> Self {
        DictionaryGrpcService {
            db_client: DbClient::from(pool),
        }
    }
}

type ServiceResponse<T> = Result<Response<T>, Status>;

#[async_trait]
impl DictionaryService for DictionaryGrpcService {
    async fn get_random_word(
        &self,
        request: Request<GetRandomWordRequest>,
    ) -> ServiceResponse<GetRandomWordResponse> {
        let metadata = request.metadata();
        let word_details = self.db_client.get_random_word().await;

        match word_details {
            Ok(word_details) => Ok(Response::new(GetRandomWordResponse {
                word: Some(word_details),
            })),
            Err(err) => Err(err.into()),
        }
    }
}
