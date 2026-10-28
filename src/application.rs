//! Application use cases for the barrier, cashier, and management workflows.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::{CourtesyReport, DomainError, ParkingTicket, Payment, RevenueReport};

pub struct IssueEntryTicket {
    pub vehicle_plate: String,
}

pub struct ValidateExit {
    pub ticket_code: Option<String>,
    pub vehicle_plate: Option<String>,
    pub requested_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitValidation {
    pub ticket_id: Uuid,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateCharge {
    pub ticket_code: String,
    pub calculated_at: DateTime<Utc>,
}

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
        _request: IssueEntryTicket,
    ) -> Result<ParkingTicket, DomainError> {
        Ok(ParkingTicket::default())
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
