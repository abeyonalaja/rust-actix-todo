mod config;
mod models;

use crate::models::Status;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "ok".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    println!("Hello, world!");
    println!(
        "Starting server atr http://{}:{}/",
        config.server.host, config.server.port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
