mod application;
mod domain;
mod persistence;
mod presentation;

use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::{get, post, routes};

use crate::application::{ChargeQuote, ExitValidation};
use crate::domain::{CourtesyReport, ParkingTicket, Payment, RevenueReport};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct EntryTicketRequest {
    vehicle_plate: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct ExitValidationRequest {
    ticket_code: Option<String>,
    vehicle_plate: Option<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct PaymentRequest {
    ticket_id: uuid::Uuid,
    amount_received: rust_decimal::Decimal,
}

#[post("/api/barrier/entry-tickets", data = "<request>")]
#[allow(unused_variables)]
async fn issue_entry_ticket(request: Json<EntryTicketRequest>) -> Json<ParkingTicket> {
    Json(ParkingTicket::default())
}

#[post("/api/barrier/exit-validation", data = "<request>")]
#[allow(unused_variables)]
async fn validate_exit(request: Json<ExitValidationRequest>) -> Json<ExitValidation> {
    Json(ExitValidation::default())
}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
#[allow(unused_variables)]
async fn calculate_charge(ticket_code: &str) -> Json<ChargeQuote> {
    Json(ChargeQuote::default())
}

#[post("/api/cashier/payments", data = "<request>")]
#[allow(unused_variables)]
async fn register_payment(request: Json<PaymentRequest>) -> Json<Payment> {
    Json(Payment::default())
}

#[get("/api/management/reports/revenue?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn revenue_report(
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<RevenueReport> {
    Json(RevenueReport::default())
}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn courtesy_report(
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<CourtesyReport> {
    Json(CourtesyReport::default())
}

#[get("/health")]
async fn health() -> &'static str {
    "ok"
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
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
