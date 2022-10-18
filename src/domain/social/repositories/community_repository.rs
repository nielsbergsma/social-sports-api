use crate::common::RepositoryResult;
use crate::domain::social::aggregates::{Community, CommunityContext, CommunityId};

#[tonic::async_trait]
pub trait CommunityRepository {
    async fn list(&self, context: &Option<CommunityContext>, after: &Option<CommunityId>) -> RepositoryResult<Vec<Community>>;
    async fn get(&self, id: &CommunityId) -> RepositoryResult<Option<Community>>;
    async fn set(&self, community: &Community) -> RepositoryResult<()>;
}
