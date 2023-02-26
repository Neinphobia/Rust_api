use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/get")]
async fn index() -> impl Responder {
    let message = json!({ "message": "Hello Rust api" });
    HttpResponse::Ok().json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
