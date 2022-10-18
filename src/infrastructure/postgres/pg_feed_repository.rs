use sqlx::{Pool, Postgres};
use std::option::Option;
use sqlx::types::Json;

use crate::common::{RepositoryResult};
use crate::domain::social::aggregates::{Feed, FeedListing, FeedFragment, Post, PostId};
use crate::infrastructure::postgres::to_repository_error;

pub struct PgFeedRepository {
    pool: Pool<Postgres>,
}

impl PgFeedRepository {
    pub fn build(pool: Pool<Postgres>) -> PgFeedRepository {
        PgFeedRepository { pool }
    }
}

#[derive(sqlx::FromRow)]
struct FeedRow {
    post: Json<Post>,
    comments: i64,
    reactions_love: i64,
    reactions_funny: i64,
    reactions_celebrate: i64,
    reactions_support: i64,
    reactions_insightful: i64
}

#[tonic::async_trait]
impl crate::domain::social::repositories::FeedRepository for PgFeedRepository {
    async fn list(&self, feed: &Feed, after: &Option<PostId>) -> RepositoryResult<FeedFragment> {
        let rows: Vec<FeedRow> = match feed {
            Feed::Memberships(user) => {
                let sql = r#"
                    with feed as (
                        select posts.id, row_number() over (order by posts.published desc) row
                        from posts
                        join communities on posts.community = communities.id
                        where $1 = any(members)
                        limit 25
                    )
                    select
                       (select data from posts as data where id = feed.id) as post,
                       (select count(1) from comments where reply_to = feed.id) as comments,
                       -- (select count(1) from post_reactions where post = feed.id) as reactions,
                       post_reactions_stats.reactions_love,
                       post_reactions_stats.reactions_funny,
                       post_reactions_stats.reactions_celebrate,
                       post_reactions_stats.reactions_support,
                       post_reactions_stats.reactions_insightful
                    from feed
                    join post_reactions_stats on feed.id = post_reactions_stats.post
                    where ($2 is null or row > (select coalesce(max(row), bigint_max()) from feed where id = $2))"#;

                sqlx::query_as(sql)
                    .bind(user.to_string())
                    .bind(after.as_ref().map(|p| p.to_string()))
                    .fetch_all(&self.pool)
                    .await
                    .map_err(to_repository_error)?
            },

            Feed::Community(community) => {
                let sql = r#"
                    with feed as (
                        select posts.id, posts.data, row_number() over (order by posts.published desc) row
                        from posts
                        where posts.community = $1
                        limit 25
                    )
                    select
                       (select data from posts as data where id = feed.id) as post,
                       (select count(1) from comments where reply_to = feed.id) as comments,
                       -- (select count(1) from post_reactions where post = feed.id) as reactions,
                       post_reactions_stats.reactions_love,
                       post_reactions_stats.reactions_funny,
                       post_reactions_stats.reactions_celebrate,
                       post_reactions_stats.reactions_support,
                       post_reactions_stats.reactions_insightful
                    from feed
                    join post_reactions_stats on feed.id = post_reactions_stats.post
                    where ($2 is null or row > (select coalesce(max(row), bigint_max()) from feed where id = $2))"#;

                sqlx::query_as(sql)
                    .bind(community.to_string())
                    .bind(after.as_ref().map(|p| p.to_string()))
                    .fetch_all(&self.pool)
                    .await
                    .map_err(to_repository_error)?
            }
        };

        Ok(FeedFragment::from_vec(rows
            .into_iter()
            .map(|row| FeedListing {
                post: row.post.0,
                comments: row.comments as u64,
                reactions_love: row.reactions_love as u64,
                reactions_funny: row.reactions_funny as u64,
                reactions_celebrate: row.reactions_celebrate as u64,
                reactions_support: row.reactions_support as u64,
                reactions_insightful: row.reactions_insightful as u64,
            })
            .collect()
        ))
    }
}