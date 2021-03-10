mod docs;
mod health;
mod me;
mod plugins;
mod users;

use warp::{Filter, Rejection, Reply};

use crate::DB;

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    health::routes(db.clone())
        .or(me::routes(db.clone()))
        .or(plugins::routes(db))
        .or(docs::routes())
}
