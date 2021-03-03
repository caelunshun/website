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
        let plugin_version = query!("
            SELECT
                plugin_versions.version as version_lts,
                plugin_versions.description,
                plugin_versions.downloads,
                plugin_versions.stars,
                plugin_versions.created_at,
                plugins.downloads as downloads_total,
                plugins.stars as stars_total
            FROM 
                plugin_versions,
                plugins
            WHERE 
                plugin_versions.plugin_id = plugins.id AND
                plugins.id IN (SELECT id FROM plugins ORDER BY downloads DESC LIMIT 10) AND
                plugin_versions.version = (SELECT MAX(plugin_versions.version) FROM plugin_versions WHERE plugin_versions.plugin_id = plugins.id)
            ;
        ").fetch_all(self.as_ref());

        let plugin_versions = query!("
            SELECT
                plugin_id,
                version
            FROM
                plugin_versions
            WHERE
                plugin_id IN (SELECT id FROM plugins ORDER BY downloads DESC LIMIT 10)
            ;
        ");

        let owners = query!("
            SELECT 
                plugin_id, 
                user_id
            FROM 
                users, 
                plugin_owners
            WHERE 
                users.id = plugin_owners.user_id AND
                plugin_id IN (SELECT id FROM plugins ORDER BY downloads DESC LIMIT 10)
            ;
        ");

        let authors = query!("
            SELECT 
                plugin_version_id, 
                name 
            FROM 
                plugin_version_authors 
            WHERE 
                plugin_version_id IN (SELECT id FROM plugins ORDER BY downloads DESC LIMIT 10)
            ;
        ");
        todo!()
    }

    // Get the latest version for the plugin
    pub async fn get_plugin_version_by_name(&self, name: &str) -> Result<PluginVersion> {
        todo!()
    }
}
