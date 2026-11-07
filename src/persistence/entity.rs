//! SeaORM table models, kept separate from domain types so database mapping
//! details do not leak into business rules.

use sea_orm::entity::prelude::*;

pub mod parking_ticket {
    use super::*;

    /// Row representation for `parking_tickets`.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, DeriveActiveModelBehavior)]
    #[sea_orm(table_name = "parking_tickets")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        #[sea_orm(unique)]
        pub code: String,
        pub vehicle_plate: String,
        pub entered_at: DateTimeWithTimeZone,
        pub paid_at: Option<DateTimeWithTimeZone>,
        pub released_at: Option<DateTimeWithTimeZone>,
        pub status: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::payment::Entity")]
        Payment,
    }

    impl Related<super::payment::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Payment.def()
        }
    }
}

pub mod payment {
    use super::*;

    /// Row representation for `payments`.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, DeriveActiveModelBehavior)]
    #[sea_orm(table_name = "payments")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub ticket_id: Uuid,
        #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
        pub amount: Decimal,
        pub paid_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::parking_ticket::Entity",
            from = "Column::TicketId",
            to = "super::parking_ticket::Column::Id",
            on_update = "Cascade",
            on_delete = "Restrict"
        )]
        ParkingTicket,
    }

    impl Related<super::parking_ticket::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ParkingTicket.def()
        }
    }
}
