use actix_web::{error::BlockingError, web, App, HttpServer};
use anyhow::format_err;
use log::{warn, LevelFilter};
use refinery::config::{Config, ConfigDbType};
use sqlx::PgPool;
use url::Url;

pub mod api;
pub mod footballer;
pub mod footballer_repository;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!();
}

async fn migrate(url: String) -> Result<(), BlockingError<anyhow::Error>> {
    web::block(move || {
        let parsed_url = Url::parse(url.as_str())?;

        let mut connection = Config::new(ConfigDbType::Postgres)
            .set_db_host(parsed_url.host_str().unwrap())
            .set_db_name(&parsed_url.path().to_string()[1..])
            .set_db_pass(parsed_url.password().unwrap())
            .set_db_port(&format!("{}", parsed_url.port().unwrap()))
            .set_db_user(parsed_url.username());

        warn!("Migrate Database");
        embedded::migrations::runner().run(&mut connection)?;
        Ok(())
    })
    .await
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn")
    };

    env_logger::builder()
        .filter(Some("actix_server::builder"), LevelFilter::Info)
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| format_err!("DATABASE_URL environment variable not set"))?;

    migrate(database_url.clone())
        .await
        .err()
        .map(|e| format_err!("Database migration failed with error: {}", e));

    let db_pool = PgPool::builder()
        .max_size(15)
        .min_size(5)
        .build(&database_url)
        .await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(api::footballer_create)
            .service(api::footballer_delete)
            .service(api::footballer_get)
            .service(api::footballer_search)
    })
    .bind("0.0.0.0:8080")?
    .keep_alive(None)
    .run()
    .await?;

    Ok(())
}
