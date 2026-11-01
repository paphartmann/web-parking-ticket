mod application;
mod domain;
mod persistence;
mod presentation;

use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::{get, post, routes, State};
use sea_orm::{Database, DatabaseConnection};

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
async fn issue_entry_ticket(
    _database: &State<DatabaseConnection>,
    request: Json<EntryTicketRequest>,
) -> Json<ParkingTicket> {
    Json(ParkingTicket::default())
}

#[post("/api/barrier/exit-validation", data = "<request>")]
#[allow(unused_variables)]
async fn validate_exit(
    _database: &State<DatabaseConnection>,
    request: Json<ExitValidationRequest>,
) -> Json<ExitValidation> {
    Json(ExitValidation::default())
}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
#[allow(unused_variables)]
async fn calculate_charge(
    _database: &State<DatabaseConnection>,
    ticket_code: &str,
) -> Json<ChargeQuote> {
    Json(ChargeQuote::default())
}

#[post("/api/cashier/payments", data = "<request>")]
#[allow(unused_variables)]
async fn register_payment(
    _database: &State<DatabaseConnection>,
    request: Json<PaymentRequest>,
) -> Json<Payment> {
    Json(Payment::default())
}

#[get("/api/management/reports/revenue?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn revenue_report(
    _database: &State<DatabaseConnection>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<RevenueReport> {
    Json(RevenueReport::default())
}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn courtesy_report(
    _database: &State<DatabaseConnection>,
    day: Option<&str>,
    year: Option<i32>,
    month: Option<u32>,
) -> Json<CourtesyReport> {
    Json(CourtesyReport::default())
}

#[get("/health")]
async fn health(_database: &State<DatabaseConnection>) -> &'static str {
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
        .manage(repositories.database)
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
