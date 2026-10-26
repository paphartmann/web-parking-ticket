use sea_orm::DatabaseConnection;

pub struct SeaOrmParkingTicketRepository {
    pub connection: DatabaseConnection,
}

