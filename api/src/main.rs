mod config;
mod db;
mod error;
mod handler;
use crate::handler::create_discount;
use axum::routing::get;
use axum::{routing::post, serve, Router};
use config::Config;
use db::run_migrations;
use deadpool_diesel::postgres::Pool;
use get_pass::url::add_pass_to_url;
use handler::{all_discounts, check_code, delete_discount, read_discount, update_discount};
#[derive(Clone)]
struct AppState {
    config: Config,
    pool: Pool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let config: Config = confy::load_path("/etc/name_api/config.toml")?;

    let mut uri_db = config.db_uri.clone();
    add_pass_to_url(&mut uri_db, &config.db_pass_path)?;
    let pool = Pool::builder(deadpool_diesel::Manager::new(
        uri_db,
        deadpool_diesel::Runtime::Tokio1,
    ))
    .build()?;
    run_migrations(&pool).await?;
    let state = AppState { config, pool };
    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", state.config.listen_port)).await?;
    serve(listener, router(state)).await?;
    Ok(())
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/discounts", post(create_discount).get(all_discounts))
        .route(
            "/discounts/:id",
            get(read_discount)
                .put(update_discount)
                .delete(delete_discount),
        )
        .route("/check_validity/:code", get(check_code))
        .with_state(state)
}
