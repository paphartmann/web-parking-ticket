//! Domain Model: core parking concepts and their business operations.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct ParkingTicket {
    pub id: Uuid,
    pub code: String,
    pub vehicle_plate: String,
    pub entered_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub status: TicketStatus,
}

pub enum TicketStatus {
    Active,
    Courtesy,
    Paid,
    Released,
}

impl ParkingTicket {
    pub fn is_within_courtesy_period(&self, at: DateTime<Utc>) {}
    pub fn calculate_charge(&self, at: DateTime<Utc>) {}
    pub fn mark_as_courtesy(&mut self, at: DateTime<Utc>) {}
    pub fn mark_as_paid(&mut self, at: DateTime<Utc>) {}
    pub fn mark_as_released(&mut self, at: DateTime<Utc>) {}
    pub fn can_exit(&self, at: DateTime<Utc>) -> bool {}
}

pub struct Payment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub amount: Decimal,
    pub paid_at: DateTime<Utc>,
}

pub struct RevenueReport {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub total_received: Decimal,
}

pub struct CourtesyReport {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub ticket_count: u64,
}

pub enum DomainError {
    TicketNotFound,
    TicketAlreadyReleased,
    TicketNotEligibleForExit,
    InvalidVehiclePlate,
    PaymentAmountMismatch,
}
