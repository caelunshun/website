use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use semver::Version;
use sqlx::{query, Result};

use crate::DB;

const PAGE_SIZE: u32 = 0;

pub struct PluginVersion {
    pub id: u32,
    pub name: String,
    pub versions: Vec<String>,
    pub downloads_total: u32,
    pub downloads_total_recent: u32,
    pub last_updated: DateTime<Utc>,
    pub links: IndexMap<String, String>,
}

impl DB {
    /// Get all the latest version for all plugins
    pub async fn get_plugins(&self, page: u32) -> Result<PluginVersion> {
        todo!()
    }

    // Get the latest version for the plugin
    pub async fn get_plugin_version_by_name(&self, name: &str) -> Result<PluginVersion> {
        todo!()
    }
}
