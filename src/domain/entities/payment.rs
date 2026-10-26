use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct Payment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub amount: Decimal,
    pub paid_at: DateTime<Utc>,
}
