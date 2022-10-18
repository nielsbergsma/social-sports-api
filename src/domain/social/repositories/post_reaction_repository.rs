use crate::common::RepositoryResult;
use crate::domain::social::aggregates::PostReaction;

#[tonic::async_trait]
pub trait PostReactionRepository {
    async fn set(&self, reaction: &PostReaction) -> RepositoryResult<bool>;
    async fn unset(&self, reaction: &PostReaction) -> RepositoryResult<bool>;
}
