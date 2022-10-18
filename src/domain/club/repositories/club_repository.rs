use crate::common::RepositoryResult;
use crate::domain::club::aggregates::{Club, ClubId};

#[tonic::async_trait]
pub trait ClubRepository {
    async fn list(&self, after: &Option<ClubId>) -> RepositoryResult<Vec<Club>>;
    async fn get(&self, id: &ClubId) -> RepositoryResult<Option<Club>>;
    async fn set(&self, club: &Club) -> RepositoryResult<()>;
}
