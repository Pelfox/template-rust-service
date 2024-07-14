use anyhow::Context;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

pub async fn initialize_database(
    host: String,
    port: String,
    username: String,
    password: String,
    database: String
) -> anyhow::Result<DatabaseConnection> {
    info!("Initializing database...");

    let mut options = ConnectOptions::new(format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, database));
    options.sqlx_logging(true).sqlx_logging_level(log::LevelFilter::Debug);

    let db: DatabaseConnection = Database::connect(options).await.context("unable to connect to the database")?;
    Migrator::up(&db, None).await.context("unable to auto-migrate models")?;

    info!("Database is initialized & migrated.");
    Ok(db)
}