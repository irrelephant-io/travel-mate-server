extern crate log;
extern crate dotenv;

use actix_web::{middleware::Logger, get, post, web, App, HttpServer};
use models::{Airport, SearchRequest, SearchResult};
use dotenv::dotenv;
use std::env;

mod models;
mod search_airports;

#[post("/search")]
async fn search(req: web::Json<Option<SearchRequest>>) -> SearchResult<Airport> {
    let request = req.0.unwrap_or_default();
    let (total_count, items) = search_airports::search(request).await;

    SearchResult::from(items, Some(total_count))
}

#[get("/health")]
async fn health() -> &'static str {
    "Ok"
}

fn get_srv_address() -> String {
    format!(
        "{}:{}",
        env::var("SRV_HOST").unwrap(),
        env::var("SRV_PORT").unwrap()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(search)
                    .service(health)
            )
    })
    .bind(get_srv_address())?
    .run()
    .await
}
