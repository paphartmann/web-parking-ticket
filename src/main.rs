mod application;
mod domain;
mod persistence;
mod presentation;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::{get, post, routes};

#[post("/api/barrier/entry-tickets")]
async fn issue_entry_ticket(_request: Json<Value>) {}

#[post("/api/barrier/exit-validation")]
async fn validate_exit(_request: Json<Value>) {}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
async fn calculate_charge(_ticket_code: &str) {}

#[post("/api/cashier/payments")]
async fn register_payment(_request: Json<Value>) {}

#[get("/api/management/reports/revenue?<day>&<year>&<month>")]
async fn revenue_report(
    _day: Option<&str>,
    _year: Option<i32>,
    _month: Option<u32>,
) {
}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
async fn courtesy_report(
    _day: Option<&str>,
    _year: Option<i32>,
    _month: Option<u32>,
) {
}

#[get("/health")]
async fn health() {}

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
