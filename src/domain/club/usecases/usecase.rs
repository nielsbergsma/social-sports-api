use std::error::Error;
use std::sync::Arc;
use crate::common::EventPublisher;
use crate::domain::club::aggregates::{Club, ClubId};
use crate::domain::club::commands::{AddStaffMember, New, NewResult, RemoveStaffMember, SetLogo};
use crate::domain::club::events::{ClubAddedV1, ClubLogoSetV1, StaffMemberAddedToClubV1, StaffMemberRemovedFromClubV1};
use crate::domain::club::repositories::ClubRepository;
use crate::domain::club::usecases::DomainError;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct ClubUsecase {
    club_repository: Box<dyn ClubRepository + Send + Sync>,
    event_publisher: Arc<EventPublisher>,
}

impl ClubUsecase {
    pub fn build(
        club_repository: Box<dyn ClubRepository + Send + Sync>,
        event_publisher: Arc<EventPublisher>) -> ClubUsecase {

        ClubUsecase {
            club_repository,
            event_publisher,
        }
    }

    // commands
    pub async fn new(&self, command: New) -> Result<NewResult> {
        let id = ClubId::random();
        let name = command.name;
        let club = Club::new(id.clone(), name);
        self.club_repository.set(&club).await?;

        let event = ClubAddedV1 {
            id: club.id,
            name: club.name
        };
        self.event_publisher.publish(&event).await?;

        Ok(NewResult{
            id
        })
    }

    pub async fn set_logo(&self, command: SetLogo) -> Result<()> {
        let mut club = self.club_repository
            .get(&command.club)
            .await?
            .ok_or(DomainError::UnknownClub)?;

        club.set_logo(&command.logo);
        self.club_repository.set(&club).await?;

        let event = ClubLogoSetV1 { club: command.club, logo: command.logo };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    pub async fn add_staff_member(&self, command: AddStaffMember) -> Result<()> {
        let mut club = self.club_repository
            .get(&command.club)
            .await?
            .ok_or(DomainError::UnknownClub)?;

        club.add_staff_member(&command.person);
        self.club_repository.set(&club).await?;

        let event = StaffMemberAddedToClubV1 { club: command.club, person: command.person };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    pub async fn remove_staff_member(&self, command: RemoveStaffMember) -> Result<()> {
        let mut club = self.club_repository
            .get(&command.club)
            .await?
            .ok_or(DomainError::UnknownClub)?;

        club.remove_staff_member(&command.staff_member);
        self.club_repository.set(&club).await?;

        let event = StaffMemberRemovedFromClubV1 { club: command.club, staff_member: command.staff_member };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    // queries
    pub async fn list_clubs(&self, after: Option<ClubId>) -> Result<Vec<Club>> {
        self.club_repository
            .list(&after).await
            .map_err(|err| err.into())
    }
}