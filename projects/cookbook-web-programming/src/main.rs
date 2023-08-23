use error_chain::error_chain;
use reqwest::StatusCode;
use select::{document::Document, predicate::Name};
use serde::Deserialize;
use std::collections::HashSet;
use url::{Position, Url};

#[tokio::main]
async fn main() {
    let _ = extract_all_links().await;
    let _ = check_webpage_for_broken_links().await;
    let _ = make_http_get_request().await;
    let _ = query_github_api().await;
}

error_chain! {
    foreign_links {
        HttpError(reqwest::Error);
        IoError(std::io::Error);
        UrlParseError(url::ParseError);
        JoinError(tokio::task::JoinError);
    }
}

async fn extract_all_links() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;
    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    return Ok(());
}

async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

async fn check_webpage_for_broken_links() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;
    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    let mut tasks = vec![];
    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap() {
                println!("  ✅ {} is OK", link);
            } else {
                println!("  ❌ {} is broken", link);
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    return Ok(());
}

async fn make_http_get_request() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

// This no longer works because the GitHub API requires an auth token.
async fn query_github_api() -> Result<()> {
    let req_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", req_url);
    let res = reqwest::get(&req_url).await?;
    let users: Vec<User> = res.json().await?;
    println!("{:?}", users);
    Ok(())
}
