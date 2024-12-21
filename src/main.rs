use actix_web::{post, web::{self}, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

#[post("/webhook")]
async fn webhook(payload: web::Json<Value>) -> impl Responder {
    let payload_inner = payload.into_inner();
    if let Some(after) = payload_inner.get("after"){
        println!("After {}",after);
        HttpResponse::Ok().json(after)
    }else{
        HttpResponse::Ok().body("Missing commit change info")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started");
    HttpServer::new(|| {
        App::new()
            .service(webhook)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}