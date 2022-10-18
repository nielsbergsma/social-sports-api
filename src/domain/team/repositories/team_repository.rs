use crate::common::RepositoryResult;
use crate::domain::team::aggregates::{Team, TeamId};

#[tonic::async_trait]
pub trait TeamRepository {
    async fn list(&self, after: &Option<TeamId>) -> RepositoryResult<Vec<Team>>;
    async fn get(&self, id: &TeamId) -> RepositoryResult<Option<Team>>;
    async fn set(&self, team: &Team) -> RepositoryResult<()>;
}
