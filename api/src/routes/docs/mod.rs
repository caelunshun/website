use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use warp::{path::Tail, Filter, Rejection, Reply};

use crate::{docs::Summary, docsbuilder::Documents, with_state};

#[derive(Default, Serialize)]
pub struct Docs {
    summary: Option<String>,
    pages: HashMap<String, String>,
}

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

pub fn routes(
    documents: Documents,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("docs").and(
        filter_summary(documents.clone())
            .or(filter_webhook(documents.clone()).or(filter_page(documents))),
    )
}

pub fn filter_summary(
    documents: Documents,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("summary"))
        .and(warp::path::end())
        .and(with_state(documents))
        .and_then(handle_summary)
}

pub async fn handle_summary(documents: Documents) -> Result<impl Reply, Rejection> {
    let map = documents.lock().await;

    let mut result: Result<String, Rejection> = Err(warp::reject());

    if let Some(stuff) = map.get("summary") {
        result = Ok(stuff.clone())
    }

    drop(map);

    result
}

pub fn filter_webhook(
    documents: Documents,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("webhook"))
        .and(warp::path(
            env::var("DOCS_WEBHOOK_SECRET").unwrap_or("foobar".to_owned()),
        ))
        .and(warp::path::end())
        .and(warp::header::exact("Content-Type", "application/json"))
        .and(warp::header::<String>("X-GitHub-Event"))
        .and(warp::body::bytes())
        .and(with_state(documents))
        .and_then(handle_webhook)
}

pub async fn handle_webhook(
    event: String,
    body: bytes::Bytes,
    documents: Documents,
) -> Result<impl Reply, Rejection> {
    match event.as_str() {
        "push" => {
            let push_json: crate::types::PushAction = parse_json(&body).unwrap();
            if push_json.r#ref == "refs/heads/Docs" {
                tokio::spawn(crate::docsbuilder::create_docs(documents));
            }
        }
        "ping" => {
            log::info!("Recieved a Ping from GitHub Webhook! => Set up correctly");
        }
        _ => {}
    }
    Ok(warp::reply())
}

fn parse_json<T>(bytes: &[u8]) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_slice(&bytes).map_err(|e| e)
}

pub fn filter_page(
    documents: Documents,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("page"))
        .and(warp::path::tail())
        /*.and(warp::query::query::<DocsPageQuery>())*/
        .and(with_state(documents))
        .and_then(handle_page)
}

#[derive(Debug, Serialize)]
pub struct PageResponse {
    html: String,
}

#[derive(Debug, Deserialize)]
pub struct DocsPageQuery {
    base_url: String,
}

pub async fn handle_page(tail: Tail, documents: Documents) -> Result<impl Reply, Rejection> {
    let map = documents.lock().await;

    let mut result: Result<String, Rejection> = Err(warp::reject());

    if let Some(stuff) = map.get(&tail.as_str().to_lowercase()) {
        result = Ok(stuff.clone())
    }

    drop(map);

    result
}
