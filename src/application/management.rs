use crate::domain::errors::DomainError;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

pub struct RevenueReport {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub total_received: Decimal,
}

pub struct CourtesyReport {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub ticket_count: u64,
}

pub trait ManagementUseCases: Send + Sync {
    async fn revenue_for_day(&self, day: NaiveDate) -> Result<RevenueReport, DomainError>;
    async fn revenue_for_month(&self, year: i32, month: u32) -> Result<RevenueReport, DomainError>;
    async fn courtesy_count_for_day(&self, day: NaiveDate) -> Result<CourtesyReport, DomainError>;
    async fn courtesy_count_for_month(
        &self,
        year: i32,
        month: u32,
    ) -> Result<CourtesyReport, DomainError>;
}
