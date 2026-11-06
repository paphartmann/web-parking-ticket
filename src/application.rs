//! Application use cases for the barrier, cashier, and management workflows.
//!
//! This layer coordinates domain rules and persistence-facing workflows; the
//! HTTP adapter translates these types to and from JSON.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::{CourtesyReport, DomainError, ParkingTicket, Payment, RevenueReport};

/// Input needed to create a ticket at the entry barrier.
pub struct IssueEntryTicket {
    pub vehicle_plate: String,
}

/// Ticket lookup values and the timestamp used for exit eligibility.
pub struct ValidateExit {
    pub ticket_code: Option<String>,
    pub vehicle_plate: Option<String>,
    pub requested_at: DateTime<Utc>,
}

/// Result of checking whether a ticket may leave through the barrier.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitValidation {
    /// Ticket that was checked.
    pub ticket_id: Uuid,
    /// Whether the barrier may be opened for this ticket.
    pub may_open_barrier: bool,
}

impl Default for ExitValidation {
    fn default() -> Self {
        Self {
            ticket_id: Uuid::nil(),
            may_open_barrier: false,
        }
    }
}

/// Input for calculating a cashier quote at a specific instant.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateCharge {
    pub ticket_code: String,
    pub calculated_at: DateTime<Utc>,
}

/// Amount currently due for a ticket and whether it qualifies for courtesy.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeQuote {
    pub ticket_id: Uuid,
    pub amount: Decimal,
    pub courtesy: bool,
}

impl Default for ChargeQuote {
    fn default() -> Self {
        Self {
            ticket_id: Uuid::nil(),
            amount: Decimal::ZERO,
            courtesy: false,
        }
    }
}

/// Input required to record a payment against a ticket.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayment {
    pub ticket_id: Uuid,
    pub amount_received: Decimal,
    pub paid_at: DateTime<Utc>,
}

pub struct Application;

impl Application {
    pub async fn issue_entry_ticket(
        &self,
        request: IssueEntryTicket,
    ) -> Result<ParkingTicket, DomainError> {
        if request.vehicle_plate.trim().is_empty() {
            return Err(DomainError::InvalidVehiclePlate);
        }
        Ok(ParkingTicket::issue(request.vehicle_plate, Utc::now()))
    }

    pub async fn validate_exit(
        &self,
        _request: ValidateExit,
    ) -> Result<ExitValidation, DomainError> {
        Ok(ExitValidation::default())
    }

    pub async fn calculate_charge(
        &self,
        _request: CalculateCharge,
    ) -> Result<ChargeQuote, DomainError> {
        Ok(ChargeQuote::default())
    }

    pub async fn register_payment(
        &self,
        _request: RegisterPayment,
    ) -> Result<Payment, DomainError> {
        Ok(Payment::default())
    }

    pub async fn revenue_for_day(&self, _day: NaiveDate) -> Result<RevenueReport, DomainError> {
        Ok(RevenueReport::default())
    }

    pub async fn revenue_for_month(
        &self,
        _year: i32,
        _month: u32,
    ) -> Result<RevenueReport, DomainError> {
        Ok(RevenueReport::default())
    }

    pub async fn courtesy_count_for_day(
        &self,
        _day: NaiveDate,
    ) -> Result<CourtesyReport, DomainError> {
        Ok(CourtesyReport::default())
    }

    pub async fn courtesy_count_for_month(
        &self,
        _year: i32,
        _month: u32,
    ) -> Result<CourtesyReport, DomainError> {
        Ok(CourtesyReport::default())
    }
}

#[cfg(test)]
mod tests {
    use super::{Application, IssueEntryTicket};
    use crate::domain::DomainError;

    #[rocket::async_test]
    async fn issue_entry_ticket_rejects_blank_plate() {
        let result = Application
            .issue_entry_ticket(IssueEntryTicket {
                vehicle_plate: " \t\n ".to_owned(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::InvalidVehiclePlate)));
    }

    #[rocket::async_test]
    async fn issue_entry_ticket_returns_normalized_active_ticket() {
        let result = Application
            .issue_entry_ticket(IssueEntryTicket {
                vehicle_plate: "  abc1234 ".to_owned(),
            })
            .await;
        let ticket = match result {
            Ok(ticket) => ticket,
            Err(_) => panic!("a non-empty plate should issue a ticket"),
        };

        assert_eq!(ticket.vehicle_plate, "ABC1234");
        assert!(ticket.paid_at.is_none());
        assert!(ticket.released_at.is_none());
        assert!(matches!(ticket.status, crate::domain::TicketStatus::Active));
    }
}
