use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub user_id: u32,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub id: u32,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub used_total: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Me {
    #[serde(flatten)]
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub tokens: Vec<Token>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub login: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub downloads_total: u32,
    pub downloads_total_recent: u32,
    pub last_updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub links: IndexMap<String, String>,
    pub versions: Vec<Version>,
    pub owners: Vec<User>,
    pub dl: Vec<PluginTar>,
}

#[derive(Debug, Serialize)]
pub struct UserPlugins {
    #[serde(flatten)]
    pub user: User,
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize)]
pub struct PluginTar {
    pub arch: String,
    // Size in bytes
    pub size: u32,
}

#[derive(Debug, Deserialize)]
pub struct PluginListOptions {
    #[serde(default)]
    pub search: String,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub owners: Vec<String>,
    #[serde(default)]
    pub page: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserListOptions {
    #[serde(default)]
    pub search: String,
    #[serde(default)]
    pub page: u32,
}
