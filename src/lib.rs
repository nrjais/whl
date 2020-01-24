pub mod config;
pub mod error;
pub mod executor;

use actix_web::*;
use config::AppConfig;
use executor::Executor;
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct AppState {
  config:   AppConfig,
  executor: Executor,
}

impl AppState {
  pub fn new(config: AppConfig, executor: Executor) -> Self {
    Self { config, executor }
  }
}

#[derive(Deserialize, Debug)]
struct Repository {
  id:        i32,
  full_name: String,
}

#[derive(Deserialize, Debug)]
pub struct PushBody {
  repository: Repository,
}

#[post("/github")]
pub fn github_push(body: web::Json<PushBody>, state: web::Data<AppState>) -> HttpResponse {
  let conf = state.config.repos.iter().find(|c| c.name == body.repository.full_name);
  if let Some(conf) = conf {
    state.executor.execute(&conf.cmd, &conf.dir).unwrap();
    HttpResponse::Ok().finish()
  } else {
    HttpResponse::NotFound().json(serde_json::json!({
      "message": "No configuration found for the provided event"
    }))
  }
}
