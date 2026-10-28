mod application;
mod domain;
mod persistence;
mod presentation;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::{get, post, routes};

#[post("/api/barrier/entry-tickets", data = "<request>")]
#[allow(unused_variables)]
async fn issue_entry_ticket(request: Json<Value>) {}

#[post("/api/barrier/exit-validation", data = "<request>")]
#[allow(unused_variables)]
async fn validate_exit(request: Json<Value>) {}

#[get("/api/cashier/tickets/<ticket_code>/charge")]
#[allow(unused_variables)]
async fn calculate_charge(ticket_code: &str) {}

#[post("/api/cashier/payments", data = "<request>")]
#[allow(unused_variables)]
async fn register_payment(request: Json<Value>) {}

#[get("/api/management/reports/revenue?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn revenue_report(day: Option<&str>, year: Option<i32>, month: Option<u32>) {}

#[get("/api/management/reports/courtesy?<day>&<year>&<month>")]
#[allow(unused_variables)]
async fn courtesy_report(day: Option<&str>, year: Option<i32>, month: Option<u32>) {}

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
