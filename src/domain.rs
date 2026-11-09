//! Core parking concepts, state, and business operations.

use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, dec};
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
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TicketStatus {
    Active,
    Courtesy,
    Paid,
    Released,
}

impl ParkingTicket {
    pub fn calculate_charge(&self, at: DateTime<Utc>) -> Decimal {
        let minutes = (at - self.entered_at).num_minutes();
        if minutes <= 15 {
            Decimal::ZERO
        } else if minutes <= 60 {
            dec!(15)
        } else {
            let additional_hours = (minutes - 1) / 60;
            dec!(15) + (dec!(4.5) * Decimal::from(additional_hours))
        }
    }

    pub fn can_exit(&self, at: DateTime<Utc>) -> bool {
        matches!(self.status, TicketStatus::Paid | TicketStatus::Courtesy)
            && self.released_at.is_none()
            && self.entered_at < at
            && if let Some(did_pay_at) = self.paid_at {
                did_pay_at < at
            } else {
                false
            }
    }

    pub fn is_within_courtesy_period(&self, at: DateTime<Utc>) -> bool {
        (at - self.entered_at).num_minutes() <= 15
    }

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

    #[test]
    fn charge_applies_courtesy_and_hour_portions() {
        use chrono::Duration;
        use rust_decimal::dec;
        let entered_at = Utc.with_ymd_and_hms(2025, 3, 8, 12, 0, 0).unwrap();
        let ticket = ParkingTicket::issue("ABC1234".to_owned(), entered_at);
        assert_eq!(
            ticket.calculate_charge(entered_at + Duration::minutes(15)),
            dec!(0)
        );
        assert_eq!(
            ticket.calculate_charge(entered_at + Duration::minutes(16)),
            dec!(15)
        );
        assert_eq!(
            ticket.calculate_charge(entered_at + Duration::minutes(60)),
            dec!(15)
        );
        assert_eq!(
            ticket.calculate_charge(entered_at + Duration::minutes(61)),
            dec!(19.5)
        );
        assert_eq!(
            ticket.calculate_charge(entered_at + Duration::minutes(120)),
            dec!(19.5)
        );
        assert_eq!(
            ticket.calculate_charge(entered_at + Duration::minutes(121)),
            dec!(24)
        );
    }

    #[test]
    fn exit_requires_payment_after_entry_and_ticket_not_released() {
        use chrono::Duration;
        let entered_at = Utc.with_ymd_and_hms(2025, 3, 8, 12, 0, 0).unwrap();
        let mut ticket = ParkingTicket::issue("ABC1234".to_owned(), entered_at);
        let paid_at = entered_at + Duration::minutes(20);
        assert!(!ticket.can_exit(paid_at));
        ticket.paid_at = Some(paid_at);
        ticket.status = TicketStatus::Paid;
        assert!(ticket.can_exit(paid_at + Duration::seconds(1)));
        ticket.released_at = Some(paid_at + Duration::seconds(2));
        assert!(!ticket.can_exit(paid_at + Duration::seconds(3)));
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

/// Revenue total over the reported time interval.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueReport {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub total_received: Decimal,
}

/// Count of tickets granted courtesy over the reported time interval.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourtesyReport {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub ticket_count: u64,
}

/// Business-rule failures returned by domain and application operations.
pub enum DomainError {
    TicketNotFound,
    InvalidStoredData,
    TicketAlreadyReleased,
    TicketNotEligibleForExit,
    InvalidVehiclePlate,
    PaymentAmountMismatch,
}
