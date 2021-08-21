use actix_web::{middleware::Logger, post, web, App, HttpServer};
use models::{Airport, SearchRequest, SearchResult};

mod models;
mod search_airports;

#[post("/search")]
async fn search(req: web::Json<Option<SearchRequest>>) -> SearchResult<Airport> {
    let request = req.0.unwrap_or_default();
    let (total_count, items) = search_airports::search(request).await;

    SearchResult::from(items, Some(total_count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
                    App::new()
                    .wrap(Logger::default())
                    .service(
                        web::scope("/api")
                            .service(search)
                    ))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
