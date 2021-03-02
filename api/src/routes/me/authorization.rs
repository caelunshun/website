use rand::RngCore;
use warp::{Filter, Rejection, Reply};

use crate::{
    github,
    rejections::{self, IntoRejection},
    with_state, DB,
};

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filter_authorization(db)
}

pub fn filter_authorization(
    db: DB,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let client_id =
        std::env::var("GITHUB_OAUTH_CLIENT_ID").expect("GITHUB_OAUTH_CLIENT_ID missing");
    let client_secret =
        std::env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("GITHUB_OAUTH_CLIENT_SECRET missing");

    warp::path("authorization")
        .and(warp::query())
        .and(warp::path::end())
        .and(warp::any().map(move || client_id.clone()))
        .and(warp::any().map(move || client_secret.clone()))
        .and(with_state(db))
        .and_then(handle_authorization)
}

pub async fn handle_authorization(
    code: String,
    client_id: String,
    client_secret: String,
    db: DB,
) -> Result<impl Reply, Rejection> {
    let github_access_token = github::access_token(&client_id, &client_secret, &code)
        .await
        .map_err(|_| rejections::unauthorized())?;

    let github_user = github::user(&github_access_token)
        .await
        .map_err(|_| rejections::unauthorized())?;

    let token = db
        .get_user_token_by_user_id_and_name(github_user.id, None)
        .await;

    let mut secret = [0; 48];

    let secret = match token {
        Ok(ref token) => token.secret.as_slice(),
        Err(sqlx::Error::RowNotFound) => {
            rand::rngs::OsRng.fill_bytes(&mut secret);
            db.insert_user_token_by_user_id_and_name(github_user.id, None, secret.as_ref())
                .await
                .map_err(rejections::Database::reject)?;
            secret.as_ref()
        }
        Err(error) => return Err(rejections::Database::reject(error)),
    };

    let secret_hex = hex::encode(secret);

    Ok(warp::reply::json(&secret_hex))
}
