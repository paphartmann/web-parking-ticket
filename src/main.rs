//! Rocket entry point and HTTP adapter for the parking service.
//!
//! Request DTOs live here; business workflows are represented by
//! [`application`] types and persisted through [`persistence`].

mod application;
mod domain;
mod persistence;
mod presentation;

use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::routes;
use sea_orm::Database;

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
        .attach(AdHoc::on_response(
            "Content Security Policy",
            |_, response| {
                Box::pin(async move {
                    response.set_header(rocket::http::Header::new(
                        "Content-Security-Policy",
                        "default-src 'self'",
                    ));
                })
            },
        ))
        .manage(repositories)
        .mount("/", FileServer::from("frontend/public"))
        .mount(
            "/",
            routes![
                presentation::issue_entry_ticket,
                presentation::list_tickets,
                presentation::validate_exit,
                presentation::calculate_charge,
                presentation::register_payment,
                presentation::revenue_report,
                presentation::courtesy_report,
                presentation::health
            ],
        )
}
