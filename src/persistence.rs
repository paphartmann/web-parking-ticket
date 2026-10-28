//! Repository pattern and relational persistence boundary (SeaORM/PostgreSQL).

use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::domain::{ParkingTicket, Payment};

pub struct Repositories {
    pub database: DatabaseConnection,
}

impl Repositories {
    pub async fn find_ticket_by_id(&self, id: Uuid) {}
    pub async fn find_ticket_by_code(&self, code: &str) {}
    pub async fn find_active_ticket_by_plate(&self, plate: &str) {}
    pub async fn save_ticket(&self, ticket: &ParkingTicket) {}
    pub async fn save_payment(&self, payment: &Payment) {}
    pub async fn total_received_between(&self, start: DateTime<Utc>, end: DateTime<Utc>) {}
    pub async fn count_courtesy_between(&self, start: DateTime<Utc>, end: DateTime<Utc>) {}
}
