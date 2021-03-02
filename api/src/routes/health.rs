use std::convert::Infallible;

use sqlx::query;
use warp::{Filter, Rejection, Reply};

use crate::{with_state, DB};

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("health").and(filter_health(db.clone()))
}

pub fn filter_health(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path::end())
        .and(with_state(db))
        .and_then(handle_health)
}

pub async fn handle_health(db: DB) -> Result<impl Reply, Infallible> {
    let health = query("SELECT 42").execute(db.as_ref()).await;
    match health {
        Ok(_) => Ok(warp::http::StatusCode::OK),
        Err(_) => Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
