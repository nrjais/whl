use actix_web::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct PushBody {
  name: String,
}

#[post("/push")]
fn index(body: web::Json<PushBody>) -> HttpResponse {
  HttpResponse::Ok().json(format!("Hello {}!", body.name))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let interface = "127.0.0.1:8080";
  let app = move || App::new().service(index);

  HttpServer::new(app).bind(interface)?.run().await?;
  println!("Listening on interface: {}", interface);
  Ok(())
}
