//! Domain Model: core parking concepts and their business operations.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParkingTicket {
    #[serde(rename = "ticketId")]
    pub id: Uuid,
    #[serde(rename = "ticketCode")]
    pub code: String,
    pub vehicle_plate: String,
    pub entered_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub status: TicketStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TicketStatus {
    Active,
    Courtesy,
    Paid,
    Released,
}

impl ParkingTicket {
    pub fn is_within_courtesy_period(&self, _at: DateTime<Utc>) -> bool {
        false
    }
    pub fn calculate_charge(&self, _at: DateTime<Utc>) -> Decimal {
        Decimal::ZERO
    }
    pub fn mark_as_courtesy(&mut self, _at: DateTime<Utc>) {}
    pub fn mark_as_paid(&mut self, _at: DateTime<Utc>) {}
    pub fn mark_as_released(&mut self, _at: DateTime<Utc>) {}
    pub fn can_exit(&self, _at: DateTime<Utc>) -> bool {
        false
    }
}

impl Default for ParkingTicket {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            code: String::new(),
            vehicle_plate: String::new(),
            entered_at: Utc::now(),
            paid_at: None,
            released_at: None,
            status: TicketStatus::Active,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    #[serde(rename = "paymentId")]
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub amount: Decimal,
    pub paid_at: DateTime<Utc>,
}

impl Default for Payment {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            ticket_id: Uuid::nil(),
            amount: Decimal::ZERO,
            paid_at: Utc::now(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueReport {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub total_received: Decimal,
}

impl Default for RevenueReport {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            start: now,
            end: now,
            total_received: Decimal::ZERO,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourtesyReport {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub ticket_count: u64,
}

impl Default for CourtesyReport {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            start: now,
            end: now,
            ticket_count: 0,
        }
    }
}

pub enum DomainError {
    TicketNotFound,
    TicketAlreadyReleased,
    TicketNotEligibleForExit,
    InvalidVehiclePlate,
    PaymentAmountMismatch,
}
