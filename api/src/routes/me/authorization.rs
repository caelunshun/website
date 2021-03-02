use rand::RngCore;
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct AccessToken {
    code: String,
}

pub async fn handle_authorization(
    access_token: AccessToken,
    client_id: String,
    client_secret: String,
    db: DB,
) -> Result<impl Reply, Rejection> {
    log::info!("1");
    let github_access_token = github::access_token(&client_id, &client_secret, &access_token.code)
        .await
        .map_err(|_| rejections::unauthorized())?;

    log::info!("2");
    let github_user = github::user(&github_access_token).await.unwrap();

    log::info!("3");
    db.insert_or_update_user(&github_user).await.unwrap();

    log::info!("4");
    let mut secret = [0; 48];

    rand::rngs::OsRng.fill_bytes(&mut secret);

    db.get_create_auth_token(github_user.id, "feathermc.org", &mut secret)
        .await
        .map_err(rejections::Database::reject)?;

    let secret_hex = hex::encode(secret);

    Ok(warp::reply::json(&secret_hex))
}
