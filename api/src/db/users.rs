use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, Result};

use crate::DB;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl DB {
    pub async fn get_user_by_id(&self, id: u32) -> Result<User> {
        let user = query!(
            "
            SELECT 
                id,
                login, 
                name, 
                email,
                created_at
            FROM 
                users
            WHERE 
                id = $1",
            id as i32
        )
        .fetch_one(self.as_ref())
        .await?;
        Ok(User {
            id: user.id as u32,
            login: user.login,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        })
    }

    pub async fn insert_or_update_user(&self, user: User) -> Result<()> {
        query!(
            "
            INSERT INTO users (
                id,
                login, 
                name, 
                email
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            ) ON CONFLICT (
                name, 
                login, 
                email
            )
            DO UPDATE SET (
                login,
                name,
                email
            ) = (
                $2,
                $3,
                $4
            )",
            user.id as i32,
            user.login,
            user.name,
            user.email
        )
        .execute(self.as_ref())
        .await?;
        Ok(())
    }
}
