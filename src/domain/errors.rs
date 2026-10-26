pub enum DomainError {
    TicketNotFound,
    TicketAlreadyReleased,
    TicketNotEligibleForExit,
    InvalidVehiclePlate,
    PaymentAmountMismatch,
}
