use crate::{docs::DocsParser, featherurl::FeatherUrl};
use indicatif::{ProgressBar, ProgressStyle};
use std::{collections::HashMap, mem::drop, sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;
use warp::{http, Rejection};

pub type Documents = Arc<Mutex<HashMap<String, crate::DocResponse>>>;
pub type StringList = Arc<Mutex<Vec<String>>>;

const PREPATH_LENGTH: usize =
    "https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/".len();

pub async fn create_docs(container: Documents) {
    let mut paths: Vec<String> = Vec::new();
    let mut locked_container = container.lock().await;
    locked_container.clear();

    let (summary, mut paths_2, topics) = fetch_and_parse(
        "https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/SUMMARY.md",
    )
    .await
    .expect("FFS!");
    locked_container.insert(
        "summary".to_owned(),
        crate::DocResponse {
            html: summary,
            topics,
        },
    );

    let mut cur_stage: i32 = 1;
    paths.append(&mut paths_2);
    loop {
        paths.retain(|i| !i.ends_with("/"));
        let mut pathindex: u64 = 1;
        let pathsize: usize = paths.len() + 1;
        if paths.is_empty() {
            break;
        }
        log::debug!(target: "api", "Links ({}): {}", cur_stage, paths.join(", "));
        let pb = ProgressBar::new(pathsize as u64);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "[{{elapsed_precise}}] Stage {} ({{pos}}/{{len}}): {{msg}}",
            cur_stage
        )));

        let mut temp_links: Vec<String> = Vec::new();

        for path in paths.iter() {
            thread::sleep(Duration::from_millis(100));
            pb.set_position(pathindex);
            let shortended_path = &path[PREPATH_LENGTH..].to_lowercase();
            pb.set_message(shortended_path.clone());
            if !locked_container.contains_key(shortended_path) {
                let mut path_a = path.clone();
                path_a.push_str(".md");
                let (cur, mut temp_2_links, cur_topics) = fetch_and_parse(&path_a)
                    .await
                    .expect(&format!("FFS: {}", path));
                locked_container.insert(
                    shortended_path.to_owned(),
                    crate::DocResponse {
                        html: cur,
                        topics: cur_topics,
                    },
                );
                temp_links.append(&mut temp_2_links);
            }
            pathindex += 1;
        }
        pb.finish();

        paths.clear();
        if !temp_links.is_empty() {
            for i in temp_links.iter() {
                paths.push(i.clone());
            }
        }
        cur_stage += 1;
    }
    drop(locked_container);

    log::info!("Finished building docs!");
}

async fn fetch_and_parse(url: &str) -> Result<(String, Vec<String>, Vec<crate::Topic>), Rejection> {
    let response = reqwest::get(url).await.map_err(|_| warp::reject())?;

    if response.status() != http::StatusCode::OK {
        return Err(warp::reject());
    }

    let page: String = response.text().await.map_err(|_| warp::reject())?;

    let res = DocsParser::static_parse_links(FeatherUrl::from(url), page);

    Ok(res)
}
