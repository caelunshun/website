use crate::{docs::DocsParser, featherurl::FeatherUrl};
use indicatif::{ProgressBar, ProgressStyle};
use std::{collections::HashMap, mem::drop, sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;
use warp::{http, Rejection};

pub type Documents = Arc<Mutex<HashMap<String, String>>>;
pub type StringList = Arc<Mutex<Vec<String>>>;

const PREPATH_LENGTH: usize =
    "https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/".len();

pub async fn create_docs(container: Documents) {
    let paths: StringList = Default::default();
    let mut locked_container = container.lock().await;
    locked_container.clear();

    let summary = fetch_and_parse(
        paths.clone(),
        "https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/SUMMARY.md",
    )
    .await
    .expect("FFS!");
    locked_container.insert("summary".to_owned(), summary);

    let mut cur_stage: i32 = 1;

    let mut fin_paths = paths.lock().await;
    loop {
        fin_paths.retain(|i| !i.ends_with("/"));
        let mut pathindex: u64 = 1;
        let pathsize: usize = fin_paths.len() + 1;
        if fin_paths.is_empty() {
            break;
        }
        log::debug!(target: "api", "Links ({}): {}", cur_stage, fin_paths.join(", "));
        let pb = ProgressBar::new(pathsize as u64);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "[{{elapsed_precise}}] Stage {} ({{pos}}/{{len}}): {{msg}}",
            cur_stage
        )));

        let temp_arc: StringList = Default::default();

        for path in fin_paths.iter() {
            thread::sleep(Duration::from_millis(100));
            pb.set_position(pathindex);
            let shortended_path = &path[PREPATH_LENGTH..].to_lowercase();
            pb.set_message(shortended_path.clone());
            if !locked_container.contains_key(shortended_path) {
                let mut path_a = path.clone();
                path_a.push_str(".md");
                let cur = fetch_and_parse(temp_arc.clone(), &path_a)
                    .await
                    .expect(&format!("FFS: {}", path));
                locked_container.insert(shortended_path.to_owned(), cur);
            }
            pathindex += 1;
        }
        pb.finish();

        let res_temp_arc = temp_arc.lock().await;

        fin_paths.clear();
        if !res_temp_arc.is_empty() {
            for i in res_temp_arc.iter() {
                fin_paths.push(i.clone());
            }
        }

        drop(res_temp_arc);
        cur_stage += 1;
    }
    drop(fin_paths);
    drop(locked_container);

    log::info!("Finished building docs!");
}

async fn fetch_and_parse(links: StringList, url: &str) -> Result<String, Rejection> {
    let response = reqwest::get(url).await.map_err(|_| warp::reject())?;

    if response.status() != http::StatusCode::OK {
        return Err(warp::reject());
    }

    let page: String = response.text().await.map_err(|_| warp::reject())?;

    let parser = DocsParser::new(&page, FeatherUrl::from(url));
    let res = parser.parse_links(links).await;

    Ok(res)
}
