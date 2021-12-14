use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::{Client, ClientResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
  pub id: i32,
  pub name: String,
  pub full_name: String,

  #[serde(rename = "private")]
  pub is_private: bool,
  pub owner: RepositoryOwner,
  pub html_url: String,
  pub description: Option<String>,

  // api url
  pub url: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub stargazers_count: i32,
  pub watchers_count: i32,
  pub forks_count: i32,
  pub language: Option<String>,
  pub visibility: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryOwner {
  pub id: i32,
  pub login: String,
  pub avatar_url: String,
}

impl Client {
  pub async fn get_repositories(&self) -> ClientResult<Vec<Repository>> {
    let path = format!("users/{}/repos", self.username);

    self.get(path).await
  }
}
