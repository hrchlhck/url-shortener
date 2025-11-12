use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};

use crate::{
    api::{entry::Entry, message::Message},
    db::key_exists,
    ping::check_server,
};

use r2d2::{Pool, PooledConnection};
use redis::{Client, TypedCommands};
use std::collections::HashMap;

use log::{debug, info};
use std::sync::Arc;

type AppState = State<Arc<Pool<Client>>>;
type ApiRet = (StatusCode, Json<Message>);

#[derive(serde::Deserialize)]
pub struct ErrorQuery {
    pub message: String,
}

pub enum RedirectResponse {
    Redirect(Redirect),
    Error((StatusCode, Json<Message>)),
}

impl IntoResponse for RedirectResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            RedirectResponse::Redirect(r) => r.into_response(),
            RedirectResponse::Error(e) => e.into_response(),
        }
    }
}

pub async fn index() -> ApiRet {
    (
        StatusCode::OK,
        Json::from(Message::new("v0.1.0".to_string())),
    )
}

pub async fn new(State(redis_pool): AppState, Json(payload): Json<Entry>) -> ApiRet {
    let mut conn = redis_pool.get().unwrap();

    debug!(
        "Retrieved connection from Redis ({:?})",
        conn.client_id().unwrap()
    );

    let k = format!("{}", payload.short_url());
    if key_exists(&mut conn, &k) {
        return (
            StatusCode::BAD_REQUEST,
            Json::from(Message::new(format!(
                "{} already exists",
                payload.short_url()
            ))),
        );
    }

    let exp = payload.get_expiration();
    if exp == 0 {
        debug!("Expiration not set for '{}'", k);
        conn.set(k, payload.long_url()).unwrap();
    } else {
        debug!("Expiration of {}s was set for '{}'", exp, k);
        conn.set_ex(k, payload.long_url(), exp).unwrap();
    }

    info!(
        "Created entry {} ({})",
        payload.short_url(),
        payload.long_url()
    );

    (
        StatusCode::CREATED,
        Json::from(Message::new(format!("{} created", payload.short_url()))),
    )
}

pub async fn redirect(
    Path(short_url): Path<String>,
    State(redis_pool): AppState,
) -> RedirectResponse {
    let mut conn = redis_pool.get().unwrap();

    if !key_exists(&mut conn, &short_url) {
        let msg = format!("{short_url} does not exist");

        info!(
            "Short URL '{}' not found. Redirecting to /error.",
            short_url
        );
        return RedirectResponse::Redirect(Redirect::to(&format!("/error?message={}", msg)));
    }

    info!("Redirecting to {short_url}");
    let val = conn.get(short_url).unwrap().unwrap();
    let uri = format!("https://{val}");

    if !check_server(uri.clone()).await {
        let msg = format!("{val} does not exist");

        info!("Long URL '{}' not found. Redirecting to /error.", val);
        return RedirectResponse::Redirect(Redirect::to(&format!("/error?message={}", msg)));
    }

    RedirectResponse::Redirect(Redirect::to(&uri))
}

pub async fn error_handler(Query(query): Query<ErrorQuery>) -> impl IntoResponse {
    info!("{}", query.message);
    (
        StatusCode::NOT_FOUND,
        Json::from(Message::new(query.message)),
    )
}

pub async fn list_shortcuts(
    State(redis_pool): AppState,
) -> (StatusCode, Json<HashMap<String, String>>) {
    let get_val: fn(&mut PooledConnection<Client>, &String) -> String =
        |conn: &mut PooledConnection<Client>, key: &String| {
            let v = conn.get(key).unwrap().unwrap();
            debug!("{key}: {v}");
            v
        };

    let mut conn = redis_pool.get().unwrap();

    let msg: HashMap<String, String> = conn
        .keys(r#"*"#)
        .unwrap()
        .iter()
        .map(|x| (x.clone(), get_val(&mut conn, x)))
        .collect();

    (StatusCode::OK, Json::from(msg))
}
