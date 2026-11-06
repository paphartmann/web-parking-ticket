//! Core parking concepts, state, and business operations.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

/// A vehicle's visit, from ticket issuance through payment and release.
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

/// Lifecycle state exposed for a parking ticket.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TicketStatus {
    Active,
    Courtesy,
    Paid,
    Released,
}

impl ParkingTicket {
    /// Create an active ticket, normalizing the plate and generating IDs.
    pub fn issue(vehicle_plate: String, at: DateTime<Utc>) -> Self {
        let plate = vehicle_plate.trim().to_ascii_uppercase();
        let code = format!("{}-{}", at.format("%y%m%d%H%M%S"), Uuid::new_v4().simple());
        Self {
            id: Uuid::new_v4(),
            code,
            vehicle_plate: plate,
            entered_at: at,
            paid_at: None,
            released_at: None,
            status: TicketStatus::Active,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::{ParkingTicket, TicketStatus};
    use chrono::{TimeZone, Utc};

    #[test]
    fn issue_normalizes_plate_and_sets_initial_ticket_state() {
        let entered_at = Utc.with_ymd_and_hms(2025, 3, 8, 12, 30, 45).unwrap();

        let ticket = ParkingTicket::issue("  abc-1234  ".to_owned(), entered_at);

        assert_eq!(ticket.vehicle_plate, "ABC-1234");
        assert_eq!(ticket.entered_at, entered_at);
        assert!(ticket.paid_at.is_none());
        assert!(ticket.released_at.is_none());
        assert!(matches!(ticket.status, TicketStatus::Active));
        assert!(!ticket.id.is_nil());
        assert!(ticket.code.starts_with("250308123045-"));
        assert_eq!(ticket.code.len(), "250308123045-".len() + 32);
    }

    #[test]
    fn issue_generates_unique_ids_and_codes() {
        let at = Utc.with_ymd_and_hms(2025, 3, 8, 12, 30, 45).unwrap();

        let first = ParkingTicket::issue("ABC1234".to_owned(), at);
        let second = ParkingTicket::issue("ABC1234".to_owned(), at);

        assert_ne!(first.id, second.id);
        assert_ne!(first.code, second.code);
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

/// A recorded amount received for a parking ticket.
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

/// Revenue total over the reported time interval.
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

/// Count of tickets granted courtesy over the reported time interval.
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

/// Business-rule failures returned by domain and application operations.
pub enum DomainError {
    TicketNotFound,
    TicketAlreadyReleased,
    TicketNotEligibleForExit,
    InvalidVehiclePlate,
    PaymentAmountMismatch,
}
