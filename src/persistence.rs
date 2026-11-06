//! Repository boundary for PostgreSQL storage through SeaORM.

pub mod entity;

use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::domain::{DomainError, ParkingTicket, Payment};

/// Database-backed operations used to store and retrieve domain records.
pub struct Repositories {
    /// Shared SeaORM connection used by repository methods.
    pub database: DatabaseConnection,
}

impl Repositories {
    /// Create the application's tables and insert stable demo rows once.
    /// The seed IDs and ticket code are fixed so rerunning startup is safe.
    pub async fn initialize(database: DatabaseConnection) -> Result<Self, sea_orm::DbErr> {
        use sea_orm::ConnectionTrait;

        database.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS parking_tickets (
                id UUID PRIMARY KEY,
                code TEXT NOT NULL UNIQUE,
                vehicle_plate TEXT NOT NULL,
                entered_at TIMESTAMPTZ NOT NULL,
                paid_at TIMESTAMPTZ,
                released_at TIMESTAMPTZ,
                status TEXT NOT NULL CHECK (status IN ('active', 'courtesy', 'paid', 'released'))
            );
            CREATE INDEX IF NOT EXISTS parking_tickets_active_plate_idx
                ON parking_tickets (vehicle_plate) WHERE status IN ('active', 'courtesy', 'paid');
            CREATE TABLE IF NOT EXISTS payments (
                id UUID PRIMARY KEY,
                ticket_id UUID NOT NULL REFERENCES parking_tickets(id),
                amount NUMERIC(10, 2) NOT NULL CHECK (amount >= 0),
                paid_at TIMESTAMPTZ NOT NULL
            );
            CREATE INDEX IF NOT EXISTS payments_paid_at_idx ON payments (paid_at);
            INSERT INTO parking_tickets (id, code, vehicle_plate, entered_at, status)
            VALUES ('11111111-1111-4111-8111-111111111111', 'DEMO-ACTIVE', 'ABC1234', NOW() - INTERVAL '40 minutes', 'active'),
                   ('22222222-2222-4222-8222-222222222222', 'DEMO-COURTESY', 'XYZ9876', NOW() - INTERVAL '10 minutes', 'courtesy')
            ON CONFLICT (id) DO NOTHING;",
        )
        .await?;
        Ok(Self { database })
    }
}

impl Repositories {
    pub async fn find_ticket_by_id(&self, _id: Uuid) -> Result<Option<ParkingTicket>, DomainError> {
        Ok(None)
    }

    pub async fn find_ticket_by_code(
        &self,
        _code: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        Ok(None)
    }

    pub async fn find_active_ticket_by_plate(
        &self,
        _plate: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        Ok(None)
    }

    pub async fn save_ticket(&self, ticket: &ParkingTicket) -> Result<(), DomainError> {
        use sea_orm::{ActiveModelTrait, Set};

        let ticket_model = entity::parking_ticket::ActiveModel {
            id: Set(ticket.id),
            code: Set(ticket.code.clone()),
            vehicle_plate: Set(ticket.vehicle_plate.clone()),
            entered_at: Set(ticket.entered_at.into()),
            paid_at: Set(ticket.paid_at.map(Into::into)),
            released_at: Set(ticket.released_at.map(Into::into)),
            status: Set(match &ticket.status {
                crate::domain::TicketStatus::Active => "active",
                crate::domain::TicketStatus::Courtesy => "courtesy",
                crate::domain::TicketStatus::Paid => "paid",
                crate::domain::TicketStatus::Released => "released",
            }
            .to_owned()),
        };

        ticket_model
            .insert(&self.database)
            .await
            .map_err(|_| DomainError::TicketNotFound)?;
        Ok(())
    }

    pub async fn save_payment(&self, _payment: &Payment) -> Result<(), DomainError> {
        Ok(())
    }

    pub async fn total_received_between(
        &self,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<rust_decimal::Decimal, DomainError> {
        Ok(rust_decimal::Decimal::ZERO)
    }

    pub async fn count_courtesy_between(
        &self,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<u64, DomainError> {
        Ok(0)
    }
}
