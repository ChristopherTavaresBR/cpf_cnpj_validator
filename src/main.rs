mod models;
mod validation;
mod anonymization;

use warp::{Filter, Reply};
use std::convert::Infallible;
use models::{ValidateParams, BulkRequest, ValidationResponse, BulkResponse};
use validation::validate_document;
use serde_json::json;

async fn validate_handler(params: ValidateParams) -> Result<impl Reply, Infallible> {
    let show_ranges = params.show.as_ref().map(|s| {
        s.split(',')
            .filter_map(|range| {
                let parts: Vec<&str> = range.split('-').collect();
                if parts.len() == 2 {
                    Some((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
                } else {
                    None
                }
            })
            .collect()
    });

    let response = validate_document(
        &params.doc,
        show_ranges,
        params.mask,
        params.hash_type.as_deref(),
    ).unwrap_or_else(|| ValidationResponse {
        valid: false,
        r#type: "INVALID".to_string(),
        number: params.doc.clone(),
        formatted: String::new(),
        anonymized: String::new(),
        anonymized_key: None,
        custom_anonymized: None,
        region: None,
    });

    Ok(warp::reply::json(&response))
}

async fn bulk_handler(body: BulkRequest) -> Result<impl Reply, Infallible> {
    let offset = body.offset.unwrap_or(0);
    let limit = body.limit.unwrap_or(100).min(100);
    let total = body.documents.len();

    let results = body.documents
        .into_iter()
        .skip(offset)
        .take(limit)
        .filter_map(|doc| validate_document(&doc, None, None, body.hash_type.as_deref()))
        .collect::<Vec<_>>();

    Ok(warp::reply::json(&BulkResponse {
        valid: results.iter().all(|r| r.valid),
        results,
        pagination: models::Pagination {
            total,
            offset,
            limit,
            next_offset: if offset + limit < total { Some(offset + limit) } else { None },
        },
    }))
}




#[tokio::main]
async fn main() {
    // Rotas principais
    let validate_route = warp::path!("validate")
        .and(warp::get())
        .and(warp::query::<ValidateParams>())
        .and_then(validate_handler);

    let bulk_route = warp::path!("validate" / "bulk")
        .and(warp::post())
        .and(warp::body::content_length_limit(2 * 1024 * 1024))
        .and(warp::body::json())
        .and_then(bulk_handler);

    // Health Check
    let health_route = warp::path!("health")
        .and(warp::get())
        .map(|| warp::reply::json(&json!({"status": "ok"})));

    // Configuração do servidor
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3030);

    let host = std::env::var("HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Server running on http://{}:{}", host, port);

    // Combine todas as rotas
    let routes = validate_route
        .or(bulk_route)
        .or(health_route)
        .with(warp::log("api")); // Adiciona logging

    warp::serve(routes)
        .run(([0, 0, 0, 0], port)) // Sempre escuta em 0.0.0.0 para Docker
        .await;
}