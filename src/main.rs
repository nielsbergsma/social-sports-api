pub mod common;
pub mod domain;
pub mod infrastructure;
pub mod api;
pub mod config;

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use tonic::{transport::Server};

use api::api_v1_server::{ApiV1Server};
use api::ApiService;
use crate::common::EventPublisher;
use crate::config::Configuration;

use crate::domain::club::usecases::ClubUsecase;
use crate::domain::social::usecases::usecase::SocialUsecase;
use crate::domain::team::usecases::TeamUsecase;
use crate::infrastructure::postgres::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = Configuration::dev()?;

    let pool = PgPoolOptions::new()
        .max_connections(configuration.postgres_pool_size)
        .connect(&configuration.postgres_url)
        .await?;

    // repositories
    let club_repository = Box::new(PgClubRepository::build(pool.clone()));
    let team_repository = Box::new(PgTeamRepository::build(pool.clone()));
    let event_repository = Box::new(PgEventRepository::build(pool.clone()));
    let community_repository = Box::new(PgCommunityRepository::build(pool.clone()));
    let post_repository = Box::new(PgPostRepository::build(pool.clone()));
    let post_reaction_repository = Box::new(PgPostReactionRepository::build(pool.clone()));
    let comment_repository = Box::new(PgCommentRepository::build(pool.clone()));
    let feed_repository = Box::new(PgFeedRepository::build(pool.clone()));

    // clients
    let event_publisher = Arc::new(EventPublisher::build(event_repository.clone()));

    // usecases
    let club_usecase = ClubUsecase::build(club_repository, event_publisher.clone());
    let team_usecase = TeamUsecase::build(team_repository, event_publisher.clone());
    let social_usecase = SocialUsecase::build(community_repository, post_repository, post_reaction_repository, comment_repository, feed_repository,event_publisher.clone());

    // api
    let service = ApiService::build(
        club_usecase,
        team_usecase,
        social_usecase
    );

    Server::builder()
        .add_service(ApiV1Server::new(service))
        .serve(configuration.api_address)
        .await
        .map_err(|err| err.into())
}
