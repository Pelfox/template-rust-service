mod routers;
mod database;

use actix_web::{App, HttpServer, web};
use anyhow::Context;
use env_logger::Env;
use log::info;
use sea_orm::DatabaseConnection;

pub struct AppData {
    pub database: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database = database::initialize_database(
        dotenvy::var("POSTGRES_HOST").unwrap_or("postgres".to_string()),
        dotenvy::var("POSTGRES_PORT").unwrap_or("5432".to_string()),
        dotenvy::var("POSTGRES_USERNAME").unwrap_or("postgres".to_string()),
        dotenvy::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string()),
        dotenvy::var("POSTGRES_DATABASE").unwrap_or("postgres".to_string()),
    )
        .await
        .context("unable to initialize database")
        .unwrap()
        ;

    let bind_addr = dotenvy::var("BIND_ADDR").unwrap_or("127.0.0.1:3000".to_string());
    info!("Starting webserver on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData { database: database.clone() }))
            .configure(routers::config)
    })
        .bind(bind_addr)?
        .run()
        .await
}
