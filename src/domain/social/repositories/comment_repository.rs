use crate::common::RepositoryResult;
use crate::domain::social::aggregates::{Comment, CommentId, PostId};

#[tonic::async_trait]
pub trait CommentRepository {
    async fn list(&self, post: &PostId, after: &Option<CommentId>) -> RepositoryResult<Vec<Comment>>;
    async fn get(&self, id: &CommentId) -> RepositoryResult<Option<Comment>>;
    async fn set(&self, comment: &Comment) -> RepositoryResult<()>;
    async fn remove(&self, id: &CommentId) -> RepositoryResult<()>;
}
