mod association;
mod docs;
mod health;
mod me;
mod plugins;
mod users;

use warp::{Filter, Rejection, Reply};

use crate::{docs::Documents, DB};

pub fn routes(
    db: DB,
    documents: Documents,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    health::routes(db.clone())
        .or(me::routes(db.clone()))
        .or(plugins::routes(db))
        .or(docs::routes(documents))
        .or(association::routes())
}
