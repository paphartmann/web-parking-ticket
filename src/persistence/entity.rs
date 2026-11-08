//! SeaORM table models, kept separate from domain types so database mapping
//! details do not leak into business rules.

use sea_orm::entity::prelude::*;

pub mod parking_ticket {
    use super::*;

    /// Row representation for `parking_tickets`.
    #[sea_orm::model]
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
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
        #[sea_orm(has_many)]
        pub payments: HasMany<payment::Entity>,
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod payment {
    use super::*;

    /// Row representation for `payments`.
    #[sea_orm::model]
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "payments")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub ticket_id: Uuid,
        #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
        pub amount: Decimal,
        pub paid_at: DateTimeWithTimeZone,
        #[sea_orm(belongs_to, from = "ticket_id", to = "id")]
        pub parking_ticket: BelongsTo<parking_ticket::Entity>,
    }

    impl ActiveModelBehavior for ActiveModel {}
}
