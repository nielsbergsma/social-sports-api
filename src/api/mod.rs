pub mod api;

use crate::domain::club::usecases::ClubUsecase;
use crate::domain::social::usecases::usecase::SocialUsecase;
use crate::domain::team::usecases::TeamUsecase;

tonic::include_proto!("api");

pub struct ApiService {
    club_usecase: ClubUsecase,
    team_usecase: TeamUsecase,
    social_usecase: SocialUsecase,
}

impl ApiService {
    pub fn build(club_usecase: ClubUsecase, team_usecase: TeamUsecase, social_usecase: SocialUsecase) -> ApiService {
        ApiService {
            club_usecase,
            team_usecase,
            social_usecase,
        }
    }
}