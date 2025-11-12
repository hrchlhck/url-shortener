use axum::{
    extract::Request,
    routing::{get, post},
    Router,
};
use tracing::Span;

mod api;
mod config;
mod db;
mod logging;
mod ping;

use crate::api::endpoints::{error_handler, index, list_shortcuts, new, redirect};
use crate::db::get_redis_connection_pool;
use tower_http::trace::TraceLayer;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    logging::init();
    
    let args = config::get_config();
    let redis_pool = get_redis_connection_pool(args.redis_address.unwrap());
    let state = Arc::new(redis_pool);

    let app = Router::new()
        .route("/", get(index))
        .route("/{short_url}", get(redirect))
        .route("/error", get(error_handler))
        .route("/list", get(list_shortcuts))
        .with_state(state.clone())
        .route("/new", post(new))
        .with_state(state.clone())
        .layer(
            TraceLayer::new_for_http().on_request(|req: &Request, _span: &Span| {
                let ip = req
                    .headers()
                    .get("host")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");

                log::info!(
                    "{host} {method} {uri} {user_agent}",
                    uri = req.uri().path(),
                    host = ip,
                    method = req.method(),
                    user_agent = req
                        .headers()
                        .get("user-agent")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("Not defined")
                );
            }),
        );

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        args.api_address.unwrap(),
        args.api_port.unwrap()
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
