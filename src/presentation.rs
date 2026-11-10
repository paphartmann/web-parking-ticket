//! Rocket controllers: translate HTTP requests into application operations.

use chrono::{Datelike, NaiveDate, Utc};
use rocket::serde::{Deserialize, json::Json};
use rocket::{State, get, http, post};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::application::{
    Application, CalculateCharge, ChargeQuote, ExitValidation, IssueEntryTicket, RegisterPayment,
    ValidateExit,
};
use crate::domain::{CourtesyReport, DomainError, ParkingTicket, Payment, RevenueReport};
use crate::persistence::Repositories;

fn domain_error_status(error: DomainError) -> http::Status {
    match error {
        DomainError::TicketNotFound => http::Status::NotFound,
        DomainError::InvalidStoredData => http::Status::InternalServerError,
        DomainError::TicketAlreadyReleased | DomainError::TicketNotEligibleForExit => {
            http::Status::Conflict
        }
        DomainError::InvalidVehiclePlate | DomainError::PaymentAmountMismatch => {
            http::Status::UnprocessableEntity
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub(crate) struct EntryTicketRequest {
    vehicle_plate: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub(crate) struct ExitValidationRequest {
    ticket_code: Option<String>,
    vehicle_plate: Option<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub(crate) struct PaymentRequest {
    ticket_id: Uuid,
    amount_received: Decimal,
}

#[post("/api/barrier/entry-tickets", data = "<request>")]
pub async fn issue_entry_ticket(
    repositories: &State<Repositories>,
    request: Json<EntryTicketRequest>,
) -> Result<Json<ParkingTicket>, http::Status> {
    let ticket = Application
        .issue_entry_ticket(IssueEntryTicket {
            vehicle_plate: request.vehicle_plate.clone(),
        })
        .await
        .map_err(domain_error_status)?;

    repositories
        .save_ticket(&ticket)
        .await
        .map_err(|_| rocket::http::Status::InternalServerError)?;

    Ok(Json(ticket))
}

#[get("/api/tickets")]
pub async fn list_tickets(
    repositories: &State<Repositories>,
) -> Result<Json<Vec<ParkingTicket>>, http::Status> {
    repositories
        .list_tickets()
        .await
        .map(Json)
        .map_err(|_| rocket::http::Status::InternalServerError)
}

#[post("/api/barrier/exit-validation", data = "<request>")]
pub async fn validate_exit(
    repositories: &State<Repositories>,
    request: Json<ExitValidationRequest>,
) -> Result<Json<ExitValidation>, http::Status> {
    if request.ticket_code.is_none() && request.vehicle_plate.is_none() {
        return Err(http::Status::BadRequest);
    }

    Application
        .validate_exit(
            ValidateExit {
                ticket_code: request.ticket_code.clone(),
                vehicle_plate: request.vehicle_plate.clone(),
                requested_at: Utc::now(),
            },
            repositories,
            false,
        )
        .await
        .map(Json)
        .map_err(domain_error_status)
}

#[post("/api/barrier/open_gate", data = "<request>")]
pub async fn open_gate(
    repositories: &State<Repositories>,
    request: Json<ExitValidationRequest>,
) -> Result<Json<Uuid>, http::Status> {
    if request.ticket_code.is_none() && request.vehicle_plate.is_none() {
        return Err(http::Status::BadRequest);
    }

    Application
        .validate_exit(
            ValidateExit {
                ticket_code: request.ticket_code.clone(),
                vehicle_plate: request.vehicle_plate.clone(),
                requested_at: Utc::now(),
            },
            repositories,
            true,
        )
        .await
        .map(|ev| Json(ev.ticket_id))
        .map_err(domain_error_status)
}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
pub async fn calculate_charge(
    repositories: &State<Repositories>,
    ticket_code: &str,
) -> Result<Json<ChargeQuote>, http::Status> {
    let result = Application
        .calculate_charge(
            CalculateCharge {
                ticket_code: ticket_code.to_string(),
                calculated_at: Utc::now(),
            },
            repositories,
        )
        .await;

    result.map(Json).map_err(domain_error_status)
}

#[post("/api/cashier/payments", data = "<request>")]
pub async fn register_payment(
    repositories: &State<Repositories>,
    request: Json<PaymentRequest>,
) -> Result<Json<Payment>, http::Status> {
    Application
        .register_payment(
            RegisterPayment {
                ticket_id: request.ticket_id,
                amount_received: request.amount_received,
                paid_at: Utc::now(),
            },
            repositories,
        )
        .await
        .map(Json)
        .map_err(domain_error_status)
}

#[get("/api/management/reports/revenue?<day>&<month>&<year>")]
pub async fn revenue_report(
    repositories: &State<Repositories>,
    day: Option<&str>,
    month: Option<u32>,
    year: Option<i32>,
) -> Result<Json<RevenueReport>, http::Status> {
    let report = match day {
        Some(day) => {
            let date = parse_report_day(day)?;
            Application
                .revenue_for_day(date.year(), date.month(), date.day(), repositories)
                .await
        }
        None if month.is_some() => {
            let Some(year) = year else {
                return Err(http::Status::BadRequest);
            };
            Application
                .revenue_for_month(year, month.unwrap(), repositories)
                .await
        }
        None => {
            let Some(year) = year else {
                return Err(http::Status::BadRequest);
            };
            Application.revenue_for_year(year, repositories).await
        }
    };
    report.map(Json).map_err(domain_error_status)
}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
pub async fn courtesy_report(
    repositories: &State<Repositories>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Json<CourtesyReport>, http::Status> {
    let report = match day {
        Some(day) => {
            let date = parse_report_day(day)?;
            Application
                .courtesy_count_for_day(date.year(), date.month(), date.day(), repositories)
                .await
        }
        None if month.is_some() => {
            let Some(year) = year else {
                return Err(http::Status::BadRequest);
            };
            Application
                .courtesy_count_for_month(year, month.unwrap(), repositories)
                .await
        }
        None => {
            let Some(year) = year else {
                return Err(http::Status::BadRequest);
            };
            Application
                .courtesy_count_for_year(year, repositories)
                .await
        }
    };

    report.map(Json).map_err(domain_error_status)
}

fn parse_report_day(day: &str) -> Result<NaiveDate, http::Status> {
    NaiveDate::parse_from_str(day, "%Y-%m-%d").map_err(|_| http::Status::BadRequest)
}

#[cfg(test)]
mod report_query_tests {
    use super::parse_report_day;
    use chrono::NaiveDate;
    use rocket::http::Status;

    #[test]
    fn parses_iso_report_day() {
        assert_eq!(
            parse_report_day("2024-02-29"),
            Ok(NaiveDate::from_ymd_opt(2024, 2, 29).unwrap())
        );
    }

    #[test]
    fn rejects_invalid_report_day() {
        assert_eq!(parse_report_day("2025-02-30"), Err(Status::BadRequest));
        assert_eq!(parse_report_day("not-a-date"), Err(Status::BadRequest));
    }
}

#[get("/health")]
pub async fn health(_repositories: &State<Repositories>) -> &'static str {
    "ok"
}
