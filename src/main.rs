use actix_web::*;
use serde::Deserialize;
use whl::config::AppConfig;

#[derive(Deserialize, Debug)]
struct Repository {
  id:        i32,
  full_name: String,
}
#[derive(Deserialize, Debug)]
struct PushBody {
  repository: Repository,
}

#[post("/push")]
fn index(body: web::Json<PushBody>, config: web::Data<AppConfig>) -> HttpResponse {
  println!("{:?}", config);
  HttpResponse::Ok().json(format!("Hello {}!", body.repository.full_name))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let config = web::Data::new(AppConfig::load().expect("Failed to load configuration"));

  let interface = "127.0.0.1:8080";
  let app = move || App::new().app_data(config.clone()).service(index);

  println!("Listening on interface: {}", interface);
  HttpServer::new(app).bind(interface)?.run().await?;
  Ok(())
}
