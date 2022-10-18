pub mod pg_club_repository;
pub mod pg_team_repository;
pub mod pg_community_repository;
pub mod pg_post_repository;
pub mod pg_comment_repository;
pub mod pg_post_reaction_repository;
pub mod pg_event_repository;
pub mod pg_feed_repository;

pub use pg_club_repository::PgClubRepository;
pub use pg_team_repository::PgTeamRepository;
pub use pg_community_repository::PgCommunityRepository;
pub use pg_post_repository::PgPostRepository;
pub use pg_comment_repository::PgCommentRepository;
pub use pg_post_reaction_repository::PgPostReactionRepository;
pub use pg_event_repository::PgEventRepository;
pub use pg_feed_repository::PgFeedRepository;

// helpers
use crate::common::RepositoryError;

fn to_repository_error(_error: sqlx::Error) -> RepositoryError {
    println!("postgres error {:?}", _error);
    RepositoryError::StorageError //TODO include message
}
