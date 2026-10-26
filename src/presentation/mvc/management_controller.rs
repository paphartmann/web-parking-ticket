use crate::application::management::{CourtesyReport, RevenueReport};
use crate::domain::errors::DomainError;
use chrono::NaiveDate;

pub struct ManagementController;

pub trait ManagementMvcController {
    async fn daily_revenue(&self, day: NaiveDate) -> Result<RevenueReport, DomainError>;
    async fn monthly_revenue(&self, year: i32, month: u32) -> Result<RevenueReport, DomainError>;
    async fn daily_courtesy_count(&self, day: NaiveDate) -> Result<CourtesyReport, DomainError>;
    async fn monthly_courtesy_count(
        &self,
        year: i32,
        month: u32,
    ) -> Result<CourtesyReport, DomainError>;
}
