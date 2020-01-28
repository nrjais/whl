use actix_web::*;
use std::env::var;
use whl::config::AppConfig;
use whl::executor::Executor;
use whl::github_push;
use whl::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let config = web::Data::new(AppState::new(
    AppConfig::load().expect("Failed to load configuration"),
    Executor::new(),
  ));

  let app = move || App::new().app_data(config.clone()).service(github_push);

  let interface = format!("0.0.0.0:{}", var("POST").unwrap_or("8080".into()));
  println!("Listening on interface: {}", interface);
  HttpServer::new(app).bind(interface)?.run().await?;
  Ok(())
}
