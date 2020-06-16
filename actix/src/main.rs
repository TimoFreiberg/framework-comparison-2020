#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use std::sync::Arc;

use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::footballer_repository::FootballerRepository;
use log::LevelFilter;

pub mod footballer;
pub mod footballer_repository;
pub mod footballer_rest_controller;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn")
    }

    env_logger::builder()
        .filter(Some("actix_server::builder"), LevelFilter::Info)
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_e| {
        error!("DATABASE_URL environment variable not set");
        std::process::exit(1);
    });

    let pool = Arc::new(init_pool(database_url.as_str()));

    let repository = Arc::new(FootballerRepository::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .data(repository.clone())
            .wrap(middleware::Logger::default())
            .service(footballer_rest_controller::footballer_create)
            .service(footballer_rest_controller::footballer_delete)
            .service(footballer_rest_controller::footballer_get)
            .service(footballer_rest_controller::footballer_search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub fn init_pool(connection_string: &str) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_string);
    Pool::builder()
        .max_size(5)
        .min_idle(Some(5))
        .build(connection_manager)
        .expect("Failed to create db connection pool")
}
