use crate::common::RepositoryResult;
use crate::domain::social::aggregates::{Feed, FeedFragment, PostId};

#[tonic::async_trait]
pub trait FeedRepository {
    async fn list(&self, feed: &Feed, after: &Option<PostId>) -> RepositoryResult<FeedFragment>;
}
