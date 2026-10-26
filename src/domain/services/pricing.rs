use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::domain::entities::parking_ticket::ParkingTicket;

pub trait PricingService: Send + Sync {
    fn amount_due(&self, ticket: &ParkingTicket, at: DateTime<Utc>) -> Decimal;
}
