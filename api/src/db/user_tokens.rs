use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, Result};

use crate::DB;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub id: u32,
    pub name: Option<String>,
    pub secret: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub used_total: u32,
}

impl DB {
    pub async fn get_user_token_by_user_id_and_name(
        &self,
        user_id: u32,
        token_name: Option<&str>,
    ) -> Result<UserToken> {
        let token = query!(
            "
            SELECT 
                id, 
                name, 
                secret, 
                created_at, 
                used_total 
            FROM 
                user_tokens
            WHERE 
                user_id = $1 AND name = $2",
            user_id as i32,
            token_name
        )
        .fetch_one(self.as_ref())
        .await?;
        Ok(UserToken {
            id: token.id as u32,
            name: token.name,
            secret: token.secret,
            created_at: token.created_at,
            used_total: token.used_total as u32,
        })
    }

    pub async fn get_user_tokens_by_user_id(&self, user_id: u32) -> Result<Vec<UserToken>> {
        let tokens = query!(
            "
            SELECT 
                id, 
                name, 
                secret, 
                created_at, 
                used_total 
            FROM 
                user_tokens
            WHERE 
                user_id = $1",
            user_id as i32
        )
        .fetch_all(self.as_ref())
        .await?;

        Ok(tokens
            .into_iter()
            .map(|token| UserToken {
                id: token.id as u32,
                name: token.name,
                secret: token.secret,
                created_at: token.created_at,
                used_total: token.used_total as u32,
            })
            .collect())
    }

    pub async fn get_user_id_by_auth_token_secret(&self, secret: &[u8]) -> Result<u32> {
        let user_token = query!(
            r#"
                SELECT user_id FROM user_tokens WHERE name IS NULL AND secret = $1
            "#,
            secret
        )
        .fetch_one(self.as_ref())
        .await?;
        Ok(user_token.user_id as u32)
    }

    pub async fn insert_user_token_by_user_id_and_name(
        &self,
        user_id: u32,
        name: Option<String>,
        secret: &[u8],
    ) -> Result<u32> {
        let user_token = query!(
            "INSERT INTO user_tokens (user_id, name, secret) VALUES ($1, $2, $3) RETURNING id",
            user_id as i32,
            name,
            secret
        )
        .fetch_one(self.as_ref())
        .await?;

        Ok(user_token.id as u32)
    }
}
