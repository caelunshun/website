pub mod filters;
pub mod github;
pub mod handlers;
pub mod models;
pub mod rejections;

use std::convert::Infallible;

use anyhow::Result;
use futures::FutureExt;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

use dotenv::dotenv;
use warp::{hyper::StatusCode, Filter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let postgres = PgConnectOptions::new()
        .host("localhost")
        .username("feather")
        .password("feather")
        .ssl_mode(PgSslMode::Prefer);

    let pool = PgPoolOptions::new().connect_with(postgres).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let routes = filters::routes(pool);
    let routes = routes.recover(handle_rejection);
    let routes = routes.with(warp::log("api"));

    let ctrl_c = tokio::signal::ctrl_c();

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
        ([0, 0, 0, 0], 7000),
        ctrl_c.map(|_| log::info!(target: "api", "shutting down!")),
    );

    log::info!(target: "api", "listening on {}!", addr);

    server.await;

    log::info!(target: "api", "shutdown!");

    Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR";
    }

    let json = warp::reply::json(&message);

    Ok(warp::reply::with_status(json, code))
}
