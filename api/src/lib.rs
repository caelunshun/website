pub mod db;
pub mod docs;
pub mod docsbuilder;
pub mod featherurl;
pub mod github;
pub mod rejections;
pub mod types;

mod routes;
pub use routes::routes;

use std::convert::Infallible;

pub use db::DB;
use warp::{Filter, Rejection};

pub fn with_state<S: Clone + Send + 'static>(
    state: S,
) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn authenticated(db: DB) -> impl Filter<Extract = (u32,), Error = Rejection> + Clone {
    warp::header::header("Authorization").and_then(move |token: String| {
        let db = db.clone();
        async move {
            let mut secret = [0u8; 48];
            hex::decode_to_slice(&token, &mut secret).map_err(|_| rejections::unauthorized())?;

            let user_id = db
                .get_user_id_by_token_secret(secret.as_ref())
                .await
                .map_err(|_| rejections::unauthorized())?;

            Ok::<_, warp::reject::Rejection>(user_id)
        }
    })
}
