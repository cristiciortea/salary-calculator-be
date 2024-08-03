use axum::response::IntoResponse;
use axum::Router;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use database::db::setup_db;
use database::db_backup::get_current_year;
use routes::calculations::calculate_router;
use routes::health::health_router;

mod database;
mod routes;
mod models;
mod services;
mod validators;
mod utils;

static SERVER_ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[INFO]: Current year is {}...", get_current_year());
    println!("[INFO]: Set up the database...");
    let conn =
        Connection::open("./tax_rates.db").expect("Sqlite conn should be able to open. Cause");
    setup_db(&conn).expect("Setup db should work. Cause");

    println!("[INFO]: Create routers...");
    let main_router = Router::new()
        .merge(health_router())
        .merge(calculate_router());

    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();
    println!("----> LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, main_router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
