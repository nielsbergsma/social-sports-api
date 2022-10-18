use crate::common::RepositoryResult;
use crate::domain::social::aggregates::{Post, PostId};

#[tonic::async_trait]
pub trait PostRepository {
    async fn get(&self, id: &PostId) -> RepositoryResult<Option<Post>>;
    async fn set(&self, post: &Post) -> RepositoryResult<()>;
    async fn remove(&self, id: &PostId) -> RepositoryResult<()>;
}
