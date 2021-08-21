use actix_web::{Error, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

pub const WILDCARD_QUERY: &str = "*";

#[derive(Serialize)]
pub struct SearchResult<TItem> {
    count: usize,
    items: Vec<TItem>,
}

#[derive(Serialize, Deserialize)]
pub struct Airport {
    pub icao: String,
    pub iata: Option<String>,
    pub name: String,
    pub city: String,
    pub country: String,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    /// Max items to respond to the search query with
    pub limit: Option<usize>,

    /// Search terms for the query
    pub query: Option<String>,
}

impl<TItem> SearchResult<TItem> {
    pub fn from(items: Vec<TItem>, total_count: Option<usize>) -> SearchResult<TItem> {
        SearchResult {
            count: total_count.unwrap_or(items.len()),
            items,
        }
    }
}

impl Default for SearchRequest {
    fn default() -> Self {
        SearchRequest {
            limit: Some(50),
            query: Some(String::from(WILDCARD_QUERY)),
        }
    }
}

impl Responder for SearchResult<Airport> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
        let json_body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json_body)))
    }
}
