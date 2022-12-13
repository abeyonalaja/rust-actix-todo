mod config;
mod db;
mod handlers;
mod models;

use crate::handlers::*;
use crate::models::Status;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

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
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todos", web::get().to(get_todo_list))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
