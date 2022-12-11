mod config;
mod models;

use crate::models::Status;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::io;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "ok".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("Hello, world!");
    println!(
        "Starting server atr http://{}:{}/",
        config.server.host, config.server.port
    );
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
