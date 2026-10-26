use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ParkingTicket {
    pub id: Uuid,
    pub code: String,
    pub vehicle_plate: String,
    pub entered_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub status: TicketStatus,
}

pub enum TicketStatus {
    Active,
    Courtesy,
    Paid,
    Released,
}

pub trait ParkingTicketBehavior {
    fn is_within_courtesy_period(&self, at: DateTime<Utc>) -> bool;
    fn mark_as_courtesy(&mut self, at: DateTime<Utc>);
    fn mark_as_paid(&mut self, at: DateTime<Utc>);
    fn mark_as_released(&mut self, at: DateTime<Utc>);
    fn can_exit(&self, at: DateTime<Utc>) -> bool;
}

