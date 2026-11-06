//! Repository boundary for PostgreSQL storage through SeaORM.

pub mod entity;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{
    domain::{
        DomainError,
        ParkingTicket, Payment, TicketStatus,
    },
    persistence::entity::{parking_ticket, payment},
};

impl TryFrom<entity::parking_ticket::Model> for ParkingTicket {
    type Error = DomainError;

    fn try_from(model: entity::parking_ticket::Model) -> Result<Self, Self::Error> {
        let status = match model.status.as_str() {
            "active" => TicketStatus::Active,
            "courtesy" => TicketStatus::Courtesy,
            "paid" => TicketStatus::Paid,
            "released" => TicketStatus::Released,
            _ => return Err(DomainError::InvalidStoredData),
        };

        Ok(Self {
            id: model.id,
            code: model.code,
            vehicle_plate: model.vehicle_plate,
            entered_at: model.entered_at.with_timezone(&Utc),
            paid_at: model.paid_at.map(|at| at.with_timezone(&Utc)),
            released_at: model.released_at.map(|at| at.with_timezone(&Utc)),
            status,
        })
    }
}

impl From<entity::payment::Model> for Payment {
    fn from(model: entity::payment::Model) -> Self {
        Self {
            id: model.id,
            ticket_id: model.ticket_id,
            amount: model.amount,
            paid_at: model.paid_at.with_timezone(&Utc),
        }
    }
}

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
    pub async fn count_courtesy_between(
        &self,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<usize, DbErr> {
        Ok(entity::parking_ticket::Entity::find()
            .all(&self.database)
            .await?
            .iter()
            .map(|pm: &parking_ticket::Model| ParkingTicket::try_from(pm.clone()))
            .filter(|rp| {
                if let Ok(p) = rp {
                    p.status == TicketStatus::Courtesy
                } else {
                    false
                }
            })
            .count()
            .into())
    }

    pub async fn find_active_ticket_by_plate(
        &self,
        plate: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        let ticket = entity::parking_ticket::Entity::find()
            .filter(entity::parking_ticket::Column::VehiclePlate.eq(plate))
            .one(&self.database)
            .await
            .map_err(|_| DomainError::TicketNotFound)?;

        ticket.map(ParkingTicket::try_from).transpose()
    }

    pub async fn find_ticket_by_code(
        &self,
        code: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        let ticket = entity::parking_ticket::Entity::find()
            .filter(entity::parking_ticket::Column::Code.eq(code))
            .one(&self.database)
            .await
            .map_err(|_| DomainError::TicketNotFound)?;

        ticket.map(ParkingTicket::try_from).transpose()
    }

    pub async fn find_ticket_by_id(&self, id: Uuid) -> Result<Option<ParkingTicket>, DomainError> {
        let ticket = entity::parking_ticket::Entity::find_by_id(id)
            .one(&self.database)
            .await
            .map_err(|_| DomainError::TicketNotFound)?;

        ticket.map(ParkingTicket::try_from).transpose()
    }

    pub async fn save_payment(&self, payment: &Payment) -> Result<(), DbErr> {
        use sea_orm::{ActiveModelTrait, Set};

        let payment_model = entity::payment::ActiveModel {
            id: Set(payment.id),
            ticket_id: Set(payment.ticket_id),
            amount: Set(payment.amount),
            paid_at: Set(payment.paid_at.into()),
        };

        payment_model.insert(&self.database).await?;
        Ok(())
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

    pub async fn total_received_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Decimal, DbErr> {
        Ok(entity::payment::Entity::find()
            .filter(payment::Column::PaidAt.gte(start))
            .filter(payment::Column::PaidAt.lte(end))
            .all(&self.database)
            .await?
            .iter()
            .map(|m| m.amount)
            .sum())
    }
}
