use crate::application::cashier::{CalculateCharge, ChargeQuote, RegisterPayment};
use crate::domain::{entities::payment::Payment, errors::DomainError};

pub struct CashierController;

pub trait CashierMvcController {
    async fn calculate_charge(&self, request: CalculateCharge) -> Result<ChargeQuote, DomainError>;
    async fn register_payment(&self, request: RegisterPayment) -> Result<Payment, DomainError>;
}
