//! Repository pattern and relational persistence boundary (SeaORM/PostgreSQL).

use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::domain::{DomainError, ParkingTicket, Payment};

pub struct Repositories {
    pub database: DatabaseConnection,
}

impl Repositories {
    pub async fn find_ticket_by_id(&self, _id: Uuid) -> Result<Option<ParkingTicket>, DomainError> {
        Ok(None)
    }

    pub async fn find_ticket_by_code(
        &self,
        _code: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        Ok(None)
    }

    pub async fn find_active_ticket_by_plate(
        &self,
        _plate: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        Ok(None)
    }

    pub async fn save_ticket(&self, _ticket: &ParkingTicket) -> Result<(), DomainError> {
        Ok(())
    }

    pub async fn save_payment(&self, _payment: &Payment) -> Result<(), DomainError> {
        Ok(())
    }

    pub async fn total_received_between(
        &self,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<rust_decimal::Decimal, DomainError> {
        Ok(rust_decimal::Decimal::ZERO)
    }

    pub async fn count_courtesy_between(
        &self,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<u64, DomainError> {
        Ok(0)
    }
}
