use pulldown_cmark::Parser;
use reqwest::Client;
use serde::Serialize;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use warp::{http, path::Tail, Filter, Rejection, Reply};

use crate::{
    docs::{DocsParser, Summary, SummaryParser},
    with_state,
};

#[derive(Default, Serialize)]
pub struct Docs {
    summary: Option<String>,
    pages: HashMap<String, String>,
}

type Cache = Arc<RwLock<Docs>>;

// pub fn cache(
//     cache: Option<impl Reply>,
//     route: impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone,
// ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
//     if let Some(reply) = cache {
//         Ok(reply)
//     } else {
//         route
//     }
// }

#[derive(Serialize)]
pub struct SummaryResponse {
    html: String,
    summary: Summary,
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cache: Cache = Default::default();
    warp::path("docs").and(filter_summary(cache.clone()).or(filter_page(cache)))
}

pub fn filter_summary(
    cache: Cache,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("summary"))
        .and(warp::path::end())
        .and(with_state(cache))
        .and_then(handle_summary)
}

pub async fn handle_summary(cache: Cache) -> Result<impl Reply, Rejection> {
    //     if let Some(summary) = cache.read().await.summary {
    //         Ok(warp::reply::json(&summary)
    //     } else {
    //         let summary = ();
    //         cache.write().await.summary = Some(summary);
    //         summary
    //     };

    let response =
        reqwest::get("https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/SUMMARY.md")
            .await
            .map_err(|_| warp::reject())?;

    let summary = response.text().await.map_err(|_| warp::reject())?;

    let html =
        DocsParser::new(&summary, url::Url::from_str("http://localhost:3000/docs").unwrap()).parse();

    let summary = SummaryParser::new(&summary)
        .parse()
        .map_err(|_| warp::reject())?;

    Ok(html)
}

pub fn filter_page(
    cache: Cache,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("page"))
        .and(warp::path::tail())
        .and(with_state(cache))
        .and_then(handle_page)
}

#[derive(Debug, Serialize)]
pub struct PageResponse {
    html: String,
}

pub async fn handle_page(tail: Tail, cache: Cache) -> Result<impl Reply, Rejection> {
    let response = reqwest::get(&format!(
        "https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/{}.md",
        tail.as_str()
    ))
    .await
    .map_err(|_| warp::reject())?;

    if response.status() != http::StatusCode::OK {
        return Err(warp::reject());
    }

    let page = response.text().await.map_err(|_| warp::reject())?;

    let parser = Parser::new(&page);
    let mut html = DocsParser::new(
        &page,
        url::Url::from_str(&format!("http://localhost:3000/docs/{}", tail.as_str())).unwrap(),
    ).parse();

    Ok(html)
}
