use actix_web::{post, web, HttpResponse};
use cetus_github::client;

use super::login::LoginInfo;

#[post("api/repositories")]
pub async fn fetch_repositories(body: web::Json<LoginInfo>) -> HttpResponse {
  let LoginInfo { username, token } = body.into_inner();
  let client = client::Client::new(username, token);
  let profile = client.get_repositories().await;

  if profile.is_ok() {
    HttpResponse::Ok().finish()
  } else {
    HttpResponse::Unauthorized().finish()
  }
}
