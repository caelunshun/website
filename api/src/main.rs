use std::{convert::Infallible, str::FromStr, time::Duration};

use anyhow::Result;
use feather_web_api::{rejections, routes, DB};
use futures::FutureExt;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};

use dotenv::dotenv;
use warp::{hyper::{StatusCode, Method, header}, Filter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut postgres = PgConnectOptions::new().application_name("api");

    postgres
        .disable_statement_logging()
        .log_slow_statements(log::LevelFilter::Error, Duration::from_millis(200));

    let pool = PgPoolOptions::new().connect_with(postgres).await?;

    let db = DB::new(pool);

    sqlx::migrate!("./migrations").run(db.as_ref()).await?;

    let routes = routes(db);

    let routes = routes.with(warp::log("api"));

    let routes = routes.recover(handle_rejection);
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header(header::AUTHORIZATION)
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        ;
    let routes = routes.with(cors);

    let ctrl_c = tokio::signal::ctrl_c();

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
        ([0, 0, 0, 0], 4000),
        ctrl_c.map(|_| log::info!(target: "api", "shutting down!")),
    );

    log::info!(target: "api", "listening on {}!", addr);

    server.await;

    log::info!(target: "api", "shutdown!");

    Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = "INTERNAL_SERVER_ERROR";
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(err) = err.find::<rejections::Database>() {
        log::info!(target: "api", "{:?}", err);
    } else if let Some(_) = err.find::<rejections::Unauthorized>() {
        code = StatusCode::UNAUTHORIZED;
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
    }

    log::info!("err: {:?}", err);

    let json = warp::reply::json(&message);

    Ok(warp::reply::with_status(json, code))
}
