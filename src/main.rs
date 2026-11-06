//! Rocket entry point and HTTP adapter for the parking service.
//!
//! Request DTOs live here; business workflows are represented by
//! [`application`] types and persisted through [`persistence`].

mod application;
mod domain;
mod persistence;
mod presentation;

use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::{State, get, post, routes};
use sea_orm::Database;

use crate::application::{Application, ChargeQuote, ExitValidation, IssueEntryTicket};
use crate::domain::{CourtesyReport, ParkingTicket, Payment, RevenueReport};

/// JSON body accepted when a vehicle enters the parking lot.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct EntryTicketRequest {
    /// License plate supplied by the client; normalized by the domain model.
    vehicle_plate: String,
}

/// Identifiers the exit workflow can use to locate a parking ticket.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct ExitValidationRequest {
    ticket_code: Option<String>,
    vehicle_plate: Option<String>,
}

/// Cashier payment details supplied by the client.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct PaymentRequest {
    ticket_id: uuid::Uuid,
    amount_received: rust_decimal::Decimal,
}

#[post("/api/barrier/entry-tickets", data = "<request>")]
#[allow(unused_variables)]
async fn issue_entry_ticket(
    repositories: &State<persistence::Repositories>,
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
#[allow(unused_variables)]
async fn validate_exit(
    _database: &State<persistence::Repositories>,
    request: Json<ExitValidationRequest>,
) -> Json<ExitValidation> {
    Json(ExitValidation::default())
}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
#[allow(unused_variables)]
async fn calculate_charge(
    _database: &State<persistence::Repositories>,
    ticket_code: &str,
) -> Json<ChargeQuote> {
    Json(ChargeQuote::default())
}

#[post("/api/cashier/payments", data = "<request>")]
#[allow(unused_variables)]
async fn register_payment(
    _database: &State<persistence::Repositories>,
    request: Json<PaymentRequest>,
) -> Json<Payment> {
    Json(Payment::default())
}

#[get("/api/management/reports/revenue?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn revenue_report(
    _database: &State<persistence::Repositories>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<RevenueReport> {
    Json(RevenueReport::default())
}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn courtesy_report(
    _database: &State<persistence::Repositories>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<CourtesyReport> {
    Json(CourtesyReport::default())
}

#[get("/health")]
async fn health(_database: &State<persistence::Repositories>) -> &'static str {
    "ok"
}

#[rocket::launch]
async fn rocket() -> _ {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must point to a PostgreSQL database");
    let database = Database::connect(&database_url)
        .await
        .expect("could not connect to PostgreSQL");
    let repositories = persistence::Repositories::initialize(database)
        .await
        .expect("could not initialize persistence schema or demo data");

    rocket::build()
        .manage(repositories)
        .mount("/", FileServer::from("frontend/public"))
        .mount(
            "/",
            routes![
                issue_entry_ticket,
                validate_exit,
                calculate_charge,
                register_payment,
                revenue_report,
                courtesy_report,
                health
            ],
        )
}
