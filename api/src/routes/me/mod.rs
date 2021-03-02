use chrono::{DateTime, Utc};
use futures::future::join;
use serde::Serialize;
use warp::{Filter, Rejection, Reply};

use crate::{
    authenticated,
    rejections::{self, IntoRejection},
    with_state, DB,
};

mod authorization;

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("me").and(filter_me(db.clone()).or(authorization::routes(db)))
}

pub fn filter_me(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path::end()
        .and(authenticated(db.clone()))
        .and(with_state(db))
        .and_then(handle_me)
}

pub async fn handle_me(user_id: u32, db: DB) -> Result<impl Reply, Rejection> {
    let (user, tokens) = join(
        db.get_user_by_id(user_id),
        db.get_user_tokens_by_user_id(user_id),
    )
    .await;

    let (user, tokens) = (
        user.map_err(rejections::Database::reject)?,
        tokens.map_err(rejections::Database::reject)?,
    );

    #[derive(Serialize)]
    struct MeToken {
        id: u32,
        name: Option<String>,
        used_total: u32,
        created_at: DateTime<Utc>,
    }

    #[derive(Serialize)]
    struct Me {
        id: u32,
        login: String,
        name: String,
        tokens: Vec<MeToken>,
        created_at: DateTime<Utc>,
    }

    Ok(warp::reply::json(&Me {
        id: user.id,
        login: user.login,
        name: user.name,
        tokens: tokens
            .into_iter()
            .map(|token| MeToken {
                id: token.id,
                name: token.name,
                used_total: token.used_total,
                created_at: token.created_at,
            })
            .collect(),
        created_at: user.created_at,
    }))
}
