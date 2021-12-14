use serde::de::DeserializeOwned;
use thiserror::Error;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
  #[error("Encounter reqwest::Error: {0}")]
  Reqwest(#[from] reqwest::Error),
}

pub struct Client {
  rq_client: reqwest::Client,
  pub username: String,
  token: String,
}

impl Client {
  pub fn new(username: String, token: String) -> Self {
    Self {
      rq_client: reqwest::Client::new(),
      username,
      token,
    }
  }

  pub async fn get<T>(&self, path: String) -> ClientResult<T>
  where
    T: DeserializeOwned,
  {
    let username = &self.username;
    let token = &self.token;
    let url = format!("https://api.github.com/{}", path);

    let res = self
      .rq_client
      .get(&url)
      .basic_auth(username, Some(token))
      .header("User-Agent", "Cetus-Rust")
      .send()
      .await?
      .json::<T>()
      .await?;

    Ok(res)
  }
}
