use actix_web::{post, web, HttpResponse};
use cetus_github::client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
  pub username: String,
  pub token: String,
}

#[post("api/login")]
pub async fn login(body: web::Json<LoginInfo>) -> HttpResponse {
  let LoginInfo { username, token } = body.into_inner();
  let client = client::Client::new(username, token);
  let profile = client.get_profile().await;

  if profile.is_ok() {
    HttpResponse::Ok().finish()
  } else {
    HttpResponse::Unauthorized().finish()
  }
}
