use reqwest_wasm::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Client, ClientBuilder,
};
use serde::{Deserialize, Serialize};

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDetail {
    pub login: String,
    pub id: u32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: Option<String>,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: Option<u32>,
    pub following: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
}

fn client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_str("application/vnd.github+json").unwrap(),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_str("2022-11-28").unwrap(),
    );
    headers.insert("User-Agent", HeaderValue::from_str(" ").unwrap());
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    client
}

#[wasm_bindgen]
pub async fn users() -> Result<JsValue, JsValue> {
    let res = client()
        .get("https://api.github.com/users")
        .send()
        .await
        .unwrap();
    let res = res.text().await.unwrap();
    let res: Vec<User> = serde_json::from_str(res.as_str()).unwrap();
    Ok(serde_wasm_bindgen::to_value(&res)?)
}

#[wasm_bindgen]
pub async fn user(login: &str) -> Result<JsValue, JsValue> {
    let res = client()
        .get(format!("https://api.github.com/users/{}", login))
        .send()
        .await
        .unwrap();
    let res = res.text().await.unwrap();
    let res: UserDetail = serde_json::from_str(res.as_str()).unwrap();
    Ok(serde_wasm_bindgen::to_value(&res)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn users_test() {
        assert!(users().await.is_ok());
    }
    #[tokio::test]
    async fn user_test() {
        assert!(user("octocat").await.is_ok());
    }
}
