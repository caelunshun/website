use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use sqlx::query;
use warp::{Filter, Rejection, Reply};

use crate::{
    rejections::{self, IntoRejection},
    with_state, DB,
};

mod invite;
mod publish;
mod version;

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("plugins").and(filter_plugins(db.clone()))
}

pub fn filter_plugins(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::query())
        .and(warp::path::end())
        .and(with_state(db))
        .and_then(handle_plguins)
}

#[derive(Debug, Deserialize)]
pub struct PluginsOptions {
    #[serde(default)]
    pub search: String,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub page: u32,
}

pub async fn handle_plguins(options: PluginsOptions, db: DB) -> Result<impl Reply, Rejection> {
    #[derive(Serialize)]
    struct Links {
        #[serde(skip_serializing_if = "String::is_empty")]
        homepage: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        repository: String,
    }

    #[derive(Serialize)]
    struct Plugin {
        name: String,
        summary: String,
        versions: Vec<Version>,
        downloads_total: u32,
        downloads_total_recent: u32,
        last_updated: DateTime<Utc>,
        links: Links,
    }

    let db = db
        .get_plugins(options.page)
        .await
        .map_err(rejections::Database::reject)?;

    let plugins: Vec<Plugin> = vec![];

    Ok(warp::reply::json(&plugins))
}
