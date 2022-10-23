use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::option::Option;

use crate::common::{RepositoryResult};
use crate::domain::social::aggregates::{Comment, CommentId, PostId};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgCommentRepository {
    pool: Pool<Postgres>,
}

impl PgCommentRepository {
    pub fn build(pool: Pool<Postgres>) -> PgCommentRepository {
        PgCommentRepository { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CommentRow {
    data: Json<Comment>,
}

#[tonic::async_trait]
impl crate::domain::social::repositories::CommentRepository for PgCommentRepository {
    async fn list(&self, reply_to: &PostId, after: &Option<CommentId>) -> RepositoryResult<Vec<Comment>> {
        let sql = r#"
            with comments as (
                select id, data, row_number() over (order by published desc) row
                from comments
                where reply_to = $1
            )

            select data
            from comments
            where ($2 is null or row > (select coalesce(max(row), bigint_max()) from comments where id = $2))
            limit 25"#;

        let rows: Vec<CommentRow> = sqlx::query_as(sql)
            .bind(reply_to.to_string())
            .bind(after.as_ref().map(|p| p.to_string()))
            .fetch_all(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(rows.into_iter().map(|row| row.data.0).collect())
    }

    async fn get(&self, id: &CommentId) -> RepositoryResult<Option<Comment>> {
        let sql = r#"
              select data
              from comments
              where id = $1
              limit 1"#;

        let row: Option<CommentRow> = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(row.map(|columns| columns.data.0))
    }

    async fn set(&self, comment: &Comment) -> RepositoryResult<()> {
        let sql = r#"
               insert into comments (id, data)
               values ($1, $2)
               on conflict (id) do update set data = $2"#;

        sqlx::query(sql)
            .bind(comment.id.to_string())
            .bind(Json(comment))
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }

    async fn remove(&self, id: &CommentId) -> RepositoryResult<()> {
        let sql = r#"delete from comments where id = $1"#;

        sqlx::query(sql)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }
}

