//! Rocket controllers: translate HTTP requests into application operations.

use chrono::Utc;
use rocket::serde::{Deserialize, json::Json};
use rocket::{State, get, post};
use uuid::Uuid;

use crate::application::{Application, ChargeQuote, ExitValidation, IssueEntryTicket};
use crate::domain::{CourtesyReport, ParkingTicket, Payment, RevenueReport};
use crate::persistence::Repositories;

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
    amount_received: rust_decimal::Decimal,
}

#[post("/api/barrier/entry-tickets", data = "<request>")]
pub async fn issue_entry_ticket(
    repositories: &State<Repositories>,
    request: Json<EntryTicketRequest>,
) -> Result<Json<ParkingTicket>, rocket::http::Status> {
    let ticket = Application
        .issue_entry_ticket(IssueEntryTicket {
            vehicle_plate: request.vehicle_plate.clone(),
        })
        .await
        .map_err(|_| rocket::http::Status::UnprocessableEntity)?;
    repositories
        .save_ticket(&ticket)
        .await
        .map_err(|_| rocket::http::Status::InternalServerError)?;
    Ok(Json(ticket))
}

#[post("/api/barrier/exit-validation", data = "<request>")]
pub async fn validate_exit(
    repositories: &State<Repositories>,
    request: Json<ExitValidationRequest>,
) -> Json<ExitValidation> {
    let null_result = Json(ExitValidation {
        ticket_id: Uuid::nil(),
        may_open_barrier: false,
    });

    let Some(code) = &request.ticket_code else {
        return null_result;
    };

    let Ok(Some(ticket)) = repositories.find_ticket_by_code(&code).await else {
        return null_result;
    };

    Json(ExitValidation {
        ticket_id: ticket.id,
        may_open_barrier: ticket.can_exit(Utc::now()),
    })
}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
pub async fn calculate_charge(
    _repositories: &State<Repositories>,
    ticket_code: &str,
) -> Json<ChargeQuote> {
    let _ = ticket_code;
    Json(ChargeQuote::default())
}

#[post("/api/cashier/payments", data = "<request>")]
pub async fn register_payment(
    _repositories: &State<Repositories>,
    request: Json<PaymentRequest>,
) -> Json<Payment> {
    let _ = request;
    Json(Payment::default())
}

#[get("/api/management/reports/revenue?<day>&<year>&<month>")]
pub async fn revenue_report(
    _repositories: &State<Repositories>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<RevenueReport> {
    let _ = (day, year, month);
    Json(RevenueReport::default())
}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
pub async fn courtesy_report(
    _repositories: &State<Repositories>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<CourtesyReport> {
    let _ = (day, year, month);
    Json(CourtesyReport::default())
}

#[get("/health")]
pub async fn health(_repositories: &State<Repositories>) -> &'static str {
    "ok"
}
