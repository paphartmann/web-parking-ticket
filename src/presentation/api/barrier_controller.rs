use crate::application::barrier::{ExitValidation, IssueEntryTicket, ValidateExit};
use crate::domain::{entities::parking_ticket::ParkingTicket, errors::DomainError};

pub struct BarrierController;

pub trait BarrierHttpApi {
    async fn issue_entry_ticket(
        &self,
        request: IssueEntryTicket,
    ) -> Result<ParkingTicket, DomainError>;
    async fn validate_exit(&self, request: ValidateExit) -> Result<ExitValidation, DomainError>;
}
