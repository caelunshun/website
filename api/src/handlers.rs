use std::convert::Infallible;

use crate::{github, models, rejections};
use futures::future::join;
use rand::RngCore;
use semver::Version;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use warp::{hyper::StatusCode, Rejection};

use crate::models::{PluginListOptions, UserListOptions};

pub async fn health(pool: PgPool) -> Result<impl warp::Reply, Rejection> {
    match query("SELECT 42").fetch_one(&pool).await {
        Ok(_) => Ok(StatusCode::OK),
        _ => Ok(StatusCode::OK),
    }
}

pub async fn get_me(user_id: u32, pool: PgPool) -> Result<impl warp::Reply, Rejection> {
    let user = query!(
        r#"SELECT id, name, login, email, created_at FROM users WHERE id = $1;"#,
        user_id as i32
    )
    .fetch_one(&pool);

    let tokens = query!(
        r#"SELECT id, name, created_at, used_total FROM user_tokens WHERE user_id = $1;"#,
        user_id as i32
    )
    .fetch_all(&pool);

    let (user, tokens) = join(user, tokens).await;
    let user = user.map_err(|_| rejections::unauthorized())?;
    let tokens = tokens.map_err(|_| rejections::unauthorized())?;

    Ok(warp::reply::json(&models::Me {
        user: models::User {
            id: user.id as u32,
            name: user.name,
            login: user.login,
            created_at: user.created_at,
        },
        email: user.email,
        tokens: tokens
            .into_iter()
            .map(|token| models::Token {
                id: token.id as u32,
                name: token.name,
                created_at: token.created_at,
                used_total: token.used_total as u32,
                secret: None,
            })
            .collect(),
        created_at: user.created_at,
    }))
}

/// One time code returned by OAuth
#[derive(Deserialize)]
pub struct GithubCode {
    code: String,
}

pub async fn login_github(
    github_code: GithubCode,
    client_id: String,
    client_secret: String,
    pool: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let github_token = github::access_token(&client_id, &client_secret, &github_code.code)
        .await
        .unwrap();
    let github_user = github::user(&github_token).await.unwrap();

    // TODO: Track last login?
    query!(
        "INSERT INTO users (id, login, name) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        github_user.id,
        github_user.login,
        github_user.name
    )
    .execute(&pool)
    .await
    .map_err(|_| rejections::database())?;

    // TODO: this is not secure
    let mut secret = [0; 48];
    rand::rngs::OsRng.fill_bytes(&mut secret);

    // TODO: Combine with above query for optimization
    query!(
        "INSERT INTO user_tokens (secret, user_id) VALUES ($1, $2)",
        &secret[..],
        github_user.id
    )
    .execute(&pool)
    .await
    .map_err(|_| rejections::database())?;

    #[derive(Serialize)]
    struct AccessToken {
        access_token: String,
    }

    Ok(warp::reply::json(&AccessToken {
        access_token: hex::encode(secret),
    }))
}

pub async fn delete_user(_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn create_token(_token: String, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn delete_token(_token: String, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn update_email(_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn send_email_verification(_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn verify_email_code(
    _code: String,
    _pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn list_users(
    _opts: UserListOptions,
    _pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn get_user(_id: u32, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn plugin_list(
    _opts: PluginListOptions,
    _pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn get_plugin(
    _id: u32,
    _version: Option<Version>,
    _pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn plugin_invite_owner(
    _plugin: String,
    _user: u32,
    _pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}
