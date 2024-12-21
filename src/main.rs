use actix_web::{post, web::{self}, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;
use chrono::Local;
use log::info;
mod requests;

fn setup_logger() {
    let log_file = fern::log_file("output.log").expect("Failed to open log file");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%d/%m/%Y %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(log_file)
        .apply()
        .expect("Failed to apply logger configuration");
}

#[post("/webhook")]
async fn webhook(payload: web::Json<Value>) -> impl Responder {
    let payload_inner = payload.into_inner();
    let commit: &Value = if let Some(value) = payload_inner.get("after") {
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

    let commit_str = commit.as_str().unwrap_or("").trim_matches('"');
    let original_repo_str = original_repo.as_str().unwrap_or("").trim_matches('"');

    requests::line_changes(commit_str, original_repo_str).await;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
    info!("Started server on http://0.0.0.0:8080/");
    HttpServer::new(|| {
        App::new()
            .service(webhook)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}