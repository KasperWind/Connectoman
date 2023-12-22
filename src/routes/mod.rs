mod error;

use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router, http::HeaderMap,
};
use tera::{Context, Tera};
use tracing::info;


#[derive(Debug, Clone)]
pub struct AppState {
    pub tera: Arc<Mutex<Tera>>,
}

pub fn routes(s: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/contacts", get(contacts))
        .with_state(s)
}

async fn index() -> impl IntoResponse {
    Redirect::permanent("/contacts")
}

async fn contacts(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> error::Result<Html<String>> {
    info!("{:<12} - headers: {headers:?}", "CONTACTS");
    let mut context = Context::new();

    context.insert("page", &0);
    let name = "";
    context.insert("name", &name);
    let t = state.tera.lock().unwrap();
    let t = t.render("layout.html", &context)?;
    Ok(Html(t))
}
