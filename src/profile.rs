use actix_web::{get, web, HttpResponse};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
  username: String,
  token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubProfile {
  id: i32,
  avatar_url: String,
  name: String,
  bio: String,
  public_repos: i32,
  public_gists: i32,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

#[get("api/profile")]
pub async fn get_profile(body: web::Json<LoginInfo>) -> HttpResponse {
  let LoginInfo { username, token } = body.into_inner();

  let profile = Client::new()
    .get("https://api.github.com/users/Arcovv")
    .basic_auth(username, Some(token))
    .header("User-Agent", "Cetus-Rust")
    .send()
    .await
    .unwrap()
    .json::<GitHubProfile>()
    .await
    .unwrap();

  debug!("profile: {:?}", profile);

  HttpResponse::Ok().json(profile)
}
