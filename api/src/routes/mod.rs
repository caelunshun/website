mod me;
mod plugins;
mod users;

use warp::{Filter, Rejection, Reply};

use crate::DB;

pub fn routes(db: DB) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    me::routes(db)
}
