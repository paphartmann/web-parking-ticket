mod application;
mod domain;
mod infrastructure;
mod presentation;

use rocket::fs::FileServer;
use rocket::{get, post, routes};

#[post("/api/barrier/entry-tickets")]
async fn issue_entry_ticket() {}

#[post("/api/barrier/exit-validation")]
async fn validate_exit() {}

#[get("/health")]
async fn health() {}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("frontend/public"))
        .mount("/", routes![issue_entry_ticket, validate_exit, health])
}
