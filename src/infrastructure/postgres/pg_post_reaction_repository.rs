use sqlx::types::Json;
use sqlx::{Pool, Postgres, Row};

use crate::common::{RepositoryResult};
use crate::domain::social::aggregates::{PostReaction};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgPostReactionRepository {
    pool: Pool<Postgres>,
}

impl PgPostReactionRepository {
    pub fn build(pool: Pool<Postgres>) -> PgPostReactionRepository {
        PgPostReactionRepository { pool }
    }
}

#[tonic::async_trait]
impl crate::domain::social::repositories::PostReactionRepository for PgPostReactionRepository {
    async fn set(&self, reaction: &PostReaction) -> RepositoryResult<bool> {
        let sql = r#"
            with current as (
                select count(0) as existing
                from post_reactions
                where post = $1 and author = $2 and emotion = $3
            )
            , update as (
                 insert into post_reactions (post, author, emotion, data)
                 values ($1, $2, $3, $4)
                 on conflict (post, author) where emotion <> $3 do update set emotion = $3, data = $4
                 returning 1 as modified
            )
            -- returns number of exact duplicates (at most 1)
            select existing from current, update"#;

        let (emotion, author, post) = reaction.values();

        let result: i64 = sqlx::query(sql)
            .bind(post.to_string())
            .bind(author.to_string())
            .bind(emotion)
            .bind(Json(reaction))
            .fetch_one(&self.pool)
            .await
            .map_err(to_repository_error)
            .map(|row| row.get(0))?;

        Ok(result == 0)
    }

    async fn unset(&self, reaction: &PostReaction) -> RepositoryResult<bool> {
        let sql = r#"
               delete from post_reactions
               where post = $1
                 and author = $2
                 and emotion = $3"#;

        let (emotion, author, post) = reaction.values();

        let result = sqlx::query(sql)
            .bind(post.to_string())
            .bind(author.to_string())
            .bind(emotion)
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(result.rows_affected() > 0)
    }
}