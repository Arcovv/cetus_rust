use actix_web::{get, web, HttpResponse};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::profile::LoginInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
  id: i32,
  name: String,
  full_name: String,

  #[serde(rename = "private")]
  is_private: bool,
  owner: RepositoryOwner,
  html_url: String,
  description: Option<String>,

  // api url
  url: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  stargazers_count: i32,
  watchers_count: i32,
  forks_count: i32,
  language: Option<String>,
  visibility: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryOwner {
  id: i32,
  login: String,
  avatar_url: String,
}

#[get("api/repos")]
pub async fn get_repositories(body: web::Json<LoginInfo>) -> HttpResponse {
  let LoginInfo { username, token } = body.into_inner();

  let repositories = Client::new()
    .get(&format!("https://api.github.com/users/{}/repos", username))
    .basic_auth(username, Some(token))
    .header("User-Agent", "Cetus-Rust")
    .send()
    .await
    .unwrap()
    .json::<Vec<Repository>>()
    .await
    .unwrap();

  debug!("repositories: {:?}", repositories);

  HttpResponse::Ok().json(repositories)
}
