use sea_orm::DatabaseConnection;

pub struct SeaOrmPaymentRepository {
    pub connection: DatabaseConnection,
}

