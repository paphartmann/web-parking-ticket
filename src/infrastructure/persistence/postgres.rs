use sea_orm::DatabaseConnection;

pub struct PostgresDatabase {
    pub connection: DatabaseConnection,
}

pub trait DatabaseConnector: Send + Sync {
    async fn connect(database_url: &str) -> Result<PostgresDatabase, sea_orm::DbErr>;
}
