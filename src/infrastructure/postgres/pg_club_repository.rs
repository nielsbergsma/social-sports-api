use itertools::Itertools;


use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::option::Option;

use crate::common::{RepositoryResult};
use crate::domain::club::aggregates::{Club, ClubId};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgClubRepository {
    pool: Pool<Postgres>,
}

impl PgClubRepository {
    pub fn build(pool: Pool<Postgres>) -> PgClubRepository {
        PgClubRepository { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ClubRow {
    data: Json<Club>,
}

#[tonic::async_trait]
impl crate::domain::team::repositories::ClubRepository for PgClubRepository {
    async fn exist(&self, id: &ClubId) -> RepositoryResult<bool> {
        let sql = r#"
              select data
              from clubs
              where id = $1
              limit 1"#;

        let row: Option<ClubRow> = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(row.is_some())
    }
}

#[tonic::async_trait]
impl crate::domain::club::repositories::ClubRepository for PgClubRepository {
    async fn list(&self, after: &Option<ClubId>) -> RepositoryResult<Vec<Club>> {
        let sql = r#"
            with clubs as (
                select id, data, row_number() over (order by name) row
                from clubs
            )

            select data
            from clubs
            where ($1 is null or row > (select coalesce(max(row), bigint_max()) from clubs where id = $1))
            limit 25"#;

        let rows: Vec<ClubRow> = sqlx::query_as(sql)
            .bind(after.as_ref().map(|p| p.to_string()))
            .fetch_all(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(rows.into_iter().map(|row| row.data.0).collect_vec())
    }

    async fn get(&self, id: &ClubId) -> RepositoryResult<Option<Club>> {
        let sql = r#"
              select data
              from clubs
              where id = $1
              limit 1"#;

        let row: Option<ClubRow> = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(row.map(|columns| columns.data.0))
    }

    async fn set(&self, club: &Club) -> RepositoryResult<()> {
        let sql = r#"
               insert into clubs (id, data)
               values ($1, $2)
               on conflict (id) do update set data = $2"#;

        sqlx::query(sql)
            .bind(club.id.to_string())
            .bind(Json(club))
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }
}
