use actix_web::{App, HttpServer, web};
use std::io::Result;
use dotenv::dotenv;
use std::env;

mod schema;
mod database;
mod routes;
mod dao;

pub struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let pool = database::init().await
        .expect("Failed to connect to database");
    let host = env::var("HOST")
        .expect("HOST not set in .env");
    let port = env::var("PORT")
        .expect("PORT not set in .env")
        .parse::<u16>()
        .expect("Invalid PORT in .env");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {pool: pool.clone()}))
            .configure(routes::config)
    })
    .bind((host, port))?
    .run()
    .await
}