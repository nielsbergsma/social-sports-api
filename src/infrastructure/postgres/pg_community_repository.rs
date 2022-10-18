use itertools::Itertools;


use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::option::Option;

use crate::common::{RepositoryResult};
use crate::domain::social::aggregates::{Community, CommunityContext, CommunityId};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgCommunityRepository {
    pool: Pool<Postgres>,
}

impl PgCommunityRepository {
    pub fn build(pool: Pool<Postgres>) -> PgCommunityRepository {
        PgCommunityRepository { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CommunityRow {
    data: Json<Community>,
}

#[tonic::async_trait]
impl crate::domain::social::repositories::CommunityRepository for PgCommunityRepository {
    async fn list(&self, context: &Option<CommunityContext>, after: &Option<CommunityId>) -> RepositoryResult<Vec<Community>> {
        let sql = r#"
            with communities as (
                select id, data, context_club, context_team, row_number() over (order by name) row
                from communities
            )

            select data
            from communities
            where ($1 is null or row > (select coalesce(max(row), bigint_max()) from communities where id = $1))
              and ($2 is null or context_club = $2)
              and ($3 is null or context_team = $3)
            limit 25"#;

        let (club, team) = match context {
            Some(CommunityContext::Club(id)) => (Some(id.to_string()), None),
            Some(CommunityContext::Team(id)) => (None, Some(id.to_string())),
            _ => (None, None)
        };

        let rows: Vec<CommunityRow> = sqlx::query_as(sql)
            .bind(after.as_ref().map(|p| p.to_string()))
            .bind(club)
            .bind(team)
            .fetch_all(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(rows.into_iter().map(|row| row.data.0).collect_vec())
    }

    async fn get(&self, id: &CommunityId) -> RepositoryResult<Option<Community>> {
        let sql = r#"
              select data
              from communities
              where id = $1
              limit 1"#;

        let row: Option<CommunityRow> = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(row.map(|columns| columns.data.0))
    }

    async fn set(&self, community: &Community) -> RepositoryResult<()> {
        let sql = r#"
               insert into communities (id, data, context_club, context_team)
               values ($1, $2, $3, $4)
               on conflict (id) do update set data = $2, context_club = $3, context_team = $4"#;

        let (context_club,context_team) = match &community.context {
            CommunityContext::Club(id) => (Some(id.to_string()), None),
            CommunityContext::Team(id) => (None, Some(id.to_string()))
        };

        sqlx::query(sql)
            .bind(community.id.to_string())
            .bind(Json(community))
            .bind(context_club)
            .bind(context_team)
            .execute(&self.pool)
            .await
            .map_err(to_repository_error)?;

        Ok(())
    }
}
