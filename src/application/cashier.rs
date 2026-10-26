use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{entities::payment::Payment, errors::DomainError};

pub struct CalculateCharge {
    pub ticket_code: String,
    pub calculated_at: DateTime<Utc>,
}

pub struct ChargeQuote {
    pub ticket_id: Uuid,
    pub amount: Decimal,
    pub courtesy: bool,
}

pub struct RegisterPayment {
    pub ticket_id: Uuid,
    pub amount_received: Decimal,
    pub paid_at: DateTime<Utc>,
}

pub trait CashierUseCases: Send + Sync {
    async fn calculate_charge(&self, request: CalculateCharge) -> Result<ChargeQuote, DomainError>;
    async fn release_ticket_after_payment(
        &self,
        request: RegisterPayment,
    ) -> Result<Payment, DomainError>;
}
