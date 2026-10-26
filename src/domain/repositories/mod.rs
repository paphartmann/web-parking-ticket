use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{
    entities::{parking_ticket::ParkingTicket, payment::Payment},
    errors::DomainError,
};

pub trait ParkingTicketRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ParkingTicket>, DomainError>;
    async fn find_by_code(&self, code: &str) -> Result<Option<ParkingTicket>, DomainError>;
    async fn find_active_by_plate(&self, plate: &str)
    -> Result<Option<ParkingTicket>, DomainError>;
    async fn save(&self, ticket: &ParkingTicket) -> Result<(), DomainError>;
    async fn count_courtesy_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<u64, DomainError>;
}

pub trait PaymentRepository: Send + Sync {
    async fn save(&self, payment: &Payment) -> Result<(), DomainError>;
    async fn total_received_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<rust_decimal::Decimal, DomainError>;
}
