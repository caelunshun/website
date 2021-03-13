use std::sync::{Arc, Mutex};
use warp::{path::Tail, Filter, Rejection, Reply};

use crate::{
    github::{get_articles_of_association, get_budget},
    with_state,
};

type Cache = Arc<Mutex<String>>;

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cache: Cache = Default::default();

    warp::path("association").and(filter_budget(cache.clone()).or(filter_aoa(cache.clone())))
}

pub fn filter_budget(
    cache: Cache,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("budget"))
        .and(warp::path::end())
        .and(with_state(cache))
        .and_then(handle_budget)
}

pub fn filter_aoa(cache: Cache) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("articles-of-association"))
        .and(warp::path::end())
        .and(with_state(cache))
        .and_then(handle_aoa)
}

pub fn filter_webhook(
    cache: Cache,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("webhook"))
        .and(warp::path::tail())
        .and(with_state(cache))
        .and_then(handle_webhook)
}

pub async fn handle_budget(_cache: Cache) -> Result<impl Reply, Rejection> {
    Ok(get_budget().await.unwrap())
}

pub async fn handle_aoa(_cache: Cache) -> Result<impl Reply, Rejection> {
    Ok(get_articles_of_association().await.unwrap())
}

pub async fn handle_webhook(tail: Tail, cache: Cache) -> Result<impl Reply, Rejection> {
    // Please do
    Ok("")
}
