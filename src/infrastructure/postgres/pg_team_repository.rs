use itertools::Itertools;
use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::option::Option;

use crate::common::{RepositoryResult};
use crate::domain::team::aggregates::{Team, TeamId};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgTeamRepository {
    pool: Pool<Postgres>,
}

impl PgTeamRepository {
    pub fn build(pool: Pool<Postgres>) -> PgTeamRepository {
        PgTeamRepository { pool }
    }
}

#[derive(sqlx::FromRow)]
struct TeamRow {
    data: Json<Team>,
}

#[tonic::async_trait]
impl crate::domain::team::repositories::TeamRepository for PgTeamRepository {
    async fn list(&self, after: &Option<TeamId>) -> RepositoryResult<Vec<Team>> {
        let sql = r#"
            with teams as (
                select id, data, row_number() over (order by name) row
                from teams
            )

            select data
            from teams
            where ($1 is null or row > (select coalesce(max(row), bigint_max()) from Teams where id = $1))
            limit 25"#;

        let rows: Vec<TeamRow> = sqlx::query_as(sql)
            .bind(after.as_ref().map(|p| p.to_string()))
            .fetch_all(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(rows.into_iter().map(|row| row.data.0).collect_vec())
    }

    async fn get(&self, id: &TeamId) -> RepositoryResult<Option<Team>> {
        let sql = r#"
              select data
              from teams
              where id = $1
              limit 1"#;

        let row: Option<TeamRow> = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(row.map(|columns| columns.data.0))
    }

    async fn set(&self, team: &Team) -> RepositoryResult<()> {
        let sql = r#"
               insert into teams (id, data)
               values ($1, $2)
               on conflict (id) do update set data = $2"#;

        sqlx::query(sql)
            .bind(team.id.to_string())
            .bind(Json(team))
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }
}
