use std::error::Error;
use std::sync::Arc;
use crate::common::EventPublisher;
use crate::domain::team::aggregates::{Team, TeamId};
use crate::domain::team::commands::{AddStaffMember, New, NewResult, RemoveStaffMember};
use crate::domain::team::events::{StaffMemberAddedToTeamV1, StaffMemberRemovedFromTeamV1, TeamAddedV1};
use crate::domain::team::repositories::TeamRepository;
use crate::domain::team::usecases::DomainError;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct TeamUsecase {
    team_repository: Box<dyn TeamRepository + Send + Sync>,
    event_publisher: Arc<EventPublisher>,
}

impl TeamUsecase {
    pub fn build(
        team_repository: Box<dyn TeamRepository + Send + Sync>,
        event_publisher: Arc<EventPublisher>) -> TeamUsecase {
        TeamUsecase {
            team_repository,
            event_publisher,
        }
    }

    // commands
    pub async fn new(&self, command: New) -> Result<NewResult> {
        let id = TeamId::random();
        let name = command.name;
        let club = command.club;
        let team = Team::new(id.clone(), name, club);
        self.team_repository.set(&team).await?;

        let event = TeamAddedV1 {
            id: team.id,
            name: team.name,
            club: team.club
        };
        self.event_publisher.publish(&event).await?;

        Ok(NewResult{
            id,
        })
    }

    pub async fn add_staff_member(&self, command: AddStaffMember) -> Result<()> {
        let mut team = self.team_repository
            .get(&command.team)
            .await?
            .ok_or(DomainError::UnknownTeam)?;

        team.add_staff_member(&command.person);
        self.team_repository.set(&team).await?;

        let event = StaffMemberAddedToTeamV1 { team: command.team, person: command.person };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    pub async fn remove_staff_member(&self, command: RemoveStaffMember) -> Result<()> {
        let mut team = self.team_repository
            .get(&command.team)
            .await?
            .ok_or(DomainError::UnknownTeam)?;

        team.remove_staff_member(&command.staff_member);
        self.team_repository.set(&team).await?;

        let event = StaffMemberRemovedFromTeamV1 { team: command.team, staff_member: command.staff_member };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    // queries
    pub async fn list_teams(&self, after: Option<TeamId>) -> Result<Vec<Team>> {
        self.team_repository
            .list(&after).await
            .map_err(|err| err.into())
    }
}