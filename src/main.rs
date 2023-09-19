extern crate dotenv;

use std::{env, sync::Arc};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod api;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port: u16 = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse()
        .expect("Port must be number");
    let conn = env::var("DB_CONNECTION").expect("DB_CONNECTION must be set");
    let (client, connection) = tokio_postgres::connect(conn.as_str(), NoTls).await.unwrap();
    let client = Arc::new(client);

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    HttpServer::new(move || {
        // let default_size = env::var("DEFAULT_REQUEST_SIZE")
        //     .unwrap_or_else(|_| "2097152".to_string())
        //     .parse::<usize>()
        //     .unwrap_or(2097152);
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(api::init)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
