use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{entities::parking_ticket::ParkingTicket, errors::DomainError};

pub struct IssueEntryTicket {
    pub vehicle_plate: String,
}

pub struct ValidateExit {
    pub ticket_code: Option<String>,
    pub vehicle_plate: Option<String>,
    pub requested_at: DateTime<Utc>,
}

pub struct ExitValidation {
    pub ticket_id: Uuid,
    pub may_open_barrier: bool,
}

pub trait BarrierUseCases: Send + Sync {
    async fn issue_entry_ticket(
        &self,
        request: IssueEntryTicket,
    ) -> Result<ParkingTicket, DomainError>;
    async fn validate_exit(&self, request: ValidateExit) -> Result<ExitValidation, DomainError>;
}
