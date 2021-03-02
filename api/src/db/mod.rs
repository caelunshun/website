mod plugins;
mod user_tokens;
mod users;

pub use plugins::*;
pub use user_tokens::*;
pub use users::*;

use std::ops::Deref;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct DB(PgPool);

impl DB {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl Deref for DB {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<PgPool> for DB {
    fn as_ref(&self) -> &PgPool {
        self
    }
}
