use crate::common::RepositoryResult;
use crate::domain::media::aggregates::ImageId;

#[tonic::async_trait]
pub trait ImageRepository {
    async fn set(&self, id: ImageId, data: Vec<u8>) -> RepositoryResult<()>; //TODO create Image type
}
