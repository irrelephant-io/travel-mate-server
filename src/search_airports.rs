use crate::models::{Airport, SearchRequest, WILDCARD_QUERY};
use async_fs as fs;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

fn match_score(airport: &Airport, matcher: &SkimMatcherV2, query: &Option<String>) -> i64 {
    if let Some(query) = query {
        if query == WILDCARD_QUERY {
            100
        } else {
            let name_match = matcher
                .fuzzy_match(&airport.name, query)
                .unwrap_or_default();
            let city_match = matcher
                .fuzzy_match(&airport.name, query)
                .unwrap_or_default();
            let icao_match: i64 = matcher
                .fuzzy_match(&airport.icao, query)
                .unwrap_or_default();
            let iata_match: i64 = if let Some(iata) = &airport.iata {
                matcher
                    .fuzzy_match(iata, query)
                    .unwrap_or_default()
            } else {
                0
            };
            
            name_match
                .max(city_match)
                .max(icao_match)
                .max(iata_match)
        }
    } else {
        100
    }
}

pub async fn search(req: SearchRequest) -> (usize, Vec<Airport>) {
    match fs::read_to_string("./resource/sampleData.json").await {
        Ok(buf) => {
            let matcher = SkimMatcherV2::default().ignore_case();
            let mut filtered = serde_json::from_str::<Vec<Airport>>(&buf)
                .expect("Sample data is not valid!")
                .into_iter()
                .map(|it|(match_score(&it, &matcher, &req.query), it))
                .filter(|(score, _)| *score > 80)
                .collect::<Vec<(i64, Airport)>>();
            filtered.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            let count = filtered.len();
            let limited = filtered
                .into_iter()
                .take(req.limit.unwrap_or(50))
                .map(|(_, airport)|airport)
                .collect();

            (count, limited)
        }
        Err(_) => {
            panic!("No sample data!");
        }
    }
}
