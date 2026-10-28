//! Application use cases for the barrier, cashier, and management workflows.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

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

pub struct CalculateCharge {
    pub ticket_code: String,
    pub calculated_at: DateTime<Utc>,
}

pub struct ChargeQuote {
    pub ticket_id: Uuid,
    pub amount: Decimal,
    pub courtesy: bool,
}

pub struct RegisterPayment {
    pub ticket_id: Uuid,
    pub amount_received: Decimal,
    pub paid_at: DateTime<Utc>,
}

pub struct Application;

impl Application {
    pub async fn issue_entry_ticket(&self, request: IssueEntryTicket) {}
    pub async fn validate_exit(&self, request: ValidateExit) {}
    pub async fn calculate_charge(&self, request: CalculateCharge) {}
    pub async fn register_payment(&self, request: RegisterPayment) {}
    pub async fn revenue_for_day(&self, day: NaiveDate) {}
    pub async fn revenue_for_month(&self, year: i32, month: u32) {}
    pub async fn courtesy_count_for_day(&self, day: NaiveDate) {}
    pub async fn courtesy_count_for_month(&self, year: i32, month: u32) {}
}
