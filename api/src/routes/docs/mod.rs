use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("docs").and(warp::path::end()).and(filter_docs())
}

pub fn filter_docs() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get().and_then(handle_docs)
}

pub async fn handle_docs() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&"summary"))
}