use anyhow::Result;
use reqwest::{header, Client};
use serde::Deserialize;

use pulldown_cmark::{html, Options, Parser};

#[derive(Deserialize)]
pub struct User {
    pub login: String,
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

pub async fn access_token(client_id: &str, client_secret: &str, code: &str) -> Result<String> {
    let client = Client::new();

    let token = client
        .post("https://github.com/login/oauth/access_token")
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "feathermc.org")
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
        ])
        .send()
        .await?
        .json::<AccessToken>()
        .await?;

    Ok(token.access_token)
}

pub async fn user(token: &str) -> Result<User> {
    let client = Client::new();

    let user = client
        .get("https://api.github.com/user")
        .header(header::ACCEPT, "application/json")
        .header(header::AUTHORIZATION, format!("token {}", token))
        .header(header::USER_AGENT, "feathermc.org")
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(user)
}

pub async fn get_budget() -> Result<String> {
    let response =
        reqwest::get("https://raw.githubusercontent.com/feather-rs/association/master/budget.md")
            .await?
            .text()
            .await?;

    let parser = Parser::new_ext(&response[..], Options::empty());
    let mut out_html = String::new();
    html::push_html(&mut out_html, parser);

    Ok(out_html)
}

pub async fn get_articles_of_association() -> Result<String> {
    let response = reqwest::get("https://raw.githubusercontent.com/feather-rs/association/master/Articles%20of%20Association.md")
        .await?
        .text()
        .await?;

    let parser = Parser::new_ext(&response[..], Options::empty());
    let mut out_html = String::new();
    html::push_html(&mut out_html, parser);

    Ok(out_html)
}
