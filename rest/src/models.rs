use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GithubToken {
    access_token: String,
}

#[derive(Debug, Serialize)]
pub struct Email {
    address: String,
    verified: bool,
}

#[derive(Debug, Serialize)]
pub struct Token {
    id: u32,
    name: String,
    crated_at: DateTime<Utc>,
    used_total: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Me {
    #[serde(flatten)]
    user: User,
    email: Email,
    tokens: Vec<Token>,
}

#[derive(Debug, Serialize)]
pub struct User {
    id: u32,
    name: String,
    github_id: u32,
    github_name: String,
}

#[derive(Debug, Serialize)]
pub struct Plugin {
    id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    downloads_total: u32,
    downloads_total_recent: u32,
    last_updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    links: IndexMap<String, String>,
    versions: Vec<Version>,
    owners: Vec<User>,
    dl: Vec<PluginTar>,
}

#[derive(Debug, Serialize)]
pub struct UserPlugins {
    #[serde(flatten)]
    user: User,
    plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize)]
pub struct PluginTar {
    arch: String,
    // Size in bytes
    size: u32,
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
