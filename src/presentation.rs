//! Controller facades grouping operations by parking workflow.
//!
//! These types provide a presentation-layer boundary for barrier, cashier,
//! and management actions; the current Rocket routes are declared in
//! `main.rs`.

use chrono::NaiveDate;

use crate::application::{
    CalculateCharge, ChargeQuote, ExitValidation, IssueEntryTicket, RegisterPayment, ValidateExit,
};
use crate::domain::{CourtesyReport, DomainError, ParkingTicket, Payment, RevenueReport};

/// Operations associated with vehicle entry and exit barriers.
pub struct BarrierController;

impl BarrierController {
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
}

/// Operations used by the cashier workflow.
pub struct CashierController;

impl CashierController {
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
}

/// Read-only reporting operations for parking management.
pub struct ManagementController;

impl ManagementController {
    pub async fn daily_revenue(&self, _day: NaiveDate) -> Result<RevenueReport, DomainError> {
        Ok(RevenueReport::default())
    }

    pub async fn monthly_revenue(
        &self,
        _year: i32,
        _month: u32,
    ) -> Result<RevenueReport, DomainError> {
        Ok(RevenueReport::default())
    }

    pub async fn daily_courtesy_count(
        &self,
        _day: NaiveDate,
    ) -> Result<CourtesyReport, DomainError> {
        Ok(CourtesyReport::default())
    }

    pub async fn monthly_courtesy_count(
        &self,
        _year: i32,
        _month: u32,
    ) -> Result<CourtesyReport, DomainError> {
        Ok(CourtesyReport::default())
    }
}
