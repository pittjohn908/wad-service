use tonic::{async_trait, Request, Response, Status};

use crate::grpc::dictionary_server::Dictionary;
use crate::grpc::{GetRandomWordReply, GetRandomWordRequest};

pub struct DictionaryService;
type ServiceResponse<T> = Result<Response<T>, Status>;

#[async_trait]
impl Dictionary for DictionaryService {
    async fn get_random_word(
        &self,
        request: Request<GetRandomWordRequest>,
    ) -> ServiceResponse<GetRandomWordReply> {
        Ok(tonic::Response::new(GetRandomWordReply {
            word: None
        }))
    }
}
