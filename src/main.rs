#[macro_use]
extern crate log;

mod profile;
mod repo;

use std::{env, io};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
  env_logger::init();

  HttpServer::new(move || App::new().service(profile::get_profile).service(repo::get_repositories))
    .workers(4)
    .bind("127.0.0.1:8555")
    .unwrap()
    .run()
    .await
}
