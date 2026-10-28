//! MVC controllers. Barrier actions are exposed as web service endpoints.

use chrono::NaiveDate;

use crate::application::{CalculateCharge, IssueEntryTicket, RegisterPayment, ValidateExit};

pub struct BarrierController;

impl BarrierController {
    pub async fn issue_entry_ticket(&self, _request: IssueEntryTicket) {}
    pub async fn validate_exit(&self, _request: ValidateExit) {}
}

pub struct CashierController;

impl CashierController {
    pub async fn calculate_charge(&self, _request: CalculateCharge) {}
    pub async fn register_payment(&self, _request: RegisterPayment) {}
}

pub struct ManagementController;

impl ManagementController {
    pub async fn daily_revenue(&self, _day: NaiveDate) {}
    pub async fn monthly_revenue(&self, _year: i32, _month: u32) {}
    pub async fn daily_courtesy_count(&self, _day: NaiveDate) {}
    pub async fn monthly_courtesy_count(&self, _year: i32, _month: u32) {}
}
