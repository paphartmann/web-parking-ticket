//! Application use cases for the barrier, cashier, and management workflows.
//!
//! This layer coordinates domain rules and persistence-facing workflows; the
//! HTTP adapter translates these types to and from JSON.

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    domain::{CourtesyReport, DomainError, ParkingTicket, Payment, RevenueReport, TicketStatus},
    persistence::Repositories,
};

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
    pub async fn calculate_charge(
        &self,
        request: CalculateCharge,
        repositories: &Repositories,
    ) -> Result<ChargeQuote, DomainError> {
        let ticket = repositories
            .find_ticket_by_code(&request.ticket_code)
            .await?
            .ok_or(DomainError::TicketNotFound)?;
        let courtesy = ticket.is_within_courtesy_period(request.calculated_at);
        Ok(ChargeQuote {
            ticket_id: ticket.id,
            amount: if courtesy {
                Decimal::ZERO
            } else {
                ticket.calculate_charge(request.calculated_at)
            },
            courtesy,
        })
    }

    pub async fn courtesy_count_for_day(
        &self,
        year: i32,
        month: u32,
        day: u32,
        repositories: &Repositories,
    ) -> Result<CourtesyReport, DomainError> {
        let (start, end) = day_bounds_for_date(year, month, day)?;
        let count = repositories
            .count_courtesy_between(start, end)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(CourtesyReport {
            start,
            end,
            ticket_count: count as u64,
        })
    }

    pub async fn courtesy_count_for_month(
        &self,
        year: i32,
        month: u32,
        repositories: &Repositories,
    ) -> Result<CourtesyReport, DomainError> {
        let (start, end) = month_bounds(year, month)?;
        let count = repositories
            .count_courtesy_between(start, end)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(CourtesyReport {
            start,
            end,
            ticket_count: count as u64,
        })
    }

    pub async fn courtesy_count_for_year(
        &self,
        year: i32,
        repositories: &Repositories,
    ) -> Result<CourtesyReport, DomainError> {
        let (start, end) = year_bounds(year)?;
        let ticket_count = repositories
            .count_courtesy_between(start, end)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(CourtesyReport {
            start,
            end,
            ticket_count: ticket_count as u64,
        })
    }

    pub async fn issue_entry_ticket(
        &self,
        request: IssueEntryTicket,
    ) -> Result<ParkingTicket, DomainError> {
        if request.vehicle_plate.trim().is_empty() {
            return Err(DomainError::InvalidVehiclePlate);
        }
        Ok(ParkingTicket::issue(request.vehicle_plate, Utc::now()))
    }

    pub async fn register_payment(
        &self,
        request: RegisterPayment,
        repositories: &Repositories,
    ) -> Result<Payment, DomainError> {
        let ticket = repositories
            .find_ticket_by_id(request.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;
        if ticket.released_at.is_some() {
            return Err(DomainError::TicketAlreadyReleased);
        }
        if ticket.status == TicketStatus::Paid || ticket.paid_at.is_some() {
            return Err(DomainError::TicketNotEligibleForExit);
        }

        let expected = if ticket.is_within_courtesy_period(request.paid_at) {
            Decimal::ZERO
        } else {
            ticket.calculate_charge(request.paid_at)
        };
        if request.amount_received != expected {
            return Err(DomainError::PaymentAmountMismatch);
        }

        let payment = Payment {
            id: Uuid::new_v4(),
            ticket_id: ticket.id,
            amount: request.amount_received,
            paid_at: request.paid_at,
        };
        repositories
            .save_payment(&payment)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(payment)
    }

    pub async fn revenue_for_day(
        &self,
        year: i32,
        month: u32,
        day: u32,
        repositories: &Repositories,
    ) -> Result<RevenueReport, DomainError> {
        let (start, end) = day_bounds_for_date(year, month, day)?;
        let total_received = repositories
            .total_received_between(start, end)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(RevenueReport {
            start,
            end,
            total_received,
        })
    }

    pub async fn revenue_for_month(
        &self,
        year: i32,
        month: u32,
        repositories: &Repositories,
    ) -> Result<RevenueReport, DomainError> {
        let (start, end) = month_bounds(year, month)?;
        let total_received = repositories
            .total_received_between(start, end)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(RevenueReport {
            start,
            end,
            total_received,
        })
    }

    pub async fn revenue_for_year(
        &self,
        year: i32,
        repositories: &Repositories,
    ) -> Result<RevenueReport, DomainError> {
        let (start, end) = year_bounds(year)?;
        let total_received = repositories
            .total_received_between(start, end)
            .await
            .map_err(|_| DomainError::InvalidStoredData)?;
        Ok(RevenueReport {
            start,
            end,
            total_received,
        })
    }

    pub async fn validate_exit(
        &self,
        request: ValidateExit,
        repositories: &Repositories,
    ) -> Result<ExitValidation, DomainError> {
        let ticket = if let Some(code) = request.ticket_code.as_deref() {
            repositories.find_ticket_by_code(code).await?
        } else if let Some(plate) = request.vehicle_plate.as_deref() {
            repositories
                .find_active_ticket_by_plate(&plate.trim().to_ascii_uppercase())
                .await?
        } else {
            None
        }
        .ok_or(DomainError::TicketNotFound)?;

        Ok(ExitValidation {
            ticket_id: ticket.id,
            may_open_barrier: ticket.can_exit(request.requested_at),
        })
    }
}

fn day_bounds(day: NaiveDate) -> Result<(DateTime<Utc>, DateTime<Utc>), DomainError> {
    let start = Utc.from_utc_datetime(
        &day.and_hms_opt(0, 0, 0)
            .ok_or(DomainError::InvalidStoredData)?,
    );
    let end = Utc.from_utc_datetime(
        &day.succ_opt()
            .ok_or(DomainError::InvalidStoredData)?
            .and_hms_opt(0, 0, 0)
            .ok_or(DomainError::InvalidStoredData)?,
    );
    Ok((start, end))
}

fn day_bounds_for_date(
    year: i32,
    month: u32,
    day: u32,
) -> Result<(DateTime<Utc>, DateTime<Utc>), DomainError> {
    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or(DomainError::InvalidStoredData)?;
    day_bounds(date)
}

fn month_bounds(year: i32, month: u32) -> Result<(DateTime<Utc>, DateTime<Utc>), DomainError> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).ok_or(DomainError::InvalidStoredData)?;
    let next = if month == 12 {
        NaiveDate::from_ymd_opt(
            year.checked_add(1).ok_or(DomainError::InvalidStoredData)?,
            1,
            1,
        )
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .ok_or(DomainError::InvalidStoredData)?;
    day_bounds(first).map(|(start, _)| {
        let end = Utc.from_utc_datetime(&next.and_hms_opt(0, 0, 0).expect("validated date"));
        (start, end)
    })
}

fn year_bounds(year: i32) -> Result<(DateTime<Utc>, DateTime<Utc>), DomainError> {
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1).ok_or(DomainError::InvalidStoredData)?;
    let next_year = year.checked_add(1).ok_or(DomainError::InvalidStoredData)?;
    let end_date =
        NaiveDate::from_ymd_opt(next_year, 1, 1).ok_or(DomainError::InvalidStoredData)?;
    let start = Utc.from_utc_datetime(
        &start_date
            .and_hms_opt(0, 0, 0)
            .ok_or(DomainError::InvalidStoredData)?,
    );
    let end = Utc.from_utc_datetime(
        &end_date
            .and_hms_opt(0, 0, 0)
            .ok_or(DomainError::InvalidStoredData)?,
    );
    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::{
        Application, IssueEntryTicket, day_bounds, day_bounds_for_date, month_bounds, year_bounds,
    };
    use crate::domain::DomainError;
    use chrono::{NaiveDate, TimeZone, Utc};

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

    #[test]
    fn day_report_bounds_are_utc_midnight_and_exclusive_next_day() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let (start, end) = match day_bounds(day) {
            Ok(bounds) => bounds,
            Err(_) => panic!("a valid date should have report bounds"),
        };
        assert_eq!(start, Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap());
        assert_eq!(end, Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap());
    }

    #[test]
    fn month_and_year_report_bounds_cover_full_periods() {
        let (month_start, month_end) = match month_bounds(2024, 2) {
            Ok(bounds) => bounds,
            Err(_) => panic!("a valid month should have report bounds"),
        };
        assert_eq!(
            month_start,
            Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap()
        );
        assert_eq!(
            month_end,
            Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap()
        );

        let (year_start, year_end) = match year_bounds(2024) {
            Ok(bounds) => bounds,
            Err(_) => panic!("a valid year should have report bounds"),
        };
        assert_eq!(
            year_start,
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()
        );
        assert_eq!(year_end, Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap());
    }

    #[test]
    fn report_bounds_reject_invalid_or_unrepresentable_periods() {
        assert!(day_bounds_for_date(2025, 2, 30).is_err());
        assert!(month_bounds(2025, 13).is_err());
        assert!(year_bounds(i32::MAX).is_err());
    }
}
