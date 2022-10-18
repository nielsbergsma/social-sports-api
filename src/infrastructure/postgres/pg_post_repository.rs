


use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::option::Option;

use crate::common::{RepositoryResult};
use crate::domain::social::aggregates::{Post, PostId};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgPostRepository {
    pool: Pool<Postgres>,
}

impl PgPostRepository {
    pub fn build(pool: Pool<Postgres>) -> PgPostRepository {
        PgPostRepository { pool }
    }
}

#[derive(sqlx::FromRow)]
struct PostRow {
    data: Json<Post>,
}

#[tonic::async_trait]
impl crate::domain::social::repositories::PostRepository for PgPostRepository {
    async fn get(&self, id: &PostId) -> RepositoryResult<Option<Post>> {
        let sql = r#"
              select data
              from posts
              where id = $1
              limit 1"#;

        let row: Option<PostRow> = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(row.map(|columns| columns.data.0))
    }

    async fn set(&self, post: &Post) -> RepositoryResult<()> {
        let sql = r#"
               insert into posts (id, data)
               values ($1, $2)
               on conflict (id) do update set data = $2"#;

        sqlx::query(sql)
            .bind(post.id.to_string())
            .bind(Json(post))
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }

    async fn remove(&self, id: &PostId) -> RepositoryResult<()> {
        let sql = r#"
               delete from posts
               where id = $1"#;

        sqlx::query(sql)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }
}

