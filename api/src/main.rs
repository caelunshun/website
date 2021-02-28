pub mod filters;
pub mod handlers;
pub mod models;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{convert::Infallible, sync::Arc};

use warp::{Filter, Reply, any, header, path::param};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://feather:feather@postgres/feather")
        .await?;

    let api = filters::routes(pool);

    warp::serve(any().map(|| "hello")).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
