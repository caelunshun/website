use std::convert::Infallible;

use semver::Version;
use sqlx::PgPool;

use crate::models::{PluginListOptions, UserListOptions};

pub async fn get_me(_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn login_github(_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
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

pub async fn verify_email_code(_code: String, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn list_users(_opts: UserListOptions, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn get_user(_id: u32, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn plugin_list(_opts: PluginListOptions, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}

pub async fn get_plugin(_id: u32, _version: Option<Version>, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}


pub async fn plugin_invite_owner(_plugin: String, _user: u32, _pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&()))
}