//! Repository boundary for PostgreSQL storage through SeaORM.

pub mod entity;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    domain::{DomainError, ParkingTicket, Payment, TicketStatus},
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
    pub async fn list_tickets(&self) -> Result<Vec<ParkingTicket>, DomainError> {
        entity::parking_ticket::Entity::find()
            .all(&self.database)
            .await
            .map_err(|_| DomainError::TicketNotFound)?
            .into_iter()
            .map(ParkingTicket::try_from)
            .collect()
    }

    pub async fn count_courtesy_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<usize, DbErr> {
        Ok(entity::parking_ticket::Entity::find()
            .filter(parking_ticket::Column::EnteredAt.gte(start))
            .filter(parking_ticket::Column::EnteredAt.lt(end))
            .filter(parking_ticket::Column::Status.eq("courtesy"))
            .all(&self.database)
            .await?
            .len())
    }

    pub async fn find_active_ticket_by_plate(
        &self,
        plate: &str,
    ) -> Result<Option<ParkingTicket>, DomainError> {
        let ticket = entity::parking_ticket::Entity::find()
            .filter(entity::parking_ticket::Column::VehiclePlate.eq(plate))
            .filter(entity::parking_ticket::Column::Status.ne("released"))
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
        use sea_orm::{ActiveModelTrait, Set, TransactionTrait};

        let transaction = self.database.begin().await?;
        let payment_model = entity::payment::ActiveModel {
            id: Set(payment.id),
            ticket_id: Set(payment.ticket_id),
            amount: Set(payment.amount),
            paid_at: Set(payment.paid_at.into()),
        };

        payment_model.insert(&transaction).await?;
        let ticket = entity::parking_ticket::Entity::find_by_id(payment.ticket_id)
            .one(&transaction)
            .await?
            .ok_or(DbErr::RecordNotFound("parking ticket".to_owned()))?;
        let mut ticket: entity::parking_ticket::ActiveModel = ticket.into();
        ticket.paid_at = Set(Some(payment.paid_at.into()));
        ticket.status = Set(if payment.amount.is_zero() {
            "courtesy"
        } else {
            "paid"
        }
        .to_owned());
        ticket.update(&transaction).await?;
        transaction.commit().await?;
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
            .filter(payment::Column::PaidAt.lt(end))
            .all(&self.database)
            .await?
            .iter()
            .map(|m| m.amount)
            .sum())
    }
}
