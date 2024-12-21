use actix_web::{post, web::{self}, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;
mod requests;

#[post("/webhook")]
async fn webhook(payload: web::Json<Value>) -> impl Responder {
    let payload_inner = payload.into_inner();
    let commit: &Value = if let Some(value) = payload_inner.get("after") {
        println!("Commit {}", value);
        value
    } else {
        &serde_json::Value::Null
    };

    let original_repo: &Value = if let Some(repo) = payload_inner.get("repository") {
        if let Some(full_name) = repo.get("full_name") {
            full_name
        } else {
            &serde_json::Value::Null
        }
    } else {
        &serde_json::Value::Null
    };
    requests::line_changes(commit.to_string().as_str(),original_repo.to_string().as_str()).await;
    let response_data = serde_json::json!({ "commit": commit, "original_repo": original_repo });
    println!("{:?}",response_data);
    HttpResponse::Ok()
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