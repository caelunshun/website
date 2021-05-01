use futures::{Stream};
use warp::{Buf, Filter, Rejection, Reply};

use crate::{authenticated, with_state, DB};

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("publish").and(filter_plugins(db))
}

pub fn filter_plugins(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path::end())
        .and(warp::body::stream())
        .and(authenticated(db.clone()))
        .and(with_state(db))
        .and_then(handle_publish)
}

pub async fn handle_publish(
    body: impl Stream<Item = Result<impl Buf, warp::Error>>,
    user_id: u32,
    db: DB,
) -> Result<impl Reply, Rejection> {
    struct Metadata {
        name: String,
        version: String,
        authors: Vec<String>,
        categories: Vec<String>,
    }

    // let reader = StreamReader::new(body);

    // Render README.md and upload to blob store
    todo!();

    let archs = 10;
    for _ in 0..archs {
        // Upload plugin tars to blob store
        todo!();
    }

    // Purge cloudflare
    todo!();

    Ok(warp::http::StatusCode::OK)
}
