//! Repository pattern and relational persistence boundary (SeaORM/PostgreSQL).

use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::domain::{ParkingTicket, Payment};

pub struct Repositories {
    pub database: DatabaseConnection,
}

impl Repositories {
    pub async fn find_ticket_by_id(&self, _id: Uuid) {}
    pub async fn find_ticket_by_code(&self, _code: &str) {}
    pub async fn find_active_ticket_by_plate(&self, _plate: &str) {}
    pub async fn save_ticket(&self, _ticket: &ParkingTicket) {}
    pub async fn save_payment(&self, _payment: &Payment) {}
    pub async fn total_received_between(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) {}
    pub async fn count_courtesy_between(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) {}
}
