use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::{Client, ClientResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
  pub id: i32,
  pub avatar_url: String,
  pub name: String,
  pub bio: String,
  pub public_repos: i32,
  pub public_gists: i32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Client {
  pub async fn get_profile(&self) -> ClientResult<Profile> {
    let path = format!("users/{}", self.username);

    self.get(path).await
  }
}
