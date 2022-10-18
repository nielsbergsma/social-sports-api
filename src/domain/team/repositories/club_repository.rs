use crate::common::RepositoryResult;
use crate::domain::club::aggregates::{ClubId};

#[tonic::async_trait]
pub trait ClubRepository {
    async fn exist(&self, id: &ClubId) -> RepositoryResult<bool>;
}
